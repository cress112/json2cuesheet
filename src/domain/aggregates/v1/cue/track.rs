use super::info::{Info, InfoFormatter};
use super::pregap::Pregap;
use super::pregap_type::PregapType;
use super::time::{Time, TimeFormatter};
use crate::share::util;

pub trait TrackFormatter {
    fn to_cdtext_strings(&self, number: usize, indent: bool) -> Result<Vec<String>, String>;
}

pub struct Track<I: InfoFormatter = Info> {
    title: String,
    info: Option<I>,
    pregap: Pregap,
    start_at: Time,
}

impl<I: InfoFormatter> Track<I> {
    const KEY_TRACK: &str = "TRACK";
    const KEY_AUDIO: &str = "AUDIO";
    const KEY_TITLE: &str = "TITLE";
    const KEY_PREGAP: &str = "PREGAP";
    const KEY_INDEX: &str = "INDEX";

    pub fn new(title: String, info: Option<I>, pregap: Pregap, start_at: Time) -> Track<I> {
        Track {
            title,
            info,
            pregap,
            start_at,
        }
    }

    fn build_strings(base_strings: Vec<String>, indent: bool) -> Vec<String> {
        if indent {
            let result: Vec<String> =
                Vec::from_iter(base_strings.iter().map(|s| util::build_indent_string(s)));
            return result;
        }
        base_strings
    }
}

impl<I> TrackFormatter for Track<I>
where
    I: InfoFormatter,
{
    fn to_cdtext_strings(&self, number: usize, indent: bool) -> Result<Vec<String>, String> {
        let mut lines = Vec::new();

        // TRACK
        lines.push(format!(
            "{} {:0>2} {}",
            Self::KEY_TRACK,
            number,
            Self::KEY_AUDIO
        ));
        // * これ以降はインデント
        // TITLE
        let built_title = format!("{} \"{}\"", Self::KEY_TITLE, self.title);
        lines.push(util::build_indent_string(&built_title));
        // info
        match &self.info {
            Some(info) => {
                lines.extend(info.to_cdtext_strings(true));
            }
            None => {}
        }
        // pregap
        let Ok(time_0) = Time::from_vec(&vec![0, 0, 0]) else {
            return Err(String::from("Time is not buildable"));
        };
        if self.pregap.duration == time_0 {
            let index_01 = format!("{} 01 {}", Self::KEY_INDEX, self.start_at.to_msf_string());
            lines.push(util::build_indent_string(&index_01));
            return Ok(Self::build_strings(lines, indent));
        }
        match self.pregap.r#type {
            PregapType::Silent => {
                // PREGAP pregap
                let pregap_str = format!(
                    "{} {}",
                    Self::KEY_PREGAP,
                    self.pregap.duration.to_msf_string()
                );
                lines.push(util::build_indent_string(&pregap_str));
                // INDEX 01 start_at
                let index_01 = format!("{} 01 {}", Self::KEY_INDEX, self.start_at.to_msf_string());
                lines.push(util::build_indent_string(&index_01));
            }
            PregapType::Included => {
                // INDEX 00 start_at
                let index_00 = format!("{} 00 {}", Self::KEY_INDEX, self.start_at.to_msf_string());
                lines.push(util::build_indent_string(&index_00));
                // INDEX 01 start_at + pregap
                let Ok(index_01_time) = self.start_at + self.pregap.duration else {
                    return Err(String::from(
                        "(start_at + pregap) exceeds range of Time object",
                    ));
                };
                let index_01 = format!("{} 01 {}", Self::KEY_INDEX, index_01_time.to_msf_string());
                lines.push(util::build_indent_string(&index_01));
            }
        };

        Ok(Self::build_strings(lines, indent))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INFO: [&str; 2] = ["info1", "info2"];

    struct MockInfo {}
    impl InfoFormatter for MockInfo {
        fn to_cdtext_strings(&self, indent: bool) -> Vec<String> {
            let mut result: Vec<String> = Vec::new();
            if indent {
                result.extend(TEST_INFO.map(|s| format!("  {s}")));
                return result;
            }
            result.extend(TEST_INFO.iter().map(|s| String::from(*s)));
            result
        }
    }

    #[test]
    fn test_to_cdtext_strings_pregap_included() -> Result<(), Box<dyn std::error::Error>> {
        let title = String::from("ここにタイトル");
        let info = Some(MockInfo {});
        let pregap = Pregap {
            r#type: PregapType::Included,
            duration: Time::from_vec(&vec![0, 2, 15])?,
        };
        let start_at = Time::from_vec(&vec![0, 0, 0])?;
        let test_track: Track<MockInfo> = Track {
            title,
            info,
            pregap,
            start_at,
        };

        let result = test_track.to_cdtext_strings(1, false)?;

        let expected = vec![
            "TRACK 01 AUDIO",
            "  TITLE \"ここにタイトル\"",
            "  info1",
            "  info2",
            "  INDEX 00 00:00:00",
            "  INDEX 01 00:02:15",
        ];
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_to_cdtext_strings_pregap_silent() -> Result<(), Box<dyn std::error::Error>> {
        let title = String::from("ここにタイトル");
        let info = Some(MockInfo {});
        let pregap = Pregap {
            r#type: PregapType::Silent,
            duration: Time::from_vec(&vec![0, 3, 49])?,
        };
        let start_at = Time::from_vec(&vec![0, 0, 0])?;
        let test_track: Track<MockInfo> = Track {
            title,
            info,
            pregap,
            start_at,
        };

        let result = test_track.to_cdtext_strings(8, true)?;

        let expected = vec![
            "  TRACK 08 AUDIO",
            "    TITLE \"ここにタイトル\"",
            "    info1",
            "    info2",
            "    PREGAP 00:03:49",
            "    INDEX 01 00:00:00",
        ];
        assert_eq!(result, expected);
        Ok(())
    }
}
