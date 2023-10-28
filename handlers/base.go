package handlers

import (
	"fmt"
	"html/template"
	"net/http"
)

func InitializeRoutes() {
	http.HandleFunc("/", IndexHandler)
	http.HandleFunc("/about", AboutHandler)
	http.HandleFunc("/blog", BlogHandler)
	http.HandleFunc("/landing/projects", ProjectsHandler)
	http.HandleFunc("/hidden/projects", ProjectsView)
	http.HandleFunc("/landing/musician", MusicHandler)
}

func RenderTemplate(w http.ResponseWriter, tmpl string) {
	templatePath := fmt.Sprintf("html/pages/%s.html", tmpl)
	indexTmpl := template.Must(template.ParseFiles("html/index.html", templatePath))

	err := indexTmpl.Execute(w, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func IndexHandler(w http.ResponseWriter, r *http.Request) {
	RenderTemplate(w, "landing/index")
}

func AboutHandler(w http.ResponseWriter, r *http.Request) {
	RenderTemplate(w, "about")
}

func BlogHandler(w http.ResponseWriter, r *http.Request) {
	RenderTemplate(w, "blog")
}
