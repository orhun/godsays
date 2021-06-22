use godsays::God;
use hyper::service;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::env;
use std::error::Error;

#[tokio::main]
pub async fn run_server(addr: &str, god: God) -> Result<(), Box<dyn Error + Send + Sync>> {
    let make_svc = service::make_service_fn(|_conn| {
        let god = god.clone();
        async move {
            Ok::<_, Infallible>(service::service_fn(move |req: Request<Body>| {
                let god = god.clone();
                async move {
                    match (req.method(), req.uri().path()) {
                        (&Method::GET, "/") => Response::builder()
                            .status(StatusCode::OK)
                            .body(Body::from(god.speak())),
                        (&Method::GET, "/json") => Response::builder()
                            .header("Content-Type", "application/json")
                            .status(StatusCode::OK)
                            .body(Body::from(format!(
                                r#"{{ "god_says": "{}" }}"#,
                                god.speak()
                            ))),
                        _ => Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("404")),
                    }
                }
            }))
        }
    });

    let server = Server::bind(&addr.parse()?).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}

fn main() {
    pretty_env_logger::init();
    let god = God::init("Happy.TXT", 32);
    if let Err(e) = run_server(
        &env::var("ADDR").unwrap_or_else(|_| String::from("127.0.0.1:3000")),
        god,
    ) {
        eprintln!("Server error: {:?}", e)
    }
}
