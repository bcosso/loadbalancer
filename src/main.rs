#[macro_use]
extern crate actix_web;

use std::{env, io};
use std::sync::Mutex;

use actix_web::{middleware, App, HttpServer, web::Data};

mod request_methods;
mod configs;
mod lb_algotithms;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        let counter : u8 = 0;
        let data = Data::new(Mutex::new(counter));
    
        App::new()
            .app_data(Data::clone(&data))
            .wrap(middleware::Logger::default())
            .service(request_methods::get)
            .service(request_methods::execute_query)
            .service(request_methods::execute_query_method)

    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}