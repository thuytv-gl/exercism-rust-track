use std::fmt;

const DAY: i32 = 24 * 60;
const HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = hours * HOUR + minutes; // convert to total minutes
        minutes = (minutes % DAY) + DAY; // add a day, negative number will go backward
        minutes = minutes % DAY; // ensure minutes is within DAY

        Clock { hours: minutes / HOUR, minutes: minutes % HOUR }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, minutes + self.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
