package routes

import (
	"fmt"
	"html/template"
	"net/http"
	"voidkandy-dot-space/src/email"
	"voidkandy-dot-space/src/views"

	"github.com/gorilla/mux"
)

func artHandler(w http.ResponseWriter, r *http.Request) {
	queryParams := r.URL.Query()
	playing := queryParams.Get("song")
	if playing == "" {
		playing = "sgitf"
	}
	currentSong, _ := views.FilterSongsByAudioName(playing)

	tmpl := template.Must(template.ParseFiles("public/html/templates/art.html", "public/html/partials/album-images.html"))

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

func songPlayerHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)

	playing, ok := vars["songName"]
	if !ok {
		playing = "sgitf"
	}
	song, _ := views.FilterSongsByAudioName(playing)

	tmpl := template.Must(template.ParseFiles("public/html/partials/song-player.html"))

	err := tmpl.Execute(w, song)
	if err != nil {
		fmt.Println("ERR: ", err)
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func projectsHandler(w http.ResponseWriter, r *http.Request) {
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

func sendEmailHandler(w http.ResponseWriter, r *http.Request) {
	type EmailInfo struct {
		IsGet         bool
		IsPostSuccess bool
	}

	tmpl := template.Must(template.ParseFiles("public/html/partials/email-form.html"))
	if r.Method == http.MethodGet {
		info := EmailInfo{
			IsGet:         true,
			IsPostSuccess: false,
		}
		err := tmpl.Execute(w, info)
		if err != nil {
			fmt.Println(err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
		return
	}

	if r.Method == http.MethodPost {

		err := r.ParseForm()
		if err != nil {
			http.Error(w, "Error parsing form", http.StatusInternalServerError)
			return
		}
		from := r.Form.Get("email")
		subject := r.Form.Get("subject")
		body := r.Form.Get("message")
		formattedString := fmt.Sprintf("%s%s%s", from, body, subject)
		fmt.Println(formattedString)
		emailErr := email.SendEmail(from, body, subject)

		info := EmailInfo{
			IsGet:         false,
			IsPostSuccess: emailErr == nil,
		}

		e := tmpl.Execute(w, info)
		if e != nil {
			fmt.Println(err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
		return
	}

	http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
}
