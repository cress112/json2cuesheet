use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Json {
    pub version: u8,
    pub title: String,
    pub genre: Option<String>,
    pub info: Option<Info>,
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub path: PathBuf,
    pub tracks: Vec<Track>,
    #[serde(rename = "type")]
    pub file_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub title: String,
    pub info: Option<Info>,
    pub pregap: Option<Pregap>,
    pub start_at: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub arranger: Option<String>,
    pub composer: Option<String>,
    pub lyricist: Option<String>,
    pub performer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pregap {
    pub duration: Vec<u8>,
    #[serde(rename = "type")]
    pub pregap_type: String,
}
