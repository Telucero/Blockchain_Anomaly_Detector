mod api_connect;

#[tokio::main]
async fn main() {
    println!("Welcome to Blockchain Anomaly Detector!");

    match api_connect::fetch_data().await {
        Ok(response) => println!("Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}

