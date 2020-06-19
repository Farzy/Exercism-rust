use std::fmt;

// Store time as minutes, implement minimum necessary derives
#[derive(PartialEq, Debug)]
pub struct Clock(i32);

// Warning: Do not use '%' as it keep the sign of the dividend
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock((hours * 60 + minutes).rem_euclid(1440))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock((self.0 + minutes).rem_euclid(1440))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0 / 60, self.0 % 60)
    }
}
