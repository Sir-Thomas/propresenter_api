use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Playlist {
    id: PlaylistId,
    #[serde(rename = "type")]
    playlist_type: String,
    children: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlaylistId {
    uuid: String,
    name: String,
    index: u32,
}

fn main() -> () {
    let resp = reqwest::blocking::get("http://192.168.3.183:1025/v1/audio/playlists").unwrap().text();
    println!("{:#?}", resp);
    let playlists: Vec<Playlist> = serde_json::from_str(&resp.unwrap()).unwrap();
    println!("{:#?}", playlists);
    println!("{:#?}", playlists.len());
}
