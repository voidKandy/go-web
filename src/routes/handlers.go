package routes

import (
	"fmt"
	"html/template"
	"net/http"
	"voidkandy-dot-space/src/views"
	"voidkandy-dot-space/src/views/songs"

	"github.com/gorilla/mux"
)

func artHandler(w http.ResponseWriter, r *http.Request) {
	queryParams := r.URL.Query()
	vars := mux.Vars(r)

	album, ok := vars["albumName"]
	if !ok {
		album = "△⁍⍝ß"
	}
	playing := queryParams.Get("song")
	if playing == "" {
		switch album {
		case songs.AlbumNames[0]:
			playing = songs.AlbumMap[songs.AlbumNames[0]][0].FileName
		case songs.AlbumNames[1]:
			playing = songs.AlbumMap[songs.AlbumNames[1]][0].FileName
		case songs.AlbumNames[2]:
			playing = songs.AlbumMap[songs.AlbumNames[2]][0].FileName
		case songs.AlbumNames[3]:
			playing = songs.AlbumMap[songs.AlbumNames[3]][0].FileName
		}
	}
	albumSongs, _ := songs.AlbumMap[album]
	currentSong := songs.GetSongInList(albumSongs, playing)
	if currentSong != nil {
		tmpl := template.Must(template.ParseFiles("public/html/templates/art.html", "public/html/partials/album.html"))

		type ArtPageInfo struct {
			CurrentAlbumName string
			NextAlbumName    *string
			PrevAlbumName    *string
			AlbumSongs       []songs.SongInfo
			CurrentSong      songs.SongInfo
		}

		// pageInfo := ArtPageInfo{AllSongs: views.SongInfos, CurrentSong: currentSong}
		pageInfo := ArtPageInfo{
			CurrentAlbumName: album,
			NextAlbumName:    songs.NextAlbum(album),
			PrevAlbumName:    songs.PrevAlbum(album),
			AlbumSongs:       albumSongs,
			CurrentSong:      *currentSong,
		}

		err := tmpl.Execute(w, pageInfo)
		if err != nil {
			fmt.Println("ERR: ", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
	}
}

func songPlayerHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)

	album, ok := vars["albumName"]
	if !ok {
		album = "△⁍⍝ß"
	}
	albumSongs, _ := songs.AlbumMap[album]
	playing, ok := vars["songName"]
	if !ok {
		playing = "sgitf"
	}
	song := songs.GetSongInList(albumSongs, playing)
	if song != nil {

		tmpl := template.Must(template.ParseFiles("public/html/partials/song-player.html"))

		err := tmpl.Execute(w, *song)
		if err != nil {
			fmt.Println("ERR: ", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
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

		r.ParseForm()
		web3form_url := "https://api.web3forms.com/submit"
		r.Form.Set("access_key", "b896a032-13cb-4639-a9ad-1fc1aacb1255")
		res, emailErr := http.PostForm(web3form_url, r.Form)
		fmt.Println("WEB3 RESPONSE: ", res)

		info := EmailInfo{
			IsGet:         false,
			IsPostSuccess: res.StatusCode == 200,
		}

		e := tmpl.Execute(w, info)
		if e != nil {
			fmt.Println(emailErr)
			http.Error(w, emailErr.Error(), http.StatusInternalServerError)
		}
		return
	}

	http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
}
