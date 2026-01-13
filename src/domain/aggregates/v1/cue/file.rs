use super::file_type::FileType;
use super::track::{Track, TrackFormatter};

pub trait FileFormatter {
    fn to_cdtext_strings(&self) -> Result<Vec<String>, String>;
}
pub struct File<T: TrackFormatter = Track> {
    path: String,
    tracks: Vec<T>,
    file_type: FileType,
}

impl<T: TrackFormatter> File<T> {
    const KEY_FILE: &str = "FILE";
    const KEY_WAVE: &str = "WAVE";

    pub fn new(path: String, tracks: Vec<T>, file_type: FileType) -> File<T> {
        File {
            path,
            tracks,
            file_type,
        }
    }
}

impl<T: TrackFormatter> FileFormatter for File<T> {
    fn to_cdtext_strings(&self) -> Result<Vec<String>, String> {
        let mut lines: Vec<String> = Vec::new();

        // file
        match self.file_type {
            FileType::Wave => {}
        }
        lines.push(format!(
            "{} \"{}\" {}",
            Self::KEY_FILE,
            self.path,
            Self::KEY_WAVE,
        ));
        // * これ以降はインデント
        // file detail
        for (index, track) in self.tracks.iter().enumerate() {
            let track_number = index + 1;
            let track_strings = match track.to_cdtext_strings(track_number, true) {
                Ok(res) => res,
                Err(res) => {
                    return Err(format!("error at track No.{}:\n{}", track_number, res));
                }
            };
            lines.extend(track_strings);
        }

        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TRACK: [&str; 3] = ["track", "  strings", "  are here"];

    struct MockTrack {}
    impl TrackFormatter for MockTrack {
        fn to_cdtext_strings(&self, number: usize, indent: bool) -> Result<Vec<String>, String> {
            let mut result: Vec<String> = Vec::from([number.to_string()]);
            if indent {
                result.extend(TEST_TRACK.map(|s| format!("  {s}")));
                return Ok(result);
            }
            result.extend(TEST_TRACK.iter().map(|s| String::from(*s)));
            Ok(result)
        }
    }

    #[test]
    fn test_to_cdtext_strings() -> Result<(), Box<dyn std::error::Error>> {
        let path = String::from("/path/to/file.wav");
        let tracks = vec![MockTrack {}, MockTrack {}];
        let file_type = FileType::Wave;
        let file: File<MockTrack> = File::new(path, tracks, file_type);

        let result = file.to_cdtext_strings()?;

        let expected = vec![
            "FILE \"/path/to/file.wav\" WAVE",
            "1",
            "  track",
            "    strings",
            "    are here",
            "2",
            "  track",
            "    strings",
            "    are here",
        ];
        assert_eq!(result, expected);
        Ok(())
    }
}

/*
FILE "files/converted_Moving Still Life.wav" WAVE
  TRACK 01 AUDIO
    TITLE "Moving Still Life"
    PERFORMER "cero"
    INDEX 01 00:00:00
    PREGAP 00:02:00


*/
