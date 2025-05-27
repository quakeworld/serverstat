use quake_infostring::parse_fields;
use quake_text::bytestr::to_unicode;

/// note: partial QtvStream
/// the response is missing client names which we get from sending
/// another query using svc_qtvusers
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct QtvStream {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) number: Option<u32>,
    pub(crate) address: Option<String>,
    pub(crate) client_count: u32,
}

impl TryFrom<&[u8]> for QtvStream {
    type Error = ParseError;

    fn try_from(response: &[u8]) -> Result<Self, Self::Error> {
        let tokens = parse_fields(to_unicode(response).as_str());

        // expected format:
        // "qtv 1 "Berlin QTV (1)" "1@berlin.qwsv.net:28000" 4"
        let [_, id_str, name, url, count_str] =
            <&[String; 5]>::try_from(&tokens[..]).map_err(|_| ParseError::InvalidTokenCount {
                expected: 5,
                actual: tokens.len(),
            })?;

        let id = id_str
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidNumber {
                field: "id",
                index: 1,
                value: id_str.clone(),
            })?;

        // expected format "1@berlin.qwsv.net:28000"
        let (number, address) = if let Some((num, addr)) = url.split_once('@') {
            match num.parse::<u32>() {
                Ok(n) => (Some(n), Some(addr.to_string())),
                Err(_) => (None, None),
            }
        } else {
            (None, None)
        };

        let client_count = count_str
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidNumber {
                field: "client_count",
                index: 4,
                value: count_str.clone(),
            })?;

        Ok(Self {
            id,
            name: name.to_string(),
            number,
            address,
            client_count,
        })
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub(crate) enum ParseError {
    #[error("invalid token count (expected {expected}, got {actual})")]
    InvalidTokenCount { expected: usize, actual: usize },

    #[error("invalid number format for '{field}' (index {index}): {value}")]
    InvalidNumber {
        field: &'static str,
        index: u32,
        value: String,
    },
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_try_from_bytes() -> Result<()> {
        // invalid token count
        assert_eq!(
            QtvStream::try_from(br#"qtv 2 "QUAKE.SE KTX Qtv (1)""#.as_slice()),
            Err(ParseError::InvalidTokenCount {
                expected: 5,
                actual: 3
            })
        );

        // invalid id
        assert_eq!(
            QtvStream::try_from(br#"qtv INVALID_ID "QUAKE.SE KTX Qtv (1)" "" 2"#.as_slice()),
            Err(ParseError::InvalidNumber {
                field: "id",
                index: 1,
                value: "INVALID_ID".to_string()
            })
        );

        // empty url
        assert_eq!(
            QtvStream::try_from(br#"qtv 2 "QUAKE.SE KTX Qtv (1)" "" 0"#.as_slice())?,
            QtvStream {
                id: 2,
                name: "QUAKE.SE KTX Qtv (1)".to_string(),
                number: None,
                address: None,
                client_count: 0,
            }
        );

        // invalid url
        assert_eq!(
            QtvStream::try_from(br#"qtv 2 "QUAKE.SE KTX Qtv (1)" "INVALID_ID@BAR" 0"#.as_slice())?,
            QtvStream {
                id: 2,
                name: "QUAKE.SE KTX Qtv (1)".to_string(),
                number: None,
                address: None,
                client_count: 0,
            }
        );

        // invalid client count
        assert_eq!(
            QtvStream::try_from(
                br#"qtv 2 "QUAKE.SE KTX Qtv (1)" "1@quake.se:28000" INVALID_COUNT"#.as_slice()
            ),
            Err(ParseError::InvalidNumber {
                field: "client_count",
                index: 4,
                value: "INVALID_COUNT".to_string()
            })
        );

        // valid
        assert_eq!(
            QtvStream::try_from(
                br#"qtv 2 "QUAKE.SE KTX Qtv (1)" "1@quake.se:28000" 0"#.as_slice()
            )?,
            QtvStream {
                id: 2,
                name: "QUAKE.SE KTX Qtv (1)".to_string(),
                number: Some(1),
                address: Some("quake.se:28000".to_string()),
                client_count: 0,
            }
        );

        Ok(())
    }
}
