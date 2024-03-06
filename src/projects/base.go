package projects

import (
	"fmt"
	"html/template"
	"net/http"

	"github.com/gorilla/mux"
)

func ProjectsHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)

	projectName, ok := vars["name"]

	if !ok {
		projectName = Projects[0].Name
	}

	fmt.Println("Name: ", projectName)

	var p Project
	for _, project := range Projects {
		if project.Path == projectName {
			p = project
			break
		}
	}
	fmt.Println("Project: ", p)

	tmpl := template.Must(template.ParseFiles("public/html/templates/projects.html", "public/html/partials/project.html"))
	err := tmpl.Execute(w, p)
	if err != nil {
		fmt.Println(err)
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func RenderProject(w http.ResponseWriter, p Project) {
	tmpl, _ := template.ParseFiles("html/partials/projects.html")
	tmpl.Execute(w, p)
}
