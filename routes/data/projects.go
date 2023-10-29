package data

type Project struct {
	Name        string
	Description string
	ImagePath   string
	Content     string
	Tags        []string
}

var Projects = []Project{
	{
		Name:        "Espionox",
		Description: "A Rust library for creating Ai driven Agent workflows",
		ImagePath:   "public/assets/espionox-gui-screenshot.png",
		Content:     "This is content",
		Tags:        []string{"Rustlang", "Applied Ai", "System Design"},
	},
}
