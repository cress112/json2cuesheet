use super::file_type::FileType;
use super::track::{Track, TrackFormatter};

pub trait FileFormatter {
    fn to_cdtext_strings(&self, initial_number: usize) -> Result<FileCdtextResult, String>;
}

pub struct File<T: TrackFormatter = Track> {
    path: String,
    tracks: Vec<T>,
    file_type: FileType,
}

pub struct FileCdtextResult {
    pub texts: Vec<String>,
    pub track_count: usize,
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
    fn to_cdtext_strings(&self, initial_number: usize) -> Result<FileCdtextResult, String> {
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
        let mut track_count = 0;
        for (index, track) in self.tracks.iter().enumerate() {
            let track_number = initial_number + index;
            let track_strings = match track.to_cdtext_strings(track_number, true) {
                Ok(res) => res,
                Err(res) => {
                    return Err(format!("error at track No.{}:\n{}", track_number, res));
                }
            };
            lines.extend(track_strings);
            track_count += 1;
        }

        Ok(FileCdtextResult {
            texts: lines,
            track_count,
        })
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
    fn test_to_cdtext_strings_initial_is_0() -> Result<(), Box<dyn std::error::Error>> {
        let path = String::from("/path/to/file.wav");
        let tracks = vec![MockTrack {}, MockTrack {}];
        let file_type = FileType::Wave;
        let file: File<MockTrack> = File::new(path, tracks, file_type);

        let result = file.to_cdtext_strings(1)?;

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
        assert_eq!(result.texts, expected);
        Ok(())
    }

    #[test]
    fn test_to_cdtext_strings_initial_is_not0() -> Result<(), Box<dyn std::error::Error>> {
        let path = String::from("/path/to/file.wav");
        let tracks = vec![MockTrack {}, MockTrack {}, MockTrack {}];
        let file_type = FileType::Wave;
        let file: File<MockTrack> = File::new(path, tracks, file_type);

        let result = file.to_cdtext_strings(4)?;

        let expected = vec![
            "FILE \"/path/to/file.wav\" WAVE",
            "4",
            "  track",
            "    strings",
            "    are here",
            "5",
            "  track",
            "    strings",
            "    are here",
            "6",
            "  track",
            "    strings",
            "    are here",
        ];
        assert_eq!(result.texts, expected);
        assert_eq!(result.track_count, 3);
        Ok(())
    }
}
