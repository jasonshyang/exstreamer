use base64::Engine as _;
use exstreamer::{StreamBuilder, models::KrakenChannel};
use futures_util::StreamExt;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};

const GET_TOKEN_URL: &str = "https://api.kraken.com/";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    let token = get_token().await;
    tracing::info!("Received token: {}", token);

    let (mut kraken_stream, kraken_handler) = StreamBuilder::kraken(KrakenChannel::L3)
        .with_id(1)
        .with_token(token)
        .with_depth(1000)
        .with_symbol("BTC/USD")
        .connect()
        .await
        .unwrap();

    // Receive messages
    loop {
        tokio::select! {
            message = kraken_stream.next() => {
                if let Some(msg) = message {
                    tracing::info!("Received Kraken message: {:?}", msg);
                } else {
                    tracing::info!("No more messages to receive.");
                    break;
                }
            }
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Received Ctrl+C, shutting down...");
                break;
            }
        }
    }

    // Shutdown the connections;
    kraken_handler
        .shutdown()
        .await
        .expect("Failed to shutdown streamers");

    tracing::info!("Streamers shut down gracefully.");
}

#[derive(Debug, serde::Deserialize)]
struct TokenResponse {
    error: Vec<String>,
    result: Token,
}

#[derive(Debug, serde::Deserialize)]
struct Token {
    token: String,
    expires: u64,
}

async fn get_token() -> String {
    let client = reqwest::Client::new();
    let key = dotenvy::var("API_KEY").expect("API_KEY not set in .env");
    let secret = dotenvy::var("API_SECRET").expect("API_SECRET not set in .env");
    let path = "/0/private/GetWebSocketsToken";
    let nonce = get_nonce();
    let data = format!(r#"{{"nonce": "{}"}}"#, nonce);
    let signature = get_signature(&secret, &data, &nonce, path);
    let url = format!("{}{}", GET_TOKEN_URL, path);
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("API-Key", key)
        .header("API-Sign", signature)
        .body(data)
        .send()
        .await
        .expect("Failed to get token");

    let text = response.text().await.expect("Failed to read response text");
    tracing::info!("Response: {}", text);
    let response: TokenResponse =
        serde_json::from_str(&text).expect("Failed to parse token response");

    if response.error.len() > 0 {
        tracing::error!("Error getting token: {:?}", response.error);
        panic!("Failed to get token");
    }

    tracing::info!(
        "Token {} expires in {} seconds",
        response.result.token,
        response.result.expires
    );
    response.result.token
}

fn get_nonce() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    (now.as_millis()).to_string()
}

fn get_signature(secret: &str, data: &str, nonce: &str, path: &str) -> String {
    let decoded_key = base64::engine::general_purpose::STANDARD
        .decode(secret)
        .expect("Failed to decode secret key");
    let mut sha256 = Sha256::new();
    sha256.update(format!("{}{}", nonce, data));
    let sha256_digest = sha256.finalize();

    let mut hmac =
        Hmac::<Sha512>::new_from_slice(&decoded_key).expect("Failed to create HMAC instance");

    hmac.update(path.as_bytes());
    hmac.update(&sha256_digest);
    let result = hmac.finalize().into_bytes();

    base64::engine::general_purpose::STANDARD.encode(result)
}
