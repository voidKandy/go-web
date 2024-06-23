use super::error::{UploadError, UploadResult};
use crate::database::model::{Attachment, UpdateBlogPost, UploadBlogPost};
use anyhow::anyhow;
use axum::extract::Multipart;
use std::fs;
use std::path::Path;
use std::{fs::File, io::Write, ops::Deref};
use tracing::{debug, error, info, warn};

const POSTS_DIRECTORY_PATH: &str = "/public/assets/posts";

#[derive(Debug, Default)]
struct MarkdownParseReturn<'c> {
    title: &'c str,
    subtitle: &'c str,
    category: Option<&'c str>,
    content: &'c str,
}

impl UpdateBlogPost {
    fn parse_markdown_content_for_info<'c>(text: &'c str) -> UploadResult<MarkdownParseReturn<'c>> {
        let mut ret = MarkdownParseReturn::default();
        let first_delimeter = text
            .lines()
            .position(|l| l.trim() == "---")
            .ok_or(anyhow!("markdown file does not have a '---'"))?;
        debug!("found first '---' on line: {}", first_delimeter);
        let next_delimeter = text
            .lines()
            .skip(first_delimeter + 1)
            .position(|l| l.trim() == "---")
            .ok_or(anyhow!("markdown file does not have a second '---'"))?;

        debug!("found next '---' on line: {}", first_delimeter);
        let info_section: Vec<&str> = text
            .lines()
            .skip(first_delimeter)
            .take(next_delimeter + 2)
            .collect();
        debug!("Info section: {:?}", info_section);
        let content_cutoff = info_section.join("\n").len() + 1;
        info_section
            .into_iter()
            .filter(|l| l.trim().split_once(":").is_some())
            .for_each(|l| match l.trim().split_once(":").unwrap() {
                ("title", title) => ret.title = title.trim(),
                ("subtitle", subtitle) => ret.subtitle = subtitle.trim(),
                ("category", category) => {
                    ret.category = if category.trim().is_empty() {
                        None
                    } else {
                        Some(category.trim())
                    }
                }
                other => warn!("no way to handle {} field", other.0),
            });

        if ret.title.is_empty() {
            return Err(anyhow!("No title in info section of markdown file").into());
        }
        if ret.subtitle.is_empty() {
            warn!("No subtitle in info section of markdown file")
        }
        ret.content = &text[content_cutoff..];
        Ok(ret)
    }
}

#[tracing::instrument(name = "save all attachments")]
pub(super) fn save_all_attachments_to_filesystem(mut vec: Vec<Attachment>) -> UploadResult<()> {
    let path_str = format!("./{}", &POSTS_DIRECTORY_PATH);

    let parent_path_str = path_str.rsplit_once('/').unwrap().0;
    let public_path_str = parent_path_str.rsplit_once('/').unwrap().0;

    let public_metadata = fs::metadata(public_path_str)?;
    let mut public_perms = public_metadata.permissions();
    public_perms.set_readonly(false);

    let parent_metadata = fs::metadata(parent_path_str)?;
    let mut parent_perms = parent_metadata.permissions();
    parent_perms.set_readonly(false);

    let path = Path::new(&path_str);
    if !path.exists() {
        fs::create_dir(path).map_err(|err| {
            error!(
                "there s an error when creating the posts assets directory: {:?}",
                err
            );
            anyhow!(
                "there was an error when creating the posts assets directory: {:?}",
                err
            )
        })?;
    }

    for attachment in vec.iter_mut() {
        let path_str = format!(".{}/{}", POSTS_DIRECTORY_PATH, attachment.filename);
        let path = Path::new(&path_str);
        match path.exists() {
            false => {
                warn!("file: {:?} does not exist, writing", path);
                let mut file = File::create_new(path).map_err(|err| anyhow!(err))?;
                file.write_all(&attachment.bytes)
                    .map_err(|err| anyhow!(err))?;
            }
            true => {
                warn!("file: {:?} already exists, overwriting", path);
                fs::write(path, &attachment.bytes).map_err(|err| {
                    error!(
                        "there was a problem overriting the file: {:?}\n: {:?}",
                        path, err
                    );
                    anyhow!(
                        "there was a problem overriting the file: {:?}\n: {:?}",
                        path,
                        err
                    )
                })?;
            } // }
        }
    }

    public_perms.set_readonly(true);
    parent_perms.set_readonly(true);
    Ok(())
}

pub(super) fn change_attachments_links(text: &str) -> UploadResult<String> {
    let mut modified_lines = Vec::new();
    for line in text.lines() {
        match line.find("![") {
            Some(idx) => {
                if line.contains("https://") || line.contains("http://") {
                    modified_lines.push(line.to_string());
                    continue;
                }
                match (line[idx..].find('('), line[idx..].find(')')) {
                    (Some(start), Some(end)) => {
                        let start = idx + start + 1; // position of '('
                        let end = idx + end; // position of ')'
                        let url = &line[start + 1..end]; // extract URL
                        let new_url = format!("{}{}", POSTS_DIRECTORY_PATH, url); // create new URL

                        let modified_line =
                            format!("{}{}{}", &line[..start + 1], new_url, &line[end..]); // construct modified line
                        modified_lines.push(modified_line);
                    }
                    _ => {
                        return Err(
                            anyhow!("could not find a closing or opening parentheses").into()
                        )
                    }
                }
            }
            None => modified_lines.push(line.to_string()),
        }
    }

    Ok(modified_lines.join("\n"))
}

pub(super) trait TryFromMultipart {
    async fn try_from_multipart(_: Multipart) -> UploadResult<Self>
    where
        Self: Sized;
}

impl TryFromMultipart for UploadBlogPost {
    async fn try_from_multipart(mut mp: Multipart) -> UploadResult<Self> {
        let mut post = UploadBlogPost::default();
        while let Some(field) = mp.next_field().await? {
            let name = field.name().ok_or(anyhow!("No name on field"))?.to_string();

            match name.as_str() {
                "files" => {
                    if let Some(filename) = field.file_name().and_then(|n| Some(n.to_string())) {
                        match filename.rsplit_once('.').and_then(|spl| Some(spl.1)) {
                            Some("md") => {
                                let md_content =
                                    std::str::from_utf8(field.bytes().await?.deref())?.to_owned();
                                let parsed_md_content =
                                    UpdateBlogPost::parse_markdown_content_for_info(&md_content)?;

                                post.content = parsed_md_content.content.to_owned();
                                post.title = parsed_md_content.title.to_owned();
                                post.subtitle = parsed_md_content.subtitle.to_owned();
                                post.category =
                                    parsed_md_content.category.and_then(|c| Some(c.to_owned()));
                            }
                            Some(_) => {
                                post.attachments
                                    .push(Attachment::new(&filename, field.bytes().await?.deref()));
                            }
                            None => error!("file without extension passed to form"),
                        }
                    } else {
                        error!("For some reason a file was passed without a filename");
                    }
                }
                f => return Err(anyhow!("encountered unexpected field: {}", f).into()),
            }
        }
        Ok(post)
    }
}

impl TryFromMultipart for UpdateBlogPost {
    async fn try_from_multipart(mut mp: Multipart) -> UploadResult<UpdateBlogPost> {
        let mut post = Self::default();
        let mut all_attachments = vec![];

        while let Some(field) = mp.next_field().await? {
            let name = field.name().ok_or(anyhow!("No name on field"))?.to_string();

            match name.as_str() {
                n @ "title" | n @ "subtitle" | n @ "category" => {
                    let bytes: &[u8] = &field.bytes().await?;
                    if bytes.is_empty() {
                        continue;
                    }
                    match n {
                        "title" => post.title = Some(std::str::from_utf8(bytes)?.to_owned()),
                        "subtitle" => post.subtitle = Some(std::str::from_utf8(bytes)?.to_owned()),
                        "category" => {
                            post.category = {
                                let str = std::str::from_utf8(bytes)?.to_owned();
                                if !str.is_empty() {
                                    Some(str)
                                } else {
                                    None
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                "files" => {
                    if let Some(filename) = field.file_name().and_then(|n| Some(n.to_string())) {
                        info!("attachment: {:?}", filename);
                        all_attachments.push(filename.to_string());

                        match filename.rsplit_once('.').and_then(|spl| Some(spl.1)) {
                            Some("md") => {
                                post.content = Some(
                                    std::str::from_utf8(field.bytes().await?.deref())?.to_owned(),
                                )
                            }
                            Some(_) => {
                                post.attachments
                                    .push(Attachment::new(&filename, field.bytes().await?.deref()));
                            }
                            None => error!("file without extension passed to form"),
                        }
                    } else {
                        error!("For some reason a file was passed without a filename");
                    }
                }
                f => return Err(anyhow!("encountered unexpected field: {}", f).into()),
            }
        }
        info!("Update built from multipart: {:?}", post);
        Ok(post)
    }
}

mod tests {
    use serde::de::Expected;

    use crate::{database::model::UpdateBlogPost, error::init_test_tracing};

    use super::change_attachments_links;

    #[test]
    fn correctly_changes_img_links() {
        let input = r#"this is text
        some more text
        ![img](./img.jpg)
        
        ![img](./img2.jpg)
        [img](./not-an-img)

        this is just me! talking (with parentheses)
        "#;

        let out = change_attachments_links(input).unwrap();
        let expected = r#"this is text
        some more text
        ![img](./public/assets/posts/img.jpg)
        
        ![img](./public/assets/posts/img2.jpg)
        [img](./not-an-img)

        this is just me! talking (with parentheses)
        "#;

        assert_eq!(out, expected);
    }

    #[test]
    fn correctly_parsed_md() {
        init_test_tracing();
        let input = r#"
---
title: Monkeylang interpreter progress report 
subtitle: Going over the basics of my interpreter 
category:  
---
Since switching gears from the Stanford compilers class to “Writing an interpreter in Go”, I’ve made a lot of progress writing an interpreter. Though I’m not completely through the book, I am very close. I have learned a lot about interpreters and programming languages more broadly as a result of this project.

## Demonstrating the interpreter"#;

        let ret = UpdateBlogPost::parse_markdown_content_for_info(input).unwrap();

        assert_eq!(
            ret.title,
            String::from("Monkeylang interpreter progress report")
        );
        assert_eq!(
            ret.subtitle,
            String::from("Going over the basics of my interpreter")
        );
        assert!(ret.category.is_none());
        assert_eq!(
            ret.content,
            String::from(
                r#"
Since switching gears from the Stanford compilers class to “Writing an interpreter in Go”, I’ve made a lot of progress writing an interpreter. Though I’m not completely through the book, I am very close. I have learned a lot about interpreters and programming languages more broadly as a result of this project.

## Demonstrating the interpreter"#
            )
        );
    }
}
