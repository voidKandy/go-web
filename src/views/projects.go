package views

type LinkInfo struct {
	HasRepo    bool
	GitUrl     string
	HasWebDemo bool
	DemoUrl    string
}

type Project struct {
	Path        string
	Name        string
	Description string
	ImagePath   string
	Links       LinkInfo
	Tags        []string
}

var Projects = []Project{
	{
		Path: "espionox",
		Name: "Espionox",
		Description: `
            Espionox is a library designed as a Rust alternative to Lang-Chain. This library allows developers to create LLM driven software with multiple agents.
            Its flexible, event-driven API streamlines the process of implementing agent cross-talk,
            self-consistency, self-evaluation; as well as giving agents arbitrary capabilities such as RAG or tool use.
            Espionox is published on crates.io and can easily be installed using cargo.
        `,
		ImagePath: "",
		Links: LinkInfo{
			HasRepo:    true,
			GitUrl:     "https://github.com/voidKandy/Espionox",
			HasWebDemo: false,
			DemoUrl:    "",
		},
		Tags: []string{"Rustlang", "Applied Ai", "System Design"},
	},
	{
		Path: "coding_assistant",
		Name: "Coding Assistant",
		Description: `
           In an effort to make myself more familiar with NeoVim, and for the fun of it, I am building a coding assistant using Espionox in an LSP.
           As of now, it provides the user with CodeActions for chatting with the code base and general questions for the assistant agent from within NeoVim.
           My vision for this project is an open-source alternative to Microsoft CoPilot with more configuration options.
     `,
		ImagePath: "",
		Links: LinkInfo{
			HasRepo:    true,
			GitUrl:     "https://github.com/voidKandy/espx-copilot",
			HasWebDemo: false,
			DemoUrl:    "",
		},
		Tags: []string{"Rustlang", "Applied Ai", "System Design", "LSP", "NeoVim"},
	},
	{
		Path: "bureau",
		Name: "Bureau",
		Description: `
            Bureau was written as an example of how to use Espionox. I used HTMX with Rust's Askama templating language to create my chatGpt clone.
            I wanted to see what HTMX was like as an alternative to JavaScript when writing a reactive front end.
            I was pleasantly surprised by the developer experience. There is no live webapp, but feel free to clone Bureau and get it up and running on localhost.
            I will continue to update Bureau, Espionox makes it easy for me to personalize agents for whatever I want.
        `,
		ImagePath: "",
		Links: LinkInfo{
			HasRepo:    true,
			GitUrl:     "https://github.com/voidKandy/bureau",
			HasWebDemo: false,
			DemoUrl:    "",
		},
		Tags: []string{"Axum", "Htmx", "Espionox"},
	},
	{
		Path: "showbeam",
		Name: "Showbeam",
		Description: `
            Showbeam is a web application for creating and sharing markdown slideshows. It works much like google slides,
            providing users with an easy interface for creating markdown document slideshows. I built this with a friend of mine—a good exercise in collaboration.
            The backend was written using the Rust library, Actix-Web, we used planetscale for our database, SvelteKit for the front end, and we deployed it on Railway. 
            `,
		ImagePath: "",
		Links: LinkInfo{
			HasRepo:    false,
			GitUrl:     "",
			HasWebDemo: false,
			DemoUrl:    "",
		},
		Tags: []string{"Collaboration", "Actix-Web", "SvelteKit"},
	},
}
