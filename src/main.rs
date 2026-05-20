mod templates;
use templates::IndexTemplate;
use askama::Template;

fn main() {
    let tmpl = IndexTemplate { string: "user!".to_string() };
    println!("{}", tmpl.render().unwrap());
}
