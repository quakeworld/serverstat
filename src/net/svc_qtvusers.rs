use super::tokenize::tokenize;
use quake_text::bytestr;

pub(crate) const COMMAND: &[u8] = b"\xff\xff\xff\xffqtvusers";
pub(crate) const BUFFER_SIZE: usize = 4 * 1024;
const RESPONSE_HEADER: &[u8] = b"\xff\xff\xff\xffnqtvusers ";

/// Parses the response from a `qtvusers` request.
/// https://github.com/QW-Group/mvdsv/blob/master/src/sv_demo_qtv.c#L1379
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
}

impl TryFrom<&[u8]> for QtvusersResponse {
    type Error = Error;

    fn try_from(response: &[u8]) -> Result<Self, Self::Error> {
        // validate header
        if !response.starts_with(RESPONSE_HEADER) {
            return Err(Error::InvalidHeader);
        }

        // extract body
        let body = {
            let Some(end_pos) = response.iter().position(|&b| b == b'\n') else {
                return Err(Error::InvalidBody("missing newline".to_string()));
            };
            &response[RESPONSE_HEADER.len()..end_pos]
        };

        // parse body
        let tokens = tokenize(&bytestr::to_unicode(body));

        let Some((first, rest)) = tokens.split_first() else {
            return Err(Error::InvalidBody("missing stream id".to_string()));
        };

        Ok(QtvusersResponse {
            stream_id: first
                .parse::<u32>()
                .map_err(|_| Error::InvalidBody("invalid stream id: not a number".to_string()))?,
            client_names: rest.iter().map(|s| s.to_string()).collect(),
        })
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum Error {
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

    // #[tokio::test]
    // async fn test_qtvusers() -> Result<()> {
    //     // invalid address
    //     assert_eq!(
    //         query_qtvusers_async("INVALID_ADDRESS", Duration::from_secs(1))
    //             .await
    //             .unwrap_err()
    //             .to_string(),
    //         "failed to send message: invalid socket address".to_string()
    //     );

    //     // timeout
    //     assert_eq!(
    //         query_qtvusers_async("quake.se:28000", Duration::default())
    //             .await
    //             .unwrap_err()
    //             .to_string(),
    //         "timeout reached while waiting for response".to_string()
    //     );
    //     Ok(())
    // }

    #[test]
    fn test_parse_qtvusers_response() -> Result<()> {
        // invalid header
        assert_eq!(
            QtvusersResponse::try_from(b"foo".as_slice()).unwrap_err(),
            Error::InvalidHeader
        );

        // invalid body
        assert_eq!(
            QtvusersResponse::try_from(b"\xff\xff\xff\xffnqtvusers ".as_slice()).unwrap_err(),
            Error::InvalidBody("missing newline".to_string())
        );
        assert_eq!(
            QtvusersResponse::try_from(b"\xff\xff\xff\xffnqtvusers foo\n".as_slice()).unwrap_err(),
            Error::InvalidBody("invalid stream id: not a number".to_string())
        );

        // no users
        {
            let response = QtvusersResponse::try_from(b"\xff\xff\xff\xffnqtvusers 1\n".as_slice())?;
            assert_eq!(response.stream_id(), 1);
            assert!(response.client_names().is_empty());
        }

        // has users
        {
            let response = QtvusersResponse::try_from(
                b"\xff\xff\xff\xffnqtvusers 12 \"[streambot]\" \"XantoM\"\n".as_slice(),
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
