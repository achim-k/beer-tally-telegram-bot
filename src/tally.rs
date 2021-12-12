use chrono::{Datelike, Utc};

#[derive (Debug, Clone)]
struct Date {
    year: u32,
    day: u32,
    month: u32
}

impl Default for Date {
    fn default() -> Date {
        Date { year: 0, day: 0, month: 0 }
    }
}

#[derive(Clone)]
#[derive (Debug)]
pub struct UserTally {
    pub name: String,
    pub records: Vec<f32>,
    modified_date: Date
}

impl UserTally {
    // Constructor for UserTally
    pub fn new(username: &str) -> UserTally {
        UserTally {
            name: username.to_string(),
            records: Vec::new(),
            modified_date: Date::default()
        }
    }

    pub fn add_record(&mut self, value: f32) -> String {
        let now = Utc::now();

        let day : u32 = now.day();
        let month : u32 = now.day();
        let year : u32 = now.day();

        // If date is today
        if self.modified_date.day == day && self.modified_date.month == month && self.modified_date.year == year {
            let index = self.records.len() - 1;
            self.records[index] = value;
            self.save_date(day, month, year);
            format!("Modified today's record correctly")
        } else {
            self.records.push(value);
            self.save_date(day, month, year);

            format!("Saved today's record correctly")
        }
    }

    fn save_date(&mut self, day: u32, month: u32, year: u32) {
        self.modified_date.day = day;
        self.modified_date.month = month;
        self.modified_date.year = year;
    }
}