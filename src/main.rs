use std::env;
use anyhow::Ok;
use anyhow::Result;
use futures::stream::TryStreamExt;
use misskey::prelude::*;
use misskey::WebSocketClient;
use misskey::HttpClient;
use url::Url;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<()> {
    let misskey_url = Url::parse("https://misskey.backspace.fm/api")?;
    let misskey_ws_url = Url::parse("wss://misskey.backspace.fm/streaming")?;
	let misskey_token = env::var("MISSKEY_API_TOKEN")?;
    let client = HttpClient::builder(misskey_url.clone())
        .token(misskey_token.clone())
        .build()?;
	// Configure and build a client using `WebSocketClientBuilder`.
	let wsclient = WebSocketClient::builder(misskey_ws_url)
			.token(misskey_token)
			.connect()
			.await?;

	// Run two asynchronous tasks simultaneously.
	// futures::try_join!(post(&client), timeline(&client))?;
    timeline(&client, &wsclient).await?;

	Ok(())
}

// Print notes on the local timeline
async fn timeline(client: &HttpClient, wsclient: &WebSocketClient) -> Result<()> {
    // Connect to the local timeline.
    // let mut stream = client.local_timeline().await?;
    let mut stream = wsclient.global_timeline().await?;
    let nurupo_re = Regex::new(r"^[\s\S]*(nu|Nu|ぬ|ゐ|ヌ)[\s\S]*(ru|ll|る|ゐ|ル)[\s\S]*(po|Pointer|ぽ|ポ)[\s\S]*$")?;
    let tikuwa_re = Regex::new(r"(ちくわ|大明神|tikuwa_daimyojin)")?;
    let mafumoko_re = Regex::new(r"(まふもこ|マフモコ|MFM|maccho|florida|mokomoko|マッチョ|フロリダ|モコモコ)")?;

    // Wait for the next note using `try_next` method from `TryStreamExt`.
    while let Some(note) = stream.try_next().await? {
        // `note` here has a type `misskey::model::note::Note`.
        if nurupo_re.is_match(&note.text.clone().unwrap_or_default().as_str()) {
            println!(
                "<@{}> {}",
                note.user.username,
                note.text.clone().unwrap_or_default()
            );
            client.react(note.id, ":galtu:").await?;
        }
        if tikuwa_re.is_match(&note.text.clone().unwrap_or_default().as_str()) {
            println!(
                "<@{}> {}",
                note.user.username,
                note.text.clone().unwrap_or_default()
            );
            client.react(note.id, ":dareda_imano:").await?;
        }
        if mafumoko_re.is_match(&note.text.clone().unwrap_or_default().as_str()) {
            println!(
                "<@{}> {}",
                note.user.username,
                note.text.clone().unwrap_or_default()
            );
            client.react(note.id, ":maccho_florida_mokomoko:").await?;
        }
    }

    Ok(())
}