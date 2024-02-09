// #[derive(Debug, Template)]
// #[template(path = "timer.html")]
// pub struct Timer {
//     pub oob: bool,
//     pub msg: String,
// }

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login {
    pub name: String,
}