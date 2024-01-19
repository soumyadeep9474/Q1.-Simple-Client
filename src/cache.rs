use tokio::time::{sleep, Duration};
use tokio_tungstenite::WebSocketStream;
use futures_util::stream::StreamExt;

use tokio_tungstenite::connect_async;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct TickerPrice {
    symbol: String,
    price: String,
}

async fn connect_to_binance_ws() -> Result<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Error> {
    // Binance WebSocket API endpoint for symbol price ticker
    //Example
    let api_url = "wss://stream.binance.com:9443/ws/btcusdt@trade";

    // Establish a WebSocket connection
    let (ws_stream, _) = connect_async(api_url).await?;

    Ok(ws_stream)
}

pub async fn run_cache(times: u64) {
    let mut total_price = 0.0;
    let mut count = 0;

    // Establish a WebSocket connection
    if let Ok(mut ws_stream) = connect_to_binance_ws().await {
        // Get the current time
        let start_time = std::time::Instant::now();

        // Run for the specified duration
        while start_time.elapsed().as_secs() < times {
            // Receive and process WebSocket messages
            if let Some(Ok(msg)) = ws_stream.next().await {
                if let Ok(response) = serde_json::from_str::<TickerPrice>(&msg.to_text().unwrap()) {
                    if response.symbol == "BTCUSDT" {
                        // Convert the price to a float for calculation
                        if let Ok(price) = response.price.parse::<f64>() {
                            total_price += price;
                            count += 1;
                        }
                    }
                }
            }

            // Sleep for a short duration before processing the next message
            sleep(Duration::from_millis(100)).await;
        }

        // Calculate the average
        let average_price = if count > 0 {
            total_price / count as f64
        } else {
            0.0
        };

        // Print the result
        println!("Cache complete. The average USD price of BTC is: {}", average_price);

        // Save both the result of the aggregate and the data points to a file
        save_to_file(average_price, count);
    } else {
        eprintln!("Error: Failed to establish WebSocket connection.");
    }
}

fn save_to_file(average_price: f64, count: u64) {
    // Create or open a file for writing
    let mut file = File::create("cache_data.txt").expect("Failed to create file");

    // Write data to the file
    writeln!(file, "Average Price: {}", average_price).expect("Failed to write to file");
    writeln!(file, "Data Points Count: {}", count).expect("Failed to write to file");
}
