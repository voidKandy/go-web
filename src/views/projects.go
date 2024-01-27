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
           Espionox is an library designed as a Rust alternative to Lang-Chain.
           Though still in its early stages of development, this library gives developers the ability to construct
           multi-agent environments. The key feature is the EnvListener trait, which provides an interface for creating dynamic protocols
           for implementing methods for model self consistancy, or increasing output quality through methods such as Tree-Of-Thought.
           Currently in active development are retrieval augmented guidance and tool use functionalities.
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
		Path: "bureau",
		Name: "Bureau",
		Description: `
            Bureau is essentially a chatGpt clone app that utilizes Espionox under the hood. It is a simple example of how to use
            Espionox to create an LLM application. I used HTMX with Rust's Askama templating language, there is no live webapp yet, but 
            Bureau is easy to clone and get up and running on localhost.
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
            providing users with an easy interface for creating markdown document slideshows. I built this with a friend of mine as an
            excersize. The backend was written using the Rust library, Actix-Web, and we used planetscale for our database. The frontend was 
            written using SvelteKit.
            
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
