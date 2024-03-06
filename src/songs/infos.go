package songs

import (
	"fmt"
)

type SongInfo struct {
	HashUniqueImage bool
	FileName        string
	DisplayName     string
	Album           string
}

func (song SongInfo) ImgFilePath() string {
	if song.HashUniqueImage {
		return fmt.Sprintf("/static/assets/art/%s/%s.png", song.Album, song.FileName)
	}
	return fmt.Sprintf("/static/assets/art/%s/Album.png", song.Album)
}

func (song SongInfo) SongFilePath() string {
	return fmt.Sprintf("/static/assets/art/%s/songs/%s.wav", song.Album, song.FileName)
}

var AlbumMap = map[string][]SongInfo{
	"△⁍⍝ß":     AdabSongs,
	"TOYW":     ToywSongs,
	"LISAC":    LisacSongs,
	"Abstract": AbstractSongs,
}

var AlbumNames = []string{
	"△⁍⍝ß",
	"TOYW",
	"LISAC",
	"Abstract",
}

var AdabSongs = []SongInfo{
	{
		HashUniqueImage: true,
		FileName:        "sgitf",
		DisplayName:     "SGITF",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: true,
		FileName:        "made_and_forgotten",
		DisplayName:     "Made&Forgotten",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: true,
		FileName:        "self_control",
		DisplayName:     "Self◁Control",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: false,
		FileName:        "losing",
		DisplayName:     "Lσsing",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: false,
		FileName:        "on_track",
		DisplayName:     "On¿Track",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: true,
		FileName:        "criminal",
		DisplayName:     "Crimi∩al ",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: true,
		FileName:        "siekg",
		DisplayName:     "SIEKG",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: true,
		FileName:        "too_far",
		DisplayName:     "†°°Far",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: false,
		FileName:        "up_with_them",
		DisplayName:     "With↑Them",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: true,
		FileName:        "curl_away",
		DisplayName:     "CurLªway",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: true,
		FileName:        "making_myself",
		DisplayName:     "MakingMy²Self",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: false,
		FileName:        "no_one",
		DisplayName:     "No❍ne",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: false,
		FileName:        "becoming",
		DisplayName:     "ℬecoming",
		Album:           "△⁍⍝ß",
	},
	{
		HashUniqueImage: true,
		FileName:        "ydkyg",
		DisplayName:     "YDKYG",
		Album:           "△⁍⍝ß",
	},
}

var ToywSongs = []SongInfo{
	{
		HashUniqueImage: false,
		FileName:        "toyw",
		DisplayName:     "The One You Want",
		Album:           "TOYW",
	},
	{
		HashUniqueImage: true,
		FileName:        "fbtt",
		DisplayName:     "Flying Backwards Through Time",
		Album:           "TOYW",
	},
	{
		HashUniqueImage: true,
		FileName:        "ylamc",
		DisplayName:     "Young Love & Mushroom Caps",
		Album:           "TOYW",
	},
	{
		HashUniqueImage: true,
		FileName:        "fol",
		DisplayName:     "Feel Out Loud",
		Album:           "TOYW",
	},
	{
		HashUniqueImage: false,
		FileName:        "changing",
		DisplayName:     "Changing",
		Album:           "TOYW",
	},
	{
		HashUniqueImage: true,
		FileName:        "rise_above",
		DisplayName:     "Rise Above",
		Album:           "TOYW",
	},
	{
		HashUniqueImage: false,
		FileName:        "clarity",
		DisplayName:     "Clarity",
		Album:           "TOYW",
	},
}

var LisacSongs = []SongInfo{
	{
		HashUniqueImage: false,
		FileName:        "real",
		DisplayName:     "Real",
		Album:           "LISAC",
	},
	{
		HashUniqueImage: false,
		FileName:        "fixated",
		DisplayName:     "Fixated",
		Album:           "LISAC",
	},
	{
		HashUniqueImage: false,
		FileName:        "ocular_migraines",
		DisplayName:     "Ocular Migraines",
		Album:           "LISAC",
	},
	{
		HashUniqueImage: true,
		FileName:        "s+c",
		DisplayName:     "Sound and Color",
		Album:           "LISAC",
	},
	{
		HashUniqueImage: false,
		FileName:        "breathe",
		DisplayName:     "Breathe.",
		Album:           "LISAC",
	},
	{
		HashUniqueImage: false,
		FileName:        "galaxy_brain",
		DisplayName:     "Galaxy Brain",
		Album:           "LISAC",
	},
	{
		HashUniqueImage: true,
		FileName:        "the_miracle",
		DisplayName:     "The Miracle",
		Album:           "LISAC",
	},
	{
		HashUniqueImage: false,
		FileName:        "lost",
		DisplayName:     "Lost!",
		Album:           "LISAC",
	},
}

var AbstractSongs = []SongInfo{
	{
		HashUniqueImage: false,
		FileName:        "stop_waiting",
		DisplayName:     "Stop Waiting",
		Album:           "Abstract",
	},
	{
		HashUniqueImage: false,
		FileName:        "dreaming",
		DisplayName:     "Dreaming",
		Album:           "Abstract",
	},
	{
		HashUniqueImage: false,
		FileName:        "bombs",
		DisplayName:     "Bombs",
		Album:           "Abstract",
	},
	{
		HashUniqueImage: true,
		FileName:        "real_tv",
		DisplayName:     "Reality Television",
		Album:           "Abstract",
	},
	{
		HashUniqueImage: false,
		FileName:        "break_out",
		DisplayName:     "Break Out",
		Album:           "Abstract",
	},
	{
		HashUniqueImage: false,
		FileName:        "hd",
		DisplayName:     "HD",
		Album:           "Abstract",
	},
	{
		HashUniqueImage: false,
		FileName:        "lose_it",
		DisplayName:     "Lose It",
		Album:           "Abstract",
	},
	{
		HashUniqueImage: false,
		FileName:        "blankface",
		DisplayName:     "Blankface",
		Album:           "Abstract",
	},
}
