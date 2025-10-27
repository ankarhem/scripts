use crate::youtube_id::YoutubeId;
use anyhow::Result;
use clap::*;
use yt_transcript_rs::YouTubeTranscriptApi;

mod youtube_id;

#[derive(Parser, Debug)]
#[command(name = "yt-sub", version, about)]
struct Args {
    /// The URL of the YouTube video
    url: String,

    /// The language code for the transcript (e.g., en, es, fr, de)
    #[arg(short, long, default_value = "en")]
    lang: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let video_id = match YoutubeId::parse(&args.url) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error parsing YouTube URL: {}", e);
            std::process::exit(1);
        }
    };

    let transcript = match fetch_transcript(&video_id, &args.lang).await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error fetching transcript: {}", e);
            std::process::exit(1);
        }
    };

    // write to stdout
    print!("{}", transcript);
}

async fn fetch_transcript(video_id: &YoutubeId, lang: &str) -> Result<String> {
    let api = YouTubeTranscriptApi::new(None, None, None)?;
    let languages = &[lang];
    let transcript = api
        .fetch_transcript(video_id.as_str(), languages, false)
        .await?;

    Ok(transcript.text())
}
