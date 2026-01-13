use super::PregapType;
use super::Time;
use super::TimeFormatter;

pub struct Pregap<T: TimeFormatter = Time> {
    pub r#type: PregapType,
    pub duration: T,
}
