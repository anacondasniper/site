use crate::templates::IndexTemplate;
use chrono::{Datelike, Local, Timelike};

pub async fn index() -> IndexTemplate {
    let now = Local::now();
    let day_of_year = now.ordinal();

    IndexTemplate {
        month: now.month(),
        day: now.day(),
        year: now.year(),
        hour: now.hour(),
        minute: now.minute(),
        second: now.second(),
        day_count: day_of_year,
    }
}
