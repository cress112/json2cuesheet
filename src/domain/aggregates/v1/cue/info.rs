use crate::share::util;

pub trait InfoFormatter {
    fn to_cdtext_strings(&self, indent: bool) -> Vec<String>;
}

#[derive(Debug)]
pub struct Info {
    arranger: Option<String>,
    composer: Option<String>,
    lyricist: Option<String>,
    performer: Option<String>,
}

impl Info {
    const KEY_ARRANGER: &str = "REM ARRANGER";
    const KEY_COMPOSER: &str = "REM COMPOSER";
    const KEY_LYRICIST: &str = "REM LYRICIST";
    const KEY_PERFORMER: &str = "PERFORMER";

    pub fn new(
        arranger: Option<String>,
        composer: Option<String>,
        lyricist: Option<String>,
        performer: Option<String>,
    ) -> Info {
        Info {
            arranger: arranger,
            composer: composer,
            lyricist: lyricist,
            performer: performer,
        }
    }

    fn build_string(base_string: String, indent: bool) -> String {
        if indent {
            return util::build_indent_string(&base_string);
        }
        base_string
    }
}

impl InfoFormatter for Info {
    fn to_cdtext_strings(&self, indent: bool) -> Vec<String> {
        let mut lines = Vec::new();

        if let Some(arranger) = &self.arranger {
            let key_value_string = format!("{} \"{}\"", Self::KEY_ARRANGER, arranger);
            lines.push(Self::build_string(key_value_string, indent));
        }
        if let Some(composer) = &self.composer {
            let key_value_string = format!("{} \"{}\"", Self::KEY_COMPOSER, composer);
            lines.push(Self::build_string(key_value_string, indent));
        }
        if let Some(lyricist) = &self.lyricist {
            let key_value_string = format!("{} \"{}\"", Self::KEY_LYRICIST, lyricist);
            lines.push(Self::build_string(key_value_string, indent));
        }
        if let Some(performer) = &self.performer {
            let key_value_string = format!("{} \"{}\"", Self::KEY_PERFORMER, performer);
            lines.push(Self::build_string(key_value_string, indent));
        }

        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() -> Result<(), String> {
        let result = Info::new(
            Some(String::from("arr")),
            None,
            Some(String::from("lyric")),
            None,
        );

        let Some(arranger) = result.arranger else {
            return Err(String::new());
        };
        match result.performer {
            Some(_) => {
                return Err(String::new());
            }
            None => {}
        }
        assert_eq!(arranger, String::from("arr"));
        Ok(())
    }

    #[test]
    fn test_to_cdtext_strings_full() {
        let test_info = Info::new(
            Some(String::from("arr")),
            Some(String::from("comp")),
            Some(String::from("lyric")),
            Some(String::from("perf")),
        );

        let result = test_info.to_cdtext_strings(true);

        assert_eq!(
            result.join("\n"),
            String::from(
                "  REM ARRANGER \"arr\"\n  REM COMPOSER \"comp\"\n  REM LYRICIST \"lyric\"\n  PERFORMER \"perf\""
            )
        );
    }

    #[test]
    fn test_to_cdtext_strings_partial() {
        let test_info = Info::new(
            None,
            Some(String::from("Talich Helfen")),
            None,
            Some(String::from("わたかん")),
        );

        let result = test_info.to_cdtext_strings(false);

        assert_eq!(
            result.join(","),
            String::from("REM COMPOSER \"Talich Helfen\",PERFORMER \"わたかん\"")
        );
    }

    #[test]
    fn test_to_cdtext_strings_empty() {
        let test_info = Info::new(None, None, None, None);

        let result = test_info.to_cdtext_strings(true);

        assert_eq!(result.join(""), String::new());
    }
}
