mod templates;
use templates::IndexTemplate;
use askama::Template;
use chrono::Local;
use chrono::Datelike;
use chrono::Timelike;

fn main() {
    let now = Local::now();
    let day_of_year = now.ordinal();

    let tmpl = IndexTemplate { 
        month: now.month(),
        day:   now.day(),
        year:  now.year(),
        hour:  now.hour(),
        minute:now.minute(),
        second:now.second(),
        day_count: day_of_year,
    };
    println!("{}", tmpl.render().unwrap());
}
