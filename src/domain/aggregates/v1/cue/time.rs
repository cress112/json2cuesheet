use std::ops::Add;

pub trait TimeFormatter {
    fn to_msf_string(&self) -> String;
}

#[derive(Debug, Copy, Clone)]
pub struct Time {
    minute: u8,
    second: u8,
    frame: u8,
}

impl Time {
    const MAX_MINUTE: u8 = 80;
    const MAX_SECOND: u8 = 60;
    const MAX_FRAME: u8 = 75;

    fn new(minute: u8, second: u8, frame: u8) -> Result<Time, String> {
        if minute >= 80 {
            return Err(format!("minute must be >=0 and <{}", Self::MAX_MINUTE));
        }
        if second >= 60 {
            return Err(format!("second must be >=0 and <{}", Self::MAX_SECOND));
        }
        if frame >= 75 {
            return Err(format!("frame must be >=0 and <{}", Self::MAX_FRAME));
        }
        Ok(Time {
            minute: minute,
            second: second,
            frame: frame,
        })
    }

    pub fn from_vec(time_vec: &Vec<u8>) -> Result<Time, String> {
        if time_vec.len() != 3 {
            return Err(String::from("length of time_vec must be 3"));
        }
        Self::new(time_vec[0], time_vec[1], time_vec[2])
    }

    fn to_frame(&self) -> u8 {
        self.minute * Self::MAX_SECOND * Self::MAX_FRAME
            + self.second * Self::MAX_FRAME
            + self.frame
    }
}

// 等価性の定義
impl PartialEq for Time {
    fn eq(&self, other: &Time) -> bool {
        self.minute == other.minute && self.second == other.second && self.frame == other.frame
    }
}

// 大小比較性の定義
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_frame().partial_cmp(&other.to_frame())
    }
}

// 加算性の定義
impl Add for Time {
    type Output = Result<Time, String>;

    fn add(self, other: Self) -> Result<Time, String> {
        let mut tmp_minute = self.minute + other.minute;
        let mut tmp_second = self.second + other.second;
        let mut tmp_frame = self.frame + other.frame;
        if tmp_frame >= Time::MAX_FRAME {
            tmp_frame -= Time::MAX_FRAME;
            tmp_second += 1;
        }
        if tmp_second >= Time::MAX_SECOND {
            tmp_second -= Time::MAX_SECOND;
            tmp_minute += 1;
        }
        Time::new(tmp_minute, tmp_second, tmp_frame)
    }
}

impl TimeFormatter for Time {
    fn to_msf_string(&self) -> String {
        format!("{:0>2}:{:0>2}:{:0>2}", self.minute, self.second, self.frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ok() -> Result<(), String> {
        let Ok(_) = Time::new(13, 59, 2) else {
            return Err("".to_string());
        };

        Ok(())
    }

    #[test]
    fn test_new_err() -> Result<(), String> {
        let Err(_) = Time::new(80, 0, 0) else {
            return Err("".to_string());
        };
        let Err(_) = Time::new(0, 60, 0) else {
            return Err("".to_string());
        };
        let Err(_) = Time::new(0, 0, 75) else {
            return Err("".to_string());
        };

        Ok(())
    }

    #[test]
    fn test_from_vec_ok() -> Result<(), String> {
        let Ok(result_ok) = Time::from_vec(&vec![1, 2, 3]) else {
            return Err("".to_string());
        };

        assert_eq!(result_ok, result_ok);
        Ok(())
    }

    #[test]
    fn test_from_vec_err() -> Result<(), String> {
        let Err(result_err_len) = Time::from_vec(&vec![1, 1, 1, 1]) else {
            return Err("".to_string());
        };
        let Err(result_err_range) = Time::from_vec(&vec![0, 90, 0]) else {
            return Err("".to_string());
        };

        assert_eq!(result_err_len, String::from("length of time_vec must be 3"));
        assert_eq!(result_err_range, String::from("second must be >=0 and <60"));
        Ok(())
    }

    #[test]
    fn test_to_msf_string() -> Result<(), Box<dyn std::error::Error>> {
        let time_1digits = Time::new(0, 5, 3)?;
        let time_2digits = Time::new(30, 15, 10)?;

        assert_eq!(time_1digits.to_msf_string(), String::from("00:05:03"));
        assert_eq!(time_2digits.to_msf_string(), String::from("30:15:10"));
        Ok(())
    }

    #[test]
    fn test_add() -> Result<(), Box<dyn std::error::Error>> {
        let time_1 = Time::new(10, 35, 36)?;
        let time_2 = Time::new(3, 40, 50)?;

        let result = (time_1 + time_2)?;

        assert_eq!(result.to_msf_string(), String::from("14:16:11"));
        Ok(())
    }
}
