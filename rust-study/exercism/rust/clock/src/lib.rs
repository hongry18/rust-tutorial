// #[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = (hours * 60 + minutes) % 1440;
        if minutes < 0 {
            minutes += 1440;
        }
        Clock { minutes }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes = self.minutes + minutes % 1440;
        if self.minutes < 0 {
            self.minutes += 1440;
        }
        self
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(format!("{:02}:{:02}", self.minutes / 60, self.minutes % 60).as_str())?;
        Ok(())
    }
}

impl std::fmt::Debug for Clock {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Clock")
            .field("minutes", &self.minutes)
            .finish()
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}
