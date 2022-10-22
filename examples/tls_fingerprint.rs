//! `cargo run --example tls_fingerprint --features="okhttp, blocking" --no-default-features`
#![deny(warnings)]

use reqwest_impersonate::blocking::Client;
// use reqwest_impersonate::browser::ChromeVersion;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://tls.peet.ws/api/all";

    let client = Client::builder()
        .user_agent("amongus/21.3.7")
        .okhttp_builder()
        .build()
        .unwrap();
        

    client.get(url).send().unwrap();
    let mut res = client.get(url).send().unwrap();


    println!("Response: {:?} {}", res.version(), res.status());

    // copy the response body directly to stdout
    res.copy_to(&mut std::io::stdout())?;

    Ok(())
}
