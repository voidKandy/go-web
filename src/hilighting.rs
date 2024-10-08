use anyhow::anyhow;
use tracing::warn;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Content {
    Highlight(String),
    Regular(String),
}

impl Content {
    pub fn into_html(self) -> anyhow::Result<String> {
        match self {
            Self::Regular(s) => Ok(markdown::to_html(&s)),
            Self::Highlight(s) => {
                let language = Self::parse_str_for_language_tag(&s)?;
                let mut string = {
                    if let Some(ref l) = language {
                        s[3 + l.len()..s.len() - 3].to_owned()
                    } else {
                        s[3..s.len() - 3].to_owned()
                    }
                };
                if !string
                    .lines()
                    .next()
                    .ok_or(anyhow!("should not pass empty string to this method"))?
                    .chars()
                    .any(|c| !c.is_whitespace())
                {
                    if let Some(first_newline) = string.find(|c| c == '\n') {
                        string = string[first_newline + 1..].to_owned();
                    }
                }
                let highlighted = HighlighterParser::highlight_text(&string, language.as_deref())?;
                Ok(highlighted)
            }
        }
    }

    fn parse_str_for_language_tag(text: &str) -> anyhow::Result<Option<String>> {
        let lines: Vec<&str> = text.lines().collect();
        if !lines[0].contains("```") {
            return Err(anyhow::anyhow!(
                "A text which should not be hilighted was passed to parse_str_for_language_tag",
            ));
        }
        if let Some(first_alph_char) = lines[0].chars().position(|c| c.is_alphabetic()) {
            return Ok(Some(lines[0][first_alph_char..].to_string()));
        }
        Ok(None)
    }
}

#[derive(Debug)]
pub struct HighlighterParser {
    current_char: char,
    next_char: Option<char>,
    text: String,
}

impl HighlighterParser {
    fn expect_char<F>(&self, f: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        if let Some(ch) = self.next_char {
            return f(ch);
        }
        false
    }

    fn peek_char(&self) -> Option<&char> {
        self.next_char.as_ref()
    }

    fn progress_char(&mut self) -> anyhow::Result<()> {
        match self.next_char {
            Some(ch) => self.current_char = ch,
            None => return Err(anyhow::anyhow!("next char is none")),
        }
        let mut chs = self.text.chars();
        self.next_char = chs.next();

        self.text = chs.collect();

        return Ok(());
    }

    #[tracing::instrument(name = "new highlighter parser")]
    pub fn new(text: &str) -> HighlighterParser {
        let mut chars = text.chars();
        let current_char = chars
            .next()
            .expect("empty strings should not be passed to this method");
        let next_char = chars.next();
        Self {
            current_char,
            next_char,
            text: text[2..].to_owned(),
        }
    }

    pub fn highlight_code_blocks(&mut self) -> anyhow::Result<Vec<Content>> {
        let mut return_vec = vec![];
        let mut current_buffer = String::new();

        while self.peek_char().is_some() {
            current_buffer.push(self.current_char);

            if current_buffer.len() >= 3 {
                let i = current_buffer.len() - 3;
                let last_three_chars: String = current_buffer.chars().rev().take(3).collect();
                if *last_three_chars == *"```" {
                    return_vec.push(Content::Regular(current_buffer.drain(..i).collect()));
                    if !self.text.contains("```") {
                        // break;
                        return Err(anyhow::anyhow!("unclosed code block"));
                    }
                    while self.expect_char(|c| c != '`') {
                        self.progress_char()?;
                        current_buffer.push(self.current_char);
                    }
                    for _ in 0..=2 {
                        self.expect_char(|c| c == '`');
                        self.progress_char()?;
                        current_buffer.push(self.current_char);
                    }
                    return_vec.push(Content::Highlight(current_buffer.drain(..).collect()));
                }
            }
            self.progress_char()?;
        }

        current_buffer.push(self.current_char);
        return_vec.push(Content::Regular(current_buffer));

        return Ok(return_vec);
    }

    fn match_language_with_extension(lang: &str) -> Option<&str> {
        match lang {
            "rust" => Some("rs"),
            "html" => Some("html"),
            "css" => Some("css"),
            "go" | "golang" => Some("go"),
            "javascript" => Some("js"),
            "typescript" => Some("ts"),
            "python" => Some("py"),
            "lua" => Some("lua"),
            other => Some(other),
        }
    }

    fn highlight_text(text: &str, lang: Option<&str>) -> anyhow::Result<String> {
        use syntect::highlighting::{Color, ThemeSet};
        use syntect::html::highlighted_html_for_string;
        use syntect::parsing::SyntaxSet;

        let ss = SyntaxSet::load_defaults_newlines();
        let sref = lang
            .and_then(|l| {
                Self::match_language_with_extension(l)
                    .and_then(|ext| ss.find_syntax_by_extension(ext))
            })
            .unwrap_or(ss.find_syntax_plain_text());

        let ts = ThemeSet::load_defaults();

        let theme = &ts.themes["base16-eighties.dark"];
        let c = theme.settings.background.unwrap_or(Color::WHITE);
        println!(
            "<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
            c.r, c.g, c.b
        );
        let html = highlighted_html_for_string(text, &ss, sref, theme)?;
        Ok(format!("{}", html))
    }
}

mod tests {
    use crate::{
        error::init_test_tracing,
        hilighting::{Content, HighlighterParser},
    };

    #[test]
    fn parse_for_language_tag_works() {
        init_test_tracing();
        let input = r#"```rust
                    println!("this is code");
                ```"#;

        let language = Content::parse_str_for_language_tag(&input).unwrap();
        assert_eq!(Some("rust"), language.as_deref());
    }

    #[test]
    fn drain() {
        let expected = "";
        let inp: String = "a".to_string().drain(1..).collect();

        assert_eq!(inp, expected.to_owned());

        let inp = "sadfasdfaaa".to_owned();
        let dr: String = inp[inp.len() - 3..].to_owned();
        assert_eq!(String::from("aaa"), dr);

        let mut inp = "aaa".to_owned();
        let dr: String = inp.drain(..inp.len() - 3).collect();
        assert_eq!(String::new(), dr);
    }

    #[test]
    fn parser_methods_behave_correctly() {
        init_test_tracing();

        let input = "abcde";
        let mut parser = HighlighterParser::new(input);

        assert_eq!(parser.current_char, 'a');
        assert_eq!(parser.peek_char(), Some(&'b'));
        parser.progress_char().unwrap();
        assert_eq!(parser.current_char, 'b');
        assert_eq!(parser.peek_char(), Some(&'c'));
        parser.progress_char().unwrap();
        assert_eq!(parser.current_char, 'c');
        assert_eq!(parser.peek_char(), Some(&'d'));
        parser.progress_char().unwrap();
        assert_eq!(parser.current_char, 'd');
        assert_eq!(parser.peek_char(), Some(&'e'));
        parser.progress_char().unwrap();
        assert_eq!(parser.current_char, 'e');
        assert_eq!(parser.peek_char(), None);
    }

    #[test]
    fn highlighting_works() {
        init_test_tracing();
        let input = r#"
                this is not code
                neither is this
                ```rust
                    println!("this is code");
                ```
                this is not code
                neither is this"#;

        let mut parser = HighlighterParser::new(input);
        let output = parser.highlight_code_blocks().unwrap();

        let expected = vec![
            Content::Regular(String::from(
                r#"
                this is not code
                neither is this
                "#,
            )),
            Content::Highlight(String::from(
                r#"```rust
                    println!("this is code");
                ```"#,
            )),
            Content::Regular(String::from(
                r#"
                this is not code
                neither is this"#,
            )),
        ];
        println!("{:?}", output);
        assert_eq!(output, expected);
    }
}
