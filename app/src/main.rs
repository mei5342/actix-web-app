use actix_web::{web, App, HttpServer, HttpResponse};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let srv = HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("0.0.0.0", 8080))?
        .shutdown_timeout(60)
        .run();
    
    let srv_handle = srv.handle();

    tokio::spawn(srv);

    srv_handle.pause().await;

    srv_handle.resume().await;

    srv_handle.stop(true).await;

    Ok(())
}