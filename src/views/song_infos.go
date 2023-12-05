package views

import (
	"fmt"
	"strings"
)

type SongInfo struct {
	Url       *string
	AudioName *string
	Name      string
}

var SongInfos = []SongInfo{
	{
		Url:       strPointer("/static/assets/art/album-art/adab.jpeg"),
		AudioName: nil,
		Name:      "△⁍⍝ß",
	},
	{
		Url:       strPointer("/static/assets/art/album-art/sgitf.jpeg"),
		AudioName: strPointer("sgitf"),
		Name:      "SGITF",
	},
	{
		Url:       strPointer("/static/assets/art/album-art/made-and-forgotten.jpeg"),
		AudioName: strPointer("made-and-forgotten"),
		Name:      "Made&Forgotten",
	},
	{
		Url:       strPointer("/static/assets/art/album-art/self-control.jpeg"),
		AudioName: strPointer("self-control"),
		Name:      "Self◁Control",
	},
	{
		Url:       nil,
		AudioName: strPointer("losing"),
		Name:      "Lσsing",
	},
	{
		Url:       nil,
		AudioName: strPointer("on-track"),
		Name:      "On¿Track",
	},
	{
		Url:       strPointer("/static/assets/art/album-art/criminal.jpeg"),
		AudioName: strPointer("criminal"),
		Name:      "Crimi∩al ",
	},
	{
		Url:       strPointer("/static/assets/art/album-art/siekg.jpeg"),
		AudioName: strPointer("siekg"),
		Name:      "SIEKG",
	},
	{
		Url:       strPointer("/static/assets/art/album-art/too-far.jpeg"),
		AudioName: strPointer("too-far"),
		Name:      "†°°Far",
	},
	{
		Url:       nil,
		AudioName: strPointer("up-with-them"),
		Name:      "With↑Them",
	},
	{
		Url:       strPointer("/static/assets/art/album-art/curl-away.jpeg"),
		AudioName: strPointer("curl-away"),
		Name:      "CurLªway",
	},
	{
		Url:       strPointer("/static/assets/art/album-art/making-myself.jpeg"),
		AudioName: strPointer("making-myself"),
		Name:      "MakingMy²Self",
	},
	{
		Url:       nil,
		AudioName: strPointer("no-one"),
		Name:      "No❍ne",
	},
	{
		Url:       nil,
		AudioName: strPointer("becoming"),
		Name:      "ℬecoming",
	},
	{
		Url:       strPointer("/static/assets/art/album-art/ydkyg.jpeg"),
		AudioName: strPointer("ydkyg"),
		Name:      "YDKYG",
	},
}

func strPointer(s string) *string {
	return &s
}

func FilterSongsByAudioName(targetAudioName string) (SongInfo, error) {
	for _, song := range SongInfos {
		if song.AudioName != nil && strings.EqualFold(*song.AudioName, targetAudioName) {
			return song, nil
		}
	}
	return SongInfo{}, fmt.Errorf("no song found with audio name: %s", targetAudioName)
}
