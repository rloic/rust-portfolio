use std::cmp::Ordering;
use serde::export::fmt::Debug;
use serde::export::Formatter;
use std::fmt;
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Clone)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}

impl Eq for Date {}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.year != other.year {
            return self.year.cmp(&other.year)
        } else if self.month != other.month {
            return self.month.cmp(&other.month)
        }
        self.day.cmp(&other.day)
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut obj = serializer.serialize_struct("Date", 3)?;
        obj.serialize_field("year", &self.year)?;
        obj.serialize_field("month", &self.month)?;
        obj.serialize_field("day", &self.day)?;
        obj.end()
    }
}

impl ToString for Date {
    fn to_string(&self) -> String {
        let mut result = String::new();

        let month = match self.month {
            0 => "Jan",
            1 => "Feb",
            2 => "Mar",
            3 => "Apr",
            4 => "May",
            5 => "Jun",
            6 => "Jul",
            7 => "Aug",
            8 => "Sep",
            9 => "Oct",
            10 => "Nov",
            11 => "Dec",
            _ => panic!("Invalid month")
        };
        result.push_str(month);
        result.push(' ');
        if self.day < 10 {
            result.push('0');
        }
        result.push_str(&self.day.to_string());
        result.push_str(", ");
        result.push_str(&self.year.to_string());

        result
    }
}