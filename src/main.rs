#![feature(async_fn_in_trait)]

mod model;
mod repo;
mod api;

use std::env;
use dotenv::dotenv;
use repo::database::mongodb::MongoRepo;
use repo::database::base::Database;
use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use api::user::{get_user, new_user};
use api::credentail::varify_password;
use api::hidden::get_hidden;

#[actix_web::main]
async fn main() -> ::std::io::Result<()>  {

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv().ok();

    let mongodb_url = env::var("MONGOURL").expect("MONGOURL needs to be defined");
    
    let mongodb = MongoRepo::init(mongodb_url, "userauth".to_owned()).await;

    let mongodb_data = Data::new(mongodb);    

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
        .wrap(logger)
        .app_data(Data::clone(&mongodb_data))
        .service(get_user)
        .service(new_user)
        .service(varify_password)
        .service(get_hidden)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
    
}
