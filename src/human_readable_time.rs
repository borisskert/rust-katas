/// https://www.codewars.com/kata/52685f7382004e774f0001f7/train/rust
pub fn make_readable(seconds: u32) -> String {
    HumanReadableTime::from_seconds(seconds).make_readable()
}

struct HumanReadableTime {
    seconds: u32,
}

impl HumanReadableTime {
    pub fn from_seconds(seconds: u32) -> Self {
        Self { seconds }
    }

    pub fn hours(&self) -> u32 {
        self.seconds / 3600
    }

    pub fn minutes(&self) -> u32 {
        (self.seconds % 3600) / 60
    }

    pub fn seconds(&self) -> u32 {
        self.seconds % 60
    }

    pub fn make_readable(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}",
            self.hours(),
            self.minutes(),
            self.seconds()
        )
    }
}
