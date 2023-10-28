package handlers

import (
	"html/template"
	"net/http"
)

type Project struct {
	Name        string
	Description string
	ImagePath   string
	Tags        []string
}

func MusicHandler(w http.ResponseWriter, r *http.Request) {
	tmpl, _ := template.ParseFiles("html/pages/landing/musician.html")
	tmpl.Execute(w, nil)
}

func ProjectsHandler(w http.ResponseWriter, r *http.Request) {
	tmpl, _ := template.ParseFiles("html/pages/landing/projects.html")
	tmpl.Execute(w, nil)
}

func ProjectsView(w http.ResponseWriter, r *http.Request) {
	tmpl, _ := template.ParseFiles("html/templates/projects.html")
	projects := map[string][]Project{
		"Projects": {
			{
				Name:        "The Godfather",
				Description: "Francis Ford Coppola",
				ImagePath:   "path",
				Tags: []string{
					"this", "that",
				},
			},
		},
	}
	tmpl.Execute(w, projects)
}
