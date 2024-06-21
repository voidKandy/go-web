mod data;
use crate::auth::{middleware::SoftAuthExtension, model::FilteredUser};

use self::data::album_map;
use super::HandlerResult;
use askama::Template;
use axum::{
    extract::{Path, Query},
    response::Html,
    Extension,
};
use rand::Rng;
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::warn;

#[derive(Template)]
#[template(path = "music/layout.html")]
pub struct MusicPlayerLayoutTemplate {
    visible: bool,
    shuffle: bool,
    is_mini: bool,
    selection_view: bool,
    current_album_name: String,
    album_songs: Vec<SongInfo>,
    current_song_idx: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MusicPlayerParams {
    visible: Option<bool>,
    shuffle: Option<bool>,
    is_mini: Option<bool>,
    selection_view: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SongInfo {
    has_unique_image: bool,
    file_name: String,
    display_name: String,
    album: String,
}

pub async fn music_handler(
    Path((current_album_name, current_song_idx)): Path<(String, usize)>,
    Query(params): Query<MusicPlayerParams>,
) -> HandlerResult<Html<String>> {
    warn!(
        "\nMusic handler got: \nAlbum name: {}\nSongIdx: {}\nParams: {:?}\n",
        current_album_name, current_song_idx, params
    );

    let album_songs = album_map()
        .get(current_album_name.as_str())
        .ok_or(StatusCode::NOT_FOUND)?
        .to_vec();
    let tmpl = MusicPlayerLayoutTemplate {
        visible: params.visible.unwrap_or(false),
        shuffle: params.shuffle.unwrap_or(false),
        is_mini: params.is_mini.unwrap_or(false),
        selection_view: params.selection_view.unwrap_or(false),
        current_album_name,
        album_songs,
        current_song_idx,
    };

    Ok(match tmpl.render() {
        Ok(r) => Html(r),
        Err(err) => Html(format!("Error rendering Layout: {}", err.to_string())),
    })
}

impl MusicPlayerLayoutTemplate {
    fn random_song_url(&self) -> String {
        let map = data::album_map();
        let mut rng = rand::thread_rng();
        let album_idx: usize = rng.gen_range(0..map.keys().len());
        let album = map
            .keys()
            .nth(album_idx)
            .expect("Why did we fail to get an album name?");

        let song_idx: usize = rng.gen_range(0..map[album].len());

        if *album == self.current_album_name && song_idx == self.current_song_idx {
            return self.random_song_url();
        }

        return format!("/music/{}/{}?shuffle=true", album, song_idx);
    }

    fn next_song_url(&self) -> String {
        if self.current_song_idx + 1 < self.album_songs.len() {
            return format!(
                "/music/{}/{}",
                self.current_album_name,
                self.current_song_idx + 1
            );
        } else {
            return self.next_album_url();
        }
    }

    fn prev_song_url(&self) -> String {
        if (self.current_song_idx as i32 - 1) >= 0 {
            return format!(
                "/music/{}/{}",
                self.current_album_name,
                self.current_song_idx - 1
            );
        } else {
            return self.prev_album_url();
        }
    }

    fn next_album_url(&self) -> String {
        let mut result = "";
        let all_names: Vec<&str> = vec![data::ADAB, data::TOYW, data::LISAC, data::ABSTRACT];
        for (i, name) in all_names.iter().enumerate() {
            if *name == self.current_album_name {
                if i < all_names.len() - 1 {
                    result = all_names[i + 1]
                } else {
                    result = all_names[0]
                }
            }
        }
        format!("/music/{}/0", result)
    }

    fn prev_album_url(&self) -> String {
        let mut result = "";
        let all_names: Vec<&str> = vec![data::ADAB, data::TOYW, data::LISAC, data::ABSTRACT];
        for (i, name) in all_names.iter().enumerate() {
            if *name == self.current_album_name {
                if i != 0 {
                    result = all_names[i - 1]
                } else {
                    result = all_names[all_names.len() - 1]
                }
            }
        }
        format!("/music/{}/0", result)
    }

    fn album_display_name(&self) -> String {
        if self.current_album_name == data::ADAB {
            return "△⁍⍝ß".to_string();
        }
        return self.current_album_name.to_owned();
    }

    fn album_img_url(&self) -> String {
        format!("/public/music/{}/imgs/album.png", self.current_album_name)
    }
}

fn song_file_path(info: &&SongInfo) -> String {
    format!("/public/music/{}/wavs/{}.wav", info.album, info.file_name)
}

fn img_file_path(info: &&SongInfo) -> String {
    if info.has_unique_image {
        format!("/public/music/{}/imgs/{}.png", info.album, info.file_name)
    } else {
        format!("/public/music/{}/imgs/album.png", info.album)
    }
}
