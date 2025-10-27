use anyhow::{Result, anyhow};
use clap::Parser;
use std::collections::HashSet;
use std::process;
use yt_transcript_rs::YouTubeTranscriptApi;

mod youtube_id;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The URL of the YouTube video
    #[arg(short, long)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("URL: {}", args.url);

    Ok(())
}

async fn fetch_transcript(video_id: &str) -> Result<String> {
    // Create API instance
    let api = YouTubeTranscriptApi::new(None, None, None)?;

    // Try to fetch transcript with English language preference
    match api.fetch_transcript(video_id, &["en"], false).await {
        Ok(transcript) => {
            // Clean up the transcript (equivalent to the awk script)
            Ok(clean_transcript(&transcript.text()))
        }
        Err(_) => {
            // If English fails, try without language preference
            match api.fetch_transcript(video_id, &[], false).await {
                Ok(transcript) => Ok(clean_transcript(&transcript.text())),
                Err(e) => Err(anyhow!("No subtitles found for this video: {}", e)),
            }
        }
    }
}

fn clean_transcript(content: &str) -> String {
    let mut seen_lines = HashSet::new();
    let mut result = Vec::new();

    for line in content.lines() {
        let cleaned_line = line.trim().replace('\r', "");

        // Skip empty lines
        if cleaned_line.is_empty() {
            continue;
        }

        // Skip duplicate lines (equivalent to awk script's !a[$0]++)
        if seen_lines.insert(cleaned_line.to_string()) {
            result.push(cleaned_line);
        }
    }

    result.join("\n")
}
