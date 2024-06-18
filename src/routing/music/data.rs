use std::collections::HashMap;

use super::SongInfo;

pub const ADAB: &'static str = "ADAB";
pub const TOYW: &'static str = "TOYW";
pub const LISAC: &'static str = "LISAC";
pub const ABSTRACT: &'static str = "Abstract";

pub fn album_map() -> HashMap<&'static str, Vec<SongInfo>> {
    let mut map = HashMap::new();
    map.insert(ADAB, adab_songs());
    map.insert(TOYW, toyw_songs());
    map.insert(LISAC, lisac_songs());
    map.insert(ABSTRACT, abstract_songs());
    map
}

fn adab_songs() -> Vec<SongInfo> {
    vec![
        SongInfo {
            has_unique_image: true,
            file_name: "sgitf".to_string(),
            display_name: "SGITF".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "made_and_forgotten".to_string(),
            display_name: "Made&Forgotten".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "self_control".to_string(),
            display_name: "Self◁Control".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "losing".to_string(),
            display_name: "Lσsing".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "on_track".to_string(),
            display_name: "On¿Track".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "criminal".to_string(),
            display_name: "Crimi∩al ".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "siekg".to_string(),
            display_name: "SIEKG".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "too_far".to_string(),
            display_name: "†°°Far".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "up_with_them".to_string(),
            display_name: "With↑Them".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "curl_away".to_string(),
            display_name: "CurLªway".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "making_myself".to_string(),
            display_name: "MakingMy²Self".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "no_one".to_string(),
            display_name: "No❍ne".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "becoming".to_string(),
            display_name: "ℬecoming".to_string(),
            album: ADAB.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "ydkyg".to_string(),
            display_name: "YDKYG".to_string(),
            album: ADAB.to_string(),
        },
    ]
}

fn toyw_songs() -> Vec<SongInfo> {
    vec![
        SongInfo {
            has_unique_image: false,
            file_name: "toyw".to_string(),
            display_name: "The One You Want".to_string(),
            album: TOYW.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "fbtt".to_string(),
            display_name: "Flying Backwards Through Time".to_string(),
            album: TOYW.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "ylamc".to_string(),
            display_name: "Young Love & Mushroom Caps".to_string(),
            album: TOYW.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "fol".to_string(),
            display_name: "Feel Out Loud".to_string(),
            album: TOYW.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "changing".to_string(),
            display_name: "Changing".to_string(),
            album: TOYW.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "rise_above".to_string(),
            display_name: "Rise Above".to_string(),
            album: TOYW.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "clarity".to_string(),
            display_name: "Clarity".to_string(),
            album: TOYW.to_string(),
        },
    ]
}

fn lisac_songs() -> Vec<SongInfo> {
    vec![
        SongInfo {
            has_unique_image: false,
            file_name: "real".to_string(),
            display_name: "Real".to_string(),
            album: LISAC.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "fixated".to_string(),
            display_name: "Fixated".to_string(),
            album: LISAC.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "ocular_migraines".to_string(),
            display_name: "Ocular Migraines".to_string(),
            album: LISAC.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "sound_and_color".to_string(),
            display_name: "Sound and Color".to_string(),
            album: LISAC.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "breathe".to_string(),
            display_name: "Breathe.".to_string(),
            album: LISAC.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "galaxy_brain".to_string(),
            display_name: "Galaxy Brain".to_string(),
            album: LISAC.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "the_miracle".to_string(),
            display_name: "The Miracle".to_string(),
            album: LISAC.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "lost".to_string(),
            display_name: "Lost!".to_string(),
            album: LISAC.to_string(),
        },
    ]
}

fn abstract_songs() -> Vec<SongInfo> {
    vec![
        SongInfo {
            has_unique_image: false,
            file_name: "stop_waiting".to_string(),
            display_name: "Stop Waiting".to_string(),
            album: ABSTRACT.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "dreaming".to_string(),
            display_name: "Dreaming".to_string(),
            album: ABSTRACT.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "bombs".to_string(),
            display_name: "Bombs".to_string(),
            album: ABSTRACT.to_string(),
        },
        SongInfo {
            has_unique_image: true,
            file_name: "real_tv".to_string(),
            display_name: "Reality Television".to_string(),
            album: ABSTRACT.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "break_out".to_string(),
            display_name: "Break Out".to_string(),
            album: ABSTRACT.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "hd".to_string(),
            display_name: "HD".to_string(),
            album: ABSTRACT.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "lose_it".to_string(),
            display_name: "Lose It".to_string(),
            album: ABSTRACT.to_string(),
        },
        SongInfo {
            has_unique_image: false,
            file_name: "blankface".to_string(),
            display_name: "Blankface".to_string(),
            album: ABSTRACT.to_string(),
        },
    ]
}
