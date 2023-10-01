use std::fmt::Display;
const DAY: i32 = 24 * 60;
const HOUR: i32 = 60;
#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    minutes: i32,
}
fn normalize(minutes: i32) -> i32 {
    ((minutes % DAY) + DAY) % DAY
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: normalize(hours * HOUR + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: normalize(self.minutes + minutes),
        }
    }
}
impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let minutes = self.minutes % HOUR;
        let hours = self.minutes / HOUR;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
