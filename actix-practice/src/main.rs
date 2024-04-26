
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
	App::new()
	.service(hello)
	.service(echo)
	.service(
	    web::scope("/app")
		.route("/index.html", web::get().to(index))
	)
	.route("/hey", web::get().to(manual_hello))
    })
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
