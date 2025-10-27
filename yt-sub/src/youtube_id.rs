use std::str::FromStr;
use winnow::Parser;
use winnow::Result;
use winnow::combinator::{alt, opt};
use winnow::token::{rest, take_while};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct YoutubeId(String);

impl YoutubeId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn parse(input: &str) -> anyhow::Result<YoutubeId> {
        let id = parse_youtube_id
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

fn parse_youtube_id(input: &mut &str) -> Result<YoutubeId> {
    let _protocol = alt(("https://", "http://")).parse_next(input)?;
    let _www = opt("www.").parse_next(input)?;
    let _url =
        alt(("youtube.com/watch?v=", "youtube.com/embed/", "youtu.be/")).parse_next(input)?;

    let id = parse_raw_id(input)?;
    // Consume any trailing query parameters
    let _ = rest.parse_next(input)?;

    Ok(id)
}

fn parse_raw_id(input: &mut &str) -> winnow::Result<YoutubeId> {
    take_while(1.., |c: char| c.is_alphanumeric() || c == '-' || c == '_')
        .map(|id: &str| YoutubeId(id.to_string()))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_youtube_id() {
        let cases = vec![
            ("https://www.youtube.com/watch?v=dQw4w9WgXcQ", "dQw4w9WgXcQ"),
            ("http://youtube.com/embed/dQw4w9WgXcQ", "dQw4w9WgXcQ"),
            ("https://youtu.be/dQw4w9WgXcQ", "dQw4w9WgXcQ"),
            (
                "https://www.youtube.com/watch?v=dQw4w9WgXcQ&ab_channel=RickAstley",
                "dQw4w9WgXcQ",
            ),
        ];

        for (input, expected_id) in cases {
            let parsed_id = YoutubeId::parse(input).unwrap();
            assert_eq!(parsed_id.as_str(), expected_id);
        }
    }
}
