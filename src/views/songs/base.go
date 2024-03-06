package songs

func PrevAlbum(AlbumName string) *string {
	for i, name := range AlbumNames {
		if name == AlbumName && i != 0 {
			prevAlbum := AlbumNames[i-1]
			return &prevAlbum
		}
	}
	return nil
}

func NextAlbum(AlbumName string) *string {
	for i, name := range AlbumNames {
		if name == AlbumName && i < len(AlbumNames)-1 {
			nextAlbum := AlbumNames[i+1]
			return &nextAlbum
		}
	}
	return nil
}

func GetSongInList(songs []SongInfo, songName string) *SongInfo {
	for _, song := range songs {
		if song.FileName == songName {
			return &song
		}
	}
	return nil
}
