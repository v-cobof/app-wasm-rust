use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode, Server};
use core::num;
use std::convert::Infallible;
use std::future::IntoFuture;
use std::net::SocketAddr;
use std::result::Result;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest;


static mut NUMBERS: Vec<f64> = Vec::new();
static mut DATA: Vec<f64> = Vec::new();


async fn handle_request(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "The valid endpoints are /init /create_order /create_orders /update_order /orders /delete_order",
        ))),


        (&Method::POST, "/inserir") => {


            let byte_stream = hyper::body::to_bytes(req).await?;
            let content = String::from_utf8(byte_stream.to_vec()).unwrap();
            let parsed : f64 = content.parse().unwrap();


            unsafe { NUMBERS.push(parsed) };

            if (unsafe { NUMBERS.len() } % 5 == 0)
            {
                let client = reqwest::Client::new();

                let res_num = client
                    .post("http://192.168.100.6:8080/receber-numeros")
                    .body(vec_to_string())
                    .send()
                    .await?;

                let body_num = res_num.text().await?;
                
                let media = calculate_mean();
                let std = calculate_std_deviation(media);
                let response = format!("{:.02} : {:.02}", media, std);

                let client2 = reqwest::Client::new();

                let res_stat = client2
                    .post("http://192.168.100.6:8080/receber-estatisticas")
                    .body(response)
                    .send()
                    .await?;

                let body_stat = res_stat.text().await?;
            }

            //Ok(Response::new(vec_to_string().into()))
            Ok(Response::new(vec_to_string().into()))
        },


        // CORS OPTIONS
        (&Method::OPTIONS, "/init") => Ok(response_build(&String::from(""))),
       
        (&Method::GET, "/init") => {
               
            Ok(response_build("{\"status\": funcionou}"))
        }

        (&Method::GET, "/obter") => {           

            Ok(response_build(vec_to_string().as_str()))
        }

        (&Method::GET, "/estatisticas") => {
               
            let media = calculate_mean();
            let std = calculate_std_deviation(media);

            Ok(response_build(format!("{:.02} : {:.02}", media, std).as_str()))
        }


        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}


fn vec_to_string() -> String {
    unsafe { NUMBERS.iter()
       .map(|&num| format!("{num}")) // Convert each f64 to String
       .collect::<Vec<String>>() // Collect into a vector of strings
       .join(", ") } // Join all strings with a separator
}


// CORS headers
fn response_build(body: &str) -> Response<Body> {
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "api,Keep-Alive,User-Agent,Content-Type")
        .body(Body::from(body.to_owned()))
        .unwrap()
}


fn calculate_mean() -> f64 {
    let sum: f64 = unsafe { NUMBERS.iter().sum() };
    let count = unsafe { NUMBERS.len() } as f64;
    sum / count
}


fn calculate_std_deviation(mean: f64) -> f64 {
    let variance: f64 = unsafe { NUMBERS.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() } / unsafe { NUMBERS.len() } as f64;
    variance.sqrt()
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
