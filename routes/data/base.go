package data

import (
	"html/template"
	"net/http"
)

func InitializeDataRoutes() {
	fs := http.FileServer(http.Dir("public"))
	http.Handle("/public/", http.StripPrefix("/public/", fs))

	http.HandleFunc("/templates/projects", RenderProjects)
}

func RenderProjects(w http.ResponseWriter, r *http.Request) {
	tmpl, _ := template.ParseFiles("html/templates/projects.html")
	projects := map[string][]Project{
		"Projects": Projects,
	}
	tmpl.Execute(w, projects)
}
