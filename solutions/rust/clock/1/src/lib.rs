use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let finalHours = (minutes.div_euclid(60) + hours).rem_euclid(24);
        let finalMinutes = minutes.rem_euclid(60);
        Self {
            hours: finalHours,
            minutes: finalMinutes,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{:0>2}:{:0>2}", self.hours, self.minutes);
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let totalMinutes = self.minutes + minutes;
        let totalHours = self.hours + totalMinutes.div_euclid(60);
        let finalMinutes = totalMinutes.rem_euclid(60);
        let finalHours = totalHours.rem_euclid(24);

        Self {
            hours: finalHours,
            minutes: finalMinutes,
        }
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2})", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
