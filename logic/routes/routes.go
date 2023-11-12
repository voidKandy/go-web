package routes

import (
	"bytes"
	"fmt"
	"html/template"
	"net/http"
	"strconv"
	"voidkandy-dot-dev/logic/views"

	"github.com/gorilla/mux"
)

func InitializePageRoutes() {
	r := mux.NewRouter()
	r.HandleFunc("/", IndexHandler)
	// r.HandleFunc("/home", LandingHandler)
	r.HandleFunc("/about", AboutHandler)
	r.HandleFunc("/blog", BlogHandler)
	// r.HandleFunc("/projects/{name}", ProjectsHandler)
	// r.HandleFunc("/musician", MusicHandler)
	http.Handle("/", Middleware(r))
}

func IndexHandler(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("html/index.html", "html/landing.html"))

	err := tmpl.Execute(w, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func LandingHandler(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("html/landing.html"))

	err := tmpl.Execute(w, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func AboutHandler(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("html/templates/about.html"))

	err := tmpl.Execute(w, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func BlogHandler(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("html/templates/blog.html"))

	err := tmpl.Execute(w, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func renderProject(t *template.Template, data interface{}) (*bytes.Buffer, error) {
	rendered := bytes.NewBuffer(nil)
	if err := t.Execute(rendered, data); err != nil {
		return nil, err
	}
	return rendered, nil
}

func ProjectsHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)

	projectName, ok := vars["name"]
	idx, atoiErr := strconv.Atoi(projectName)
	nameIsInt := atoiErr == nil

	if nameIsInt {
		if len(views.Projects) >= idx+1 {
			projectName = views.Projects[idx].Name
		} else {
			projectName = views.Projects[0].Name
		}
	}

	if !ok {
		projectName = views.Projects[0].Name
	}

	fmt.Println("Name: ", projectName)

	var p views.Project
	for _, project := range views.Projects {
		if project.Name == projectName {
			p = project
			break
		}
	}
	fmt.Println("Project: ", p)

	tmpl, err := template.New("project").ParseFiles("html/partials/project.html")
	if err != nil {
		http.Error(w, "Failed to get partial", http.StatusInternalServerError)
		return
	}

	renderedProject, err := renderProject(tmpl, p)
	if err != nil {
		http.Error(w, "Failed to render project template", http.StatusInternalServerError)
		return
	}

	parentTmpl, err := template.ParseFiles("html/templates/projects.html")
	if err != nil {
		http.Error(w, "Failed to get template", http.StatusInternalServerError)
		return
	}

	fmt.Println("", renderedProject)

	parentTmpl.Execute(w, renderedProject)
}
