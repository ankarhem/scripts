use std::str::FromStr;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::token::{rest, take_until};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct YoutubeId(String);

impl YoutubeId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn parse(input: &str) -> anyhow::Result<YoutubeId> {
        let id = parse_video_id
            .parse(input)
            .map_err(|e| anyhow::anyhow!("Failed to parse YouTube ID: {}", e))?;
        Ok(id)
    }
}

impl FromStr for YoutubeId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

fn parse_youtube_id(input: &mut &str) -> ModalResult<YoutubeId> {
    let _ = alt(("http://", "https://")).parse_next(input)?;

    let _ = alt(("youtube.com/watch?v=", "youtube.com/embed/", "youtu.be/")).parse_next(input)?;

    let id = parse_video_id(input)?;

    // Consume any trailing query parameters
    let _ = rest.parse_next(input)?;

    Ok(id)
}

fn parse_video_id(input: &mut &str) -> ModalResult<YoutubeId> {
    take_until(1.., '&')
        .map(|s: &str| YoutubeId(s.to_string()))
        .parse_next(input)
}
