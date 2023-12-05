package routes

import (
	"fmt"
	"html/template"
	"net/http"
	"voidkandy-dot-space/src/views"

	"github.com/gorilla/mux"
)

func InitializePageRoutes() {
	r := mux.NewRouter()
	r.HandleFunc("/", IndexHandler)
	r.HandleFunc("/landing", LandingHandler)
	r.HandleFunc("/about", AboutHandler)
	r.HandleFunc("/contact", ContactHandler)
	r.HandleFunc("/projects/{name}", ProjectsHandler)
	r.HandleFunc("/art", ArtHandler)
	http.Handle("/", Middleware(r))
}

func IndexHandler(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("public/html/index.html", "public/html/layout.html"))

	err := tmpl.Execute(w, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func LandingHandler(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("public/html/templates/landing.html"))

	err := tmpl.Execute(w, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func AboutHandler(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("public/html/templates/about.html"))

	err := tmpl.Execute(w, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func ContactHandler(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("public/html/templates/contact.html"))

	err := tmpl.Execute(w, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func ArtHandler(w http.ResponseWriter, r *http.Request) {
	queryParams := r.URL.Query()
	playing := queryParams.Get("song")
	if playing == "" {
		playing = "sgitf"
	}
	currentSong, _ := views.FilterSongsByAudioName(playing)

	tmpl := template.Must(template.ParseFiles("public/html/templates/art.html", "public/html/partials/song-player.html", "public/html/partials/album-images.html"))

	type ArtPageInfo struct {
		AllSongs    []views.SongInfo
		CurrentSong views.SongInfo
	}

	info := ArtPageInfo{AllSongs: views.SongInfos, CurrentSong: currentSong}

	err := tmpl.Execute(w, info)
	if err != nil {
		fmt.Println("ERR: ", err)
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func ProjectsHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)

	projectName, ok := vars["name"]

	if !ok {
		projectName = views.Projects[0].Name
	}

	fmt.Println("Name: ", projectName)

	var p views.Project
	for _, project := range views.Projects {
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
