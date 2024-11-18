use rand::Rng;
use std::thread::sleep;
use std::time::{Duration, Instant};


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), reqwest::Error> {
    let interval = Duration::from_secs(5);
    let mut next_time = Instant::now() + interval;

    loop {
        
        let client = reqwest::Client::new();
        let num = rand::thread_rng().gen_range(0..100);

        println!("{num}");

        let res = client
            .post("http://192.168.100.5:8080/inserir")
            .body(num.to_string())
            .send()
            .await?;
        let body = res.text().await?;

        //println!("POST: {}", body);

        sleep(next_time - Instant::now());
        next_time += interval;
    }

    Ok(())
}