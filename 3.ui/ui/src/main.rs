use std::thread::sleep;
use std::time::{Duration, Instant};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode, Server};
use std::result::Result;
use std::convert::Infallible;
use std::future::IntoFuture;
use std::net::SocketAddr;
use std::collections::HashMap;
use reqwest;

/* 
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), reqwest::Error> {

    let interval = Duration::from_secs(10);
    let mut next_time = Instant::now() + interval;

    loop {
        
        let estatisticasRes = reqwest::get("http://192.168.100.5:8080/estatisticas").await?;
        let estatisticasBody = estatisticasRes.text().await?;
        println!("MEDIA E DESVIO PADRAO: {}", estatisticasBody);

        let numerosRes = reqwest::get("http://192.168.100.5:8080/obter").await?;
        let numerosBody = numerosRes.text().await?;
        println!("NUMEROS: {}", numerosBody);

        sleep(next_time - Instant::now());
        next_time += interval;
    }
    
    Ok(())
}
*/
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        // CORS OPTIONS
        (&Method::OPTIONS, "/init") => Ok(response_build(&String::from(""))),

        (&Method::GET, "/init") => {
               
            Ok(response_build("{\"status\": funcionou}"))
        }

        (&Method::POST, "/receber-numeros") => {


            let byte_stream = hyper::body::to_bytes(req).await?;
            let content = String::from_utf8(byte_stream.to_vec()).unwrap();
            println!("NUMEROS: {}", content);

            Ok(response_build("OK"))
        },

        (&Method::POST, "/receber-estatisticas") => {


            let byte_stream = hyper::body::to_bytes(req).await?;
            let content = String::from_utf8(byte_stream.to_vec()).unwrap();
            println!("MEDIA E DESVIO PADRAO: {}", content);

            Ok(response_build("OK"))
        },

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn response_build(body: &str) -> Response<Body> {
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "api,Keep-Alive,User-Agent,Content-Type")
        .body(Body::from(body.to_owned()))
        .unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
   


    let make_svc = make_service_fn(|_| {
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req)
            }))
        }
    });


    let server = Server::bind(&addr).serve(make_svc);


    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
   
    Ok(())
}