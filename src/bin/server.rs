use godsays::God;
use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::env;
use std::error::Error;
use tokio::net::TcpListener;

type Body = Full<Bytes>;

fn response(status: StatusCode, body: String) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(body)))
        .expect("response builder should be infallible for fixed bodies")
}

fn json_response(body: String) -> Response<Body> {
    Response::builder()
        .header("Content-Type", "application/json")
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from(body)))
        .expect("response builder should be infallible for fixed bodies")
}

pub async fn run_server(addr: &str, god: God) -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let god = god.clone();
        tokio::spawn(async move {
            let service = service_fn(move |req: Request<Incoming>| {
                let god = god.clone();
                async move {
                    let response = match (req.method(), req.uri().path()) {
                        (&Method::GET, "/") => response(StatusCode::OK, god.speak()),
                        (&Method::GET, "/json") => {
                            json_response(format!(r#"{{ "god_says": "{}" }}"#, god.speak()))
                        }
                        _ => response(StatusCode::NOT_FOUND, String::from("404")),
                    };

                    Ok::<_, Infallible>(response)
                }
            });

            if let Err(error) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("Connection error: {:?}", error);
            }
        });
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let god = God::init(32);
    if let Err(e) = run_server(
        &env::var("ADDR").unwrap_or_else(|_| String::from("127.0.0.1:3000")),
        god,
    )
    .await
    {
        eprintln!("Server error: {:?}", e)
    }
}
