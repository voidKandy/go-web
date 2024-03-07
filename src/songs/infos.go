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
	return fmt.Sprintf("/static/assets/art/%s/album.png", song.Album)
}

func (song SongInfo) SongFilePath() string {
	return fmt.Sprintf("/static/assets/art/%s/songs/%s.wav", song.Album, song.FileName)
}

func (song SongInfo) AlbumDisplayName() string {
	switch song.Album {
	case "△⁍⍝ß":
		return "ADAB"
	case "LISAC":
		return "Lost in Sound and Color"
	case "TOYW":
		return "The One You Want"
	}
	return song.Album
}

var AlbumMap = map[string][]SongInfo{
	"ADAB":     AdabSongs,
	"TOYW":     ToywSongs,
	"LISAC":    LisacSongs,
	"Abstract": AbstractSongs,
}

var AlbumNames = []string{
	"ADAB",
	"TOYW",
	"LISAC",
	"Abstract",
}

var AdabSongs = []SongInfo{
	{
		HashUniqueImage: true,
		FileName:        "sgitf",
		DisplayName:     "SGITF",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: true,
		FileName:        "made_and_forgotten",
		DisplayName:     "Made&Forgotten",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: true,
		FileName:        "self_control",
		DisplayName:     "Self◁Control",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: false,
		FileName:        "losing",
		DisplayName:     "Lσsing",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: false,
		FileName:        "on_track",
		DisplayName:     "On¿Track",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: true,
		FileName:        "criminal",
		DisplayName:     "Crimi∩al ",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: true,
		FileName:        "siekg",
		DisplayName:     "SIEKG",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: true,
		FileName:        "too_far",
		DisplayName:     "†°°Far",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: false,
		FileName:        "up_with_them",
		DisplayName:     "With↑Them",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: true,
		FileName:        "curl_away",
		DisplayName:     "CurLªway",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: true,
		FileName:        "making_myself",
		DisplayName:     "MakingMy²Self",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: false,
		FileName:        "no_one",
		DisplayName:     "No❍ne",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: false,
		FileName:        "becoming",
		DisplayName:     "ℬecoming",
		Album:           "ADAB",
	},
	{
		HashUniqueImage: true,
		FileName:        "ydkyg",
		DisplayName:     "YDKYG",
		Album:           "ADAB",
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
		FileName:        "sound_and_color",
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
