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
		Path:        "espionox",
		Name:        "Espionox",
		Description: "A Rust library for creating Ai driven Agent workflows",
		ImagePath:   "public/assets/espionox-gui-screenshot.png",
		Links: LinkInfo{
			HasRepo:    true,
			GitUrl:     "https://github.com/voidKandy/espionox_lib",
			HasWebDemo: true,
			DemoUrl:    "web-demo",
		},
		Tags: []string{"Rustlang", "Applied Ai", "System Design"},
	},
	{
		Path:        "espionox-demo",
		Name:        "Espionox Web Demo",
		Description: "A chat web app utilizing Espionox",
		ImagePath:   "public/assets/espionox-gui-screenshot.png",
		Links: LinkInfo{
			HasRepo:    false,
			GitUrl:     "",
			HasWebDemo: false,
			DemoUrl:    "",
		},
		Tags: []string{"Axum", "Htmx"},
	},
	{
		Path:        "showbeam",
		Name:        "Showbeam",
		Description: "A web app for sharing and creating markdown slide shows",
		ImagePath:   "public/assets/espionox-gui-screenshot.png",
		Links: LinkInfo{
			HasRepo:    false,
			GitUrl:     "",
			HasWebDemo: false,
			DemoUrl:    "",
		},
		Tags: []string{"Collaboration", "Actix-Web", "SvelteKit"},
	},
}
