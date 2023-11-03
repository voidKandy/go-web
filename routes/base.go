package routes

import (
	"fmt"
	"html/template"
	"net/http"
	"voidkandy-dot-dev/routes/data"

	"github.com/gorilla/mux"
)

func InitializePageRoutes() {
	r := mux.NewRouter()
	r.HandleFunc("/", LandingHandler)
	// r.HandleFunc("/about", AboutHandler)
	// r.HandleFunc("/blog", BlogHandler)
	r.HandleFunc("/projects/{action}", ProjectsHandler)
	// r.HandleFunc("/musician", MusicHandler)
	http.Handle("/", r)
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

//	func MusicHandler(w http.ResponseWriter, r *http.Request) {
//		tmpl, _ := template.ParseFiles("html/pages/landing/musician.html")
//		tmpl.Execute(w, nil)
//	}

var currentProjectIndex = 0

func ProjectsHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	action := vars["action"]

	switch action {
	case "first":
		currentProjectIndex = 0
	case "next":
		currentProjectIndex++
		if currentProjectIndex >= len(data.Projects) {
			currentProjectIndex = 0 // Wrap around to the first project
		}
	case "prev":
		currentProjectIndex--
		if currentProjectIndex < 0 {
			currentProjectIndex = len(data.Projects) - 1 // Wrap around to the last project
		}
	default:
		http.Error(w, "Invalid action", http.StatusBadRequest)
		return
	}

	project := data.Projects[currentProjectIndex]
	fmt.Println("Project: ", project)

	tmpl, _ := template.ParseFiles("html/templates/project.html")
	tmpl.Execute(w, project)
}
