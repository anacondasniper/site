mod templates;
use templates::IndexTemplate;
use askama::Template;

fn main() {
    let tmpl = IndexTemplate { user: "Michael".to_string() };
    println!("{}", tmpl.render().unwrap());
}
