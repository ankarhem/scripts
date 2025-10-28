use anthropic_api::message::MessageResponse;
use anthropic_api::{AnthropicClient, AnthropicConfig};
use anyhow::{Result, anyhow};
use clap::Parser;
use clap_stdin::MaybeStdin;

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

    let text = args.text.trim();
    if text.is_empty() {
        return Err(anyhow!("No text provided to process"));
    }

    let anthropic_config = AnthropicConfig::builder().with_defaults()?.build()?;
    let client = AnthropicClient::new(anthropic_config.base_url(), anthropic_config.auth_token())?;

    let response = summarize_text(
        &client,
        anthropic_config.model(),
        text,
        args.command.as_deref(),
    )
    .await?;
    match response.content.first() {
        Some(content) => {
            println!("{}", content.text.trim());
        }
        None => {
            return Err(anyhow!("No summary returned from the API"));
        }
    }

    Ok(())
}

async fn summarize_text(
    client: &AnthropicClient,
    model: &str,
    text: &str,
    custom_command: Option<&str>,
) -> Result<MessageResponse> {
    let base_prompt = "Please provide a concise summary of the following text.";
    let prompt = match custom_command {
        Some(command) => format!("{base_prompt} {command}"),
        None => base_prompt.to_string(),
    };

    let message_request = anthropic_api::message::MessageRequest::builder()
        .model(model.to_string())
        .build()
        .add_user(prompt)
        .add_user(text);

    client.send_message(message_request).await
}
