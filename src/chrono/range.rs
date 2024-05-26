use std::mem;

use chrono::{Days, NaiveDate, NaiveWeek};

pub trait NaiveWeekExt {
    fn iter_days(&self) -> impl Iterator<Item = NaiveDate>;
}

impl NaiveWeekExt for NaiveWeek {
    fn iter_days(&self) -> impl Iterator<Item = NaiveDate> {
        DateRange(self.first_day(), self.last_day())
    }
}

const ONE_DAY: Days = Days::new(1);

struct DateRange(NaiveDate, NaiveDate);

impl Iterator for DateRange {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + ONE_DAY;
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}
