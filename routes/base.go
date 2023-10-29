package routes

import (
	"fmt"
	"html/template"
	"net/http"
)

func InitializePageRoutes() {
	http.HandleFunc("/", LandingHandler)
	http.HandleFunc("/about", AboutHandler)
	http.HandleFunc("/blog", BlogHandler)
	http.HandleFunc("/landing/projects", ProjectsHandler)
	http.HandleFunc("/landing/musician", MusicHandler)
}

func RenderPage(w http.ResponseWriter, tmpl string) {
	templatePath := fmt.Sprintf("html/pages/%s.html", tmpl)
	indexTmpl := template.Must(template.ParseFiles("html/index.html", templatePath))

	err := indexTmpl.Execute(w, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func LandingHandler(w http.ResponseWriter, r *http.Request) {
	RenderPage(w, "landing/index")
}

func AboutHandler(w http.ResponseWriter, r *http.Request) {
	RenderPage(w, "about")
}

func BlogHandler(w http.ResponseWriter, r *http.Request) {
	RenderPage(w, "blog")
}

func MusicHandler(w http.ResponseWriter, r *http.Request) {
	tmpl, _ := template.ParseFiles("html/pages/landing/musician.html")
	tmpl.Execute(w, nil)
}

func ProjectsHandler(w http.ResponseWriter, r *http.Request) {
	tmpl, _ := template.ParseFiles("html/pages/landing/projects.html")
	tmpl.Execute(w, nil)
}
