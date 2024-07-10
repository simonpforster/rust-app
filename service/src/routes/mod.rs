use handlebars::Handlebars;
use hyper::Response;
use crate::router::ResponseResult;

pub mod private;
pub(crate) mod utils;
pub mod v1;


// TODO fix this
pub fn get_index(handlebars: &Handlebars) -> ResponseResult {
    let a = handlebars.render("index", &String::from("index")).unwrap();

    let r = Response::builder()
        .status(200)
        .body(utils::full(a))
        .unwrap();
    Ok(r)
}