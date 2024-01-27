package views

import (
	"html/template"
	"net/http"
)

func RenderProject(w http.ResponseWriter, p Project) {
	tmpl, _ := template.ParseFiles("html/partials/projects.html")
	tmpl.Execute(w, p)
}
