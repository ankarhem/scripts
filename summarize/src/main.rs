use anyhow::{Result, anyhow};
use clap::Parser;
use clap_stdin::MaybeStdin;
use serde_json::json;

mod config;
use config::Config;

#[derive(Parser, Debug)]
#[command(name = "summarize", version, about)]
struct Args {
    /// Custom command/question to ask about the text
    #[arg(short, long)]
    command: Option<String>,

    /// Text to summarize. To read from stdin, use '-'.
    text: MaybeStdin<String>,
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Args::parse();
    let config = config::Config::new()?;

    let text = args.text.trim();
    if text.is_empty() {
        return Err(anyhow!("No text provided to process"));
    }

    let response = summarize_text(&config, text, args.command.as_deref()).await?;
    println!("{}", response);

    Ok(())
}

async fn summarize_text(
    config: &Config,
    text: &str,
    custom_command: Option<&str>,
) -> Result<String> {
    let client = reqwest::Client::new();

    let base_prompt = "Please provide a concise summary of the following text.";
    let prompt = match custom_command {
        Some(command) => format!("{base_prompt} {command}"),
        None => base_prompt.to_string(),
    };

    let payload = json!({
        "model": config.model(),
        "max_tokens": 1024,
        "messages": [
            {
                "role": "user",
                "content": prompt
            },
            {
                "role": "user",
                "content": text
            }
        ]
    });

    let response = client
        .post(format!("{}/v1/messages", config.base_url()))
        .header("Content-Type", "application/json")
        .header("x-api-key", config.auth_token())
        .header("anthropic-version", "2023-06-01")
        .json(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow!(
            "API request failed with status {}: {}",
            status,
            error_text
        ));
    }

    let response_json: serde_json::Value = response.json().await?;

    response_json
        .pointer("/content/0/text")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("Invalid response format from API"))
}
