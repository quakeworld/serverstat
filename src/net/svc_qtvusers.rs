use super::tokenize::tokenize;
use quake_text::bytestr;
use std::time::Duration;
use tinyudp::ReadOptions;

/// Sends a `qtvusers` request to the specified address and returns the response.
/// https://github.com/QW-Group/mvdsv/blob/master/src/sv_demo_qtv.c#L1379
const REQUEST_MESSAGE: &[u8] = b"\xff\xff\xff\xffqtvusers";
const REQUEST_BUFFER_SIZE: usize = 4 * 1024;
const EXPECTED_RESPONSE_HEADER: &[u8] = b"\xff\xff\xff\xffnqtvusers ";

pub(super) fn query_qtvusers(address: &str, timeout: Duration) -> Result<QtvusersResponse, Error> {
    let response = tinyudp::send_and_receive(
        address,
        REQUEST_MESSAGE,
        ReadOptions::new(timeout, REQUEST_BUFFER_SIZE),
    )?;
    Ok(QtvusersResponse::parse(&response)?)
}

/// Async version
#[cfg(feature = "tokio")]
pub(super) async fn query_qtvusers_async(
    address: &str,
    timeout: Duration,
) -> Result<QtvusersResponse, Error> {
    let response = tinyudp::send_and_receive_async(
        address,
        REQUEST_MESSAGE,
        ReadOptions::new(timeout, REQUEST_BUFFER_SIZE),
    )
    .await?;
    Ok(QtvusersResponse::parse(&response)?)
}

/// Parses the response from a `qtvusers` request.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct QtvusersResponse {
    /// The stream ID of the server.
    stream_id: u32,

    /// The names of the clients connected to the server.
    client_names: Vec<String>,
}

#[allow(dead_code)]
impl QtvusersResponse {
    pub fn stream_id(&self) -> u32 {
        self.stream_id
    }

    pub fn client_names(&self) -> &[String] {
        &self.client_names
    }

    pub fn parse(response: &[u8]) -> Result<Self, ParseError> {
        // validate header
        if !response.starts_with(EXPECTED_RESPONSE_HEADER) {
            return Err(ParseError::InvalidHeader);
        }

        // extract body
        let body = {
            let Some(end_pos) = response.iter().position(|&b| b == b'\n') else {
                return Err(ParseError::InvalidBody("missing newline".to_string()));
            };
            &response[EXPECTED_RESPONSE_HEADER.len()..end_pos]
        };

        // parse body
        let tokens = tokenize(&bytestr::to_unicode(body));

        let Some((first, rest)) = tokens.split_first() else {
            return Err(ParseError::InvalidBody("missing stream id".to_string()));
        };

        Ok(QtvusersResponse {
            stream_id: first.parse::<u32>().map_err(|_| {
                ParseError::InvalidBody("invalid stream id: not a number".to_string())
            })?,
            client_names: rest.iter().map(|s| s.to_string()).collect(),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    QueryError(#[from] tinyudp::Error),

    #[error(transparent)]
    ParseError(#[from] ParseError),
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub(crate) enum ParseError {
    #[error("invalid response header")]
    InvalidHeader,

    #[error("invalid response body: {0}")]
    InvalidBody(String),
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_qtvusers() -> Result<()> {
        // invalid address
        assert_eq!(
            query_qtvusers_async("INVALID_ADDRESS", Duration::from_secs(1))
                .await
                .unwrap_err()
                .to_string(),
            "failed to send message: invalid socket address".to_string()
        );

        // timeout
        assert_eq!(
            query_qtvusers_async("quake.se:28000", Duration::default())
                .await
                .unwrap_err()
                .to_string(),
            "timeout reached while waiting for response".to_string()
        );
        Ok(())
    }

    #[test]
    fn test_parse_qtvusers_response() -> Result<()> {
        // invalid header
        assert_eq!(
            QtvusersResponse::parse(b"foo").unwrap_err(),
            ParseError::InvalidHeader
        );

        // invalid body
        assert_eq!(
            QtvusersResponse::parse(b"\xff\xff\xff\xffnqtvusers ").unwrap_err(),
            ParseError::InvalidBody("missing newline".to_string())
        );
        assert_eq!(
            QtvusersResponse::parse(b"\xff\xff\xff\xffnqtvusers foo\n").unwrap_err(),
            ParseError::InvalidBody("invalid stream id: not a number".to_string())
        );

        // no users
        {
            let response = QtvusersResponse::parse(b"\xff\xff\xff\xffnqtvusers 1\n")?;
            assert_eq!(response.stream_id(), 1);
            assert!(response.client_names().is_empty());
        }

        // has users
        {
            let response = QtvusersResponse::parse(
                b"\xff\xff\xff\xffnqtvusers 12 \"[streambot]\" \"XantoM\"\n",
            )?;
            assert_eq!(response.stream_id(), 12);
            assert_eq!(
                response.client_names(),
                &["[streambot]".to_string(), "XantoM".to_string()]
            );
        }

        Ok(())
    }
}
