package songs

import (
	"fmt"
	"html/template"
	"net/http"

	"github.com/gorilla/mux"
)

type ArtPageInfo struct {
	CurrentAlbumName string
	AlbumSongs       []SongInfo
	CurrentSong      SongInfo
}

type AdjacentAlbum struct {
	AlbumName *string
	FirstSong *string
}

func (page ArtPageInfo) PrevAlbum() *AdjacentAlbum {
	for i, name := range AlbumNames {
		if name == page.CurrentAlbumName && i != 0 {
			AlbumName := AlbumNames[i-1]
			FirstSong := AlbumMap[AlbumName][0].FileName
			return &AdjacentAlbum{
				&AlbumName,
				&FirstSong,
			}

		}
	}
	return nil
}

func (page ArtPageInfo) NextAlbum() *AdjacentAlbum {
	for i, name := range AlbumNames {
		if name == page.CurrentAlbumName && i < len(AlbumNames)-1 {
			AlbumName := AlbumNames[i+1]
			FirstSong := AlbumMap[AlbumName][0].FileName
			return &AdjacentAlbum{
				&AlbumName,
				&FirstSong,
			}

		}
	}
	return nil
}

func (artPage ArtPageInfo) AlbumDisplayName() string {
	if artPage.CurrentAlbumName == "ADAB" {
		return "△⁍⍝ß"
	}
	return artPage.CurrentAlbumName
}

func ArtHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	queryParams := r.URL.Query()

	albumName := vars["albumName"]
	fmt.Println("Got Album ", albumName)
	album := AlbumMap[albumName]
	playing := queryParams.Get("song")
	fmt.Println("Playing: ", playing)
	currentSong := getSongInList(album, playing)
	fmt.Println("Got current song: ", currentSong)
	if currentSong != nil {
		tmpl := template.Must(template.ParseFiles("public/html/pages/art.html", "public/html/partials/album.html", "public/html/partials/song-player.html"))
		pageInfo := ArtPageInfo{
			CurrentAlbumName: albumName,
			AlbumSongs:       album,
			CurrentSong:      *currentSong,
		}

		err := tmpl.Execute(w, pageInfo)
		if err != nil {
			fmt.Println("ERR: ", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
	} else {
		http.Error(w, "Current song is nil", http.StatusPreconditionFailed)
	}
}

//	func SongPlayerHandler(w http.ResponseWriter, r *http.Request) {
//		vars := mux.Vars(r)
//
//		albumName, ok := vars["albumName"]
//		if !ok {
//			albumName = "ADAB"
//		}
//		albumSongs, _ := AlbumMap[albumName]
//		playing, ok := vars["songName"]
//		if !ok {
//			playing = "sgitf"
//		}
//		song := getSongInList(albumSongs, playing)
//		if song != nil {
//
//			tmpl := template.Must(template.ParseFiles("public/html/partials/song-player.html"))
//
//			err := tmpl.Execute(w, *song)
//			if err != nil {
//				fmt.Println("ERR: ", err)
//				http.Error(w, err.Error(), http.StatusInternalServerError)
//			}
//		}
//	}
func getSongInList(songs []SongInfo, songName string) *SongInfo {
	for _, song := range songs {
		if song.FileName == songName {
			return &song
		}
	}
	return nil
}
