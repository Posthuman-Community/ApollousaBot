#[derive(Debug, PartialEq)]
pub struct TimezoneOffest {
    offset_hours: i32,
    offset_minutes: i32,
}

impl std::fmt::Display for TimezoneOffest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+03}:{:02}", self.offset_hours, self.offset_minutes)
    }
}

impl std::str::FromStr for TimezoneOffest {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 6 {
            return Err("Invalid timezone format");
        }

        let sign = match &s[0..1] {
            "+" => 1,
            "-" => -1,
            _ => return Err("Invalid timezone format"),
        };

        let hours: i32 = s[1..3].parse().map_err(|_| "Invalid hours")?;
        let minutes: i32 = s[4..6].parse().map_err(|_| "Invalid minutes")?;

        Ok(TimezoneOffest {
            offset_hours: sign * hours,
            offset_minutes: sign * minutes,
        })
    }
}

impl TimezoneOffest {
    fn new(offset_hours: i32, offset_minutes: i32) -> Self {
        Self {
            offset_hours,
            offset_minutes,
        }
    }
    pub fn to_duration(&self) -> i32 {
        self.offset_hours * 3600 + self.offset_minutes * 60
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_timezone() {
        let timezone_str = "+09:00";
        let timezone_offset: TimezoneOffest = timezone_str.parse().unwrap();
        let expected = TimezoneOffest::new(9, 0);
        assert_eq!(timezone_offset, expected);

        let timezone_str = "-09:40";
        let timezone_offset: TimezoneOffest = timezone_str.parse().unwrap();
        let expected = TimezoneOffest::new(-9, -40);

        assert_eq!(timezone_offset, expected);
    }

    #[test]
    fn test_to_duration() {
        let timezone_offset = TimezoneOffest::new(2, 30);
        let duration = timezone_offset.to_duration();
        assert_eq!(duration, 2 * 3600 + 30 * 60);
    }
}
