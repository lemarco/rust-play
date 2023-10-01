use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
    }
}
impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let minutes = if self.minutes > 10 {
            format!("{}", self.minutes)
        } else {
            format!("0{}", self.minutes)
        };
        let hours = if self.hours > 10 {
            format!("{}", self.hours)
        } else {
            format!("0{}", self.hours)
        };

        write!(f, "{}:{}", hours, minutes)
    }
}
