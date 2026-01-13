use crate::domain::aggregates::v1::cue::{
    Cue, File as CueFile, FileType as CueFileType, Info as CueInfo, Pregap as CuePregap,
    PregapType as CuePregapType, Time as CueTime, Track as CueTrack,
};
use crate::domain::aggregates::v1::json::{
    File as JsonFile, Info as JsonInfo, Json, Track as JsonTrack,
};
use std::path;

pub struct Json2CueV1 {
    json: Json,
}

impl Json2CueV1 {
    const VERSION: u8 = 1;
    const FILE_TYPE: &str = "WAVE";
    const PREGAP_TYPE_INCLUDED: &str = "included";
    const PREGAP_TYPE_SILENT: &str = "silent";

    pub fn new(json: Json) -> Json2CueV1 {
        Json2CueV1 { json }
    }

    pub fn is_parsable_version(version: u8) -> bool {
        version == Self::VERSION
    }

    pub fn build(&self) -> Result<String, String> {
        if !Self::is_parsable_version(self.json.version) {
            return Err(format!("incompetible version: {}", self.json.version));
        };

        let title = self.json.title.clone();
        let genre = self.json.genre.clone();
        let info = Self::build_info(&self.json.info);
        let files = match Self::build_files(&self.json.files) {
            Ok(res) => res,
            Err(res) => {
                return Err(res);
            }
        };
        let cue = Cue::new(title, genre, info, files);

        cue.to_cdtext_string()
    }

    fn build_info(info: &Option<JsonInfo>) -> Option<CueInfo> {
        match info {
            None => None,
            Some(res) => Some(Self::build_confirmed_info(res)),
        }
    }

    fn build_confirmed_info(info: &JsonInfo) -> CueInfo {
        CueInfo::new(
            info.arranger.clone(),
            info.composer.clone(),
            info.lyricist.clone(),
            info.performer.clone(),
        )
    }

    fn build_files(files: &Vec<JsonFile>) -> Result<Vec<CueFile>, String> {
        let mut result: Vec<CueFile> = Vec::new();

        for (index, file) in files.iter().enumerate() {
            if file.file_type != Self::FILE_TYPE {
                return Err(format!(
                    "file type {} is unknown in index {}(path={:?})",
                    file.file_type, index, file.path
                ));
            }
            let Ok(abs_file_path_buf) = path::absolute(&file.path) else {
                return Err(format!("file path({:?}) is not parsable", file.path));
            }; //file.path.clone();
            let Some(abs_file_path) = abs_file_path_buf.to_str() else {
                return Err(format!("file path({:?}) is not parsable", file.path));
            };
            let tracks = match Self::build_tracks(&file.tracks) {
                Ok(res) => res,
                Err(res) => {
                    return Err(format!(
                        "tracks are not parsable in index={}\n{}",
                        index, res
                    ));
                }
            };
            result.push(CueFile::new(
                String::from(abs_file_path),
                tracks,
                CueFileType::Wave,
            ));
        }

        Ok(result)
    }

    fn build_tracks(tracks: &Vec<JsonTrack>) -> Result<Vec<CueTrack>, String> {
        let mut result: Vec<CueTrack> = Vec::new();

        for (index, track) in tracks.iter().enumerate() {
            let title = track.title.clone();
            let info = match &track.info {
                Some(info) => Some(Self::build_confirmed_info(info)),
                None => None,
            };
            let pregap = if let Some(pregap) = &track.pregap {
                let pregap_type: CuePregapType = match pregap.pregap_type.as_str() {
                    Self::PREGAP_TYPE_INCLUDED => CuePregapType::Included,
                    Self::PREGAP_TYPE_SILENT => CuePregapType::Silent,
                    _ => {
                        return Err(format!(
                            "unknown pregap_type in index={}: {}",
                            index, pregap.pregap_type
                        ));
                    }
                };
                let duration = match CueTime::from_vec(&pregap.duration) {
                    Ok(res) => res,
                    Err(res) => {
                        return Err(format!(
                            "pregap in index={} is not parsable: {:?}\n{}",
                            index, pregap.duration, res
                        ));
                    }
                };
                CuePregap {
                    r#type: pregap_type,
                    duration,
                }
            } else {
                let pregap_type = CuePregapType::Silent;
                let duration = match CueTime::from_vec(&vec![0, 0, 0]) {
                    Ok(res) => res,
                    Err(_) => {
                        return Err(String::from("unknown error occured in json2cue"));
                    }
                };
                CuePregap {
                    r#type: pregap_type,
                    duration,
                }
            };
            let postgap = if let Some(postgap) = &track.postgap {
                match CueTime::from_vec(postgap) {
                    Ok(res) => res,
                    Err(res) => {
                        return Err(format!(
                            "postgap in index={} is not parsable: {:?}\n{}",
                            index, postgap, res
                        ));
                    }
                }
            } else {
                match CueTime::from_vec(&vec![0, 0, 0]) {
                    Ok(res) => res,
                    Err(_) => {
                        return Err(String::from("unknown error occured in json2cue"));
                    }
                }
            };
            let start_at = if let Some(start_at) = &track.start_at {
                match CueTime::from_vec(&start_at) {
                    Ok(res) => res,
                    Err(res) => {
                        return Err(format!(
                            "start_at in index={} is not parsable: {:?}\n{}",
                            index, start_at, res
                        ));
                    }
                }
            } else {
                match CueTime::from_vec(&vec![0, 0, 0]) {
                    Ok(res) => res,
                    Err(_) => {
                        return Err(String::from("unknown error occured in json2cue"));
                    }
                }
            };
            result.push(CueTrack::new(title, info, pregap, postgap, start_at));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {}
