package songs

import (
	"fmt"
	"html/template"
	"net/http"

	"github.com/gorilla/mux"
)

type ArtPageInfo struct {
	CurrentAlbumName string
	NextAlbumName    *string
	PrevAlbumName    *string
	AlbumSongs       []SongInfo
	CurrentSong      SongInfo
}

func ArtHandler(w http.ResponseWriter, r *http.Request) {
	queryParams := r.URL.Query()
	vars := mux.Vars(r)

	albumName, ok := vars["albumName"]
	if !ok {
		albumName = "△⁍⍝ß"
	}
	playing := queryParams.Get("song")
	if playing == "" {
		switch albumName {
		case AlbumNames[0]:
			playing = AlbumMap[AlbumNames[0]][0].FileName
		case AlbumNames[1]:
			playing = AlbumMap[AlbumNames[1]][0].FileName
		case AlbumNames[2]:
			playing = AlbumMap[AlbumNames[2]][0].FileName
		case AlbumNames[3]:
			playing = AlbumMap[AlbumNames[3]][0].FileName
		}
	}
	album := AlbumMap[albumName]
	currentSong := getSongInList(album, playing)
	if currentSong != nil {
		tmpl := template.Must(template.ParseFiles("public/html/templates/art.html", "public/html/partials/album.html"))
		pageInfo := ArtPageInfo{
			CurrentAlbumName: albumName,
			NextAlbumName:    nextAlbum(albumName),
			PrevAlbumName:    prevAlbum(albumName),
			AlbumSongs:       album,
			CurrentSong:      *currentSong,
		}

		err := tmpl.Execute(w, pageInfo)
		if err != nil {
			fmt.Println("ERR: ", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
	}
}

func SongPlayerHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)

	album, ok := vars["albumName"]
	if !ok {
		album = "△⁍⍝ß"
	}
	albumSongs, _ := AlbumMap[album]
	playing, ok := vars["songName"]
	if !ok {
		playing = "sgitf"
	}
	song := getSongInList(albumSongs, playing)
	if song != nil {

		tmpl := template.Must(template.ParseFiles("public/html/partials/song-player.html"))

		err := tmpl.Execute(w, *song)
		if err != nil {
			fmt.Println("ERR: ", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
	}
}

func prevAlbum(AlbumName string) *string {
	for i, name := range AlbumNames {
		if name == AlbumName && i != 0 {
			prevAlbum := AlbumNames[i-1]
			return &prevAlbum
		}
	}
	return nil
}

func nextAlbum(AlbumName string) *string {
	for i, name := range AlbumNames {
		if name == AlbumName && i < len(AlbumNames)-1 {
			nextAlbum := AlbumNames[i+1]
			return &nextAlbum
		}
	}
	return nil
}

func getSongInList(songs []SongInfo, songName string) *SongInfo {
	for _, song := range songs {
		if song.FileName == songName {
			return &song
		}
	}
	return nil
}
