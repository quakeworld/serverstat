use crate::util::tokenize::tokenize;
use anyhow::Result;
use quake_text::bytestr;
use std::time::Duration;
use thiserror::Error;

/// Sends a `qtvusers` request to the specified address and returns the response.
pub async fn qtvusers(
    address: &str,
    timeout: Duration,
) -> Result<QtvusersResponse, QtvusersResponseError> {
    // https://github.com/QW-Group/mvdsv/blob/master/src/sv_demo_qtv.c#L1379
    let bytes = {
        let message = b"\xff\xff\xff\xffqtvusers".to_vec();
        let options = tinyudp::ReadOptions {
            timeout,
            buffer_size: 4 * 1024, // 4 kb
        };
        tinyudp::send_and_receive(address, &message, options)
            .await
            .map_err(|e| QtvusersResponseError::UdpError(e.to_string()))?
    };

    QtvusersResponse::try_from(bytes.as_slice())
}

/// Parses the response from a `qtvusers` request.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QtvusersResponse {
    /// The stream ID of the server.
    stream_id: usize,

    /// The names of the clients connected to the server.
    client_names: Vec<String>,
}

#[allow(dead_code)]
impl QtvusersResponse {
    pub fn stream_id(&self) -> usize {
        self.stream_id
    }

    pub fn client_names(&self) -> std::slice::Iter<String> {
        self.client_names.iter()
    }
}

impl TryFrom<&[u8]> for QtvusersResponse {
    type Error = QtvusersResponseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        // validate header
        let header = b"\xff\xff\xff\xffnqtvusers ".to_vec();

        if !bytes.starts_with(&header) {
            return Err(QtvusersResponseError::InvalidHeader);
        }

        // extract body
        let body = {
            let Some(end_pos) = bytes.iter().position(|&b| b == b'\n') else {
                return Err(QtvusersResponseError::InvalidBody(
                    "No newline found".to_string(),
                ));
            };
            &bytes[header.len()..end_pos]
        };

        // parse body
        let parts = tokenize(&bytestr::to_unicode(body));
        let stream_id = parts[0].parse::<usize>()?;
        let client_names = parts[1..].iter().map(|s| s.to_string()).collect();

        Ok(Self {
            stream_id,
            client_names,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum QtvusersResponseError {
    #[error("UDP error: {0}")]
    UdpError(String),

    #[error("Invalid response header")]
    InvalidHeader,

    #[error("Invalid response body")]
    InvalidBody(String),

    #[error("Invalid stream id")]
    InvalidStreamId(#[from] std::num::ParseIntError),
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
            qtvusers("INVALID_ADDRESS", Duration::from_secs(1))
                .await
                .unwrap_err(),
            QtvusersResponseError::UdpError(
                "failed to send message: invalid socket address".to_string()
            )
        );

        // timeout
        assert_eq!(
            qtvusers("quake.se:28000", Duration::default())
                .await
                .unwrap_err(),
            QtvusersResponseError::UdpError(
                "timeout reached while waiting for response".to_string()
            )
        );
        Ok(())
    }

    #[test]
    fn test_try_from() -> Result<()> {
        // invalid header
        assert_eq!(
            QtvusersResponse::try_from(b"foo".as_slice()).unwrap_err(),
            QtvusersResponseError::InvalidHeader
        );

        // invalid body
        assert_eq!(
            QtvusersResponse::try_from(b"\xff\xff\xff\xffnqtvusers ".as_slice()).unwrap_err(),
            QtvusersResponseError::InvalidBody("No newline found".to_string())
        );

        // invalid stream id
        assert_eq!(
            QtvusersResponse::try_from(b"\xff\xff\xff\xffnqtvusers foo\n".as_slice())
                .unwrap_err()
                .to_string(),
            "Invalid stream id".to_string()
        );

        // no users
        {
            let bytes = b"\xff\xff\xff\xffnqtvusers 1\n".as_slice();
            let res = QtvusersResponse::try_from(bytes)?;
            assert_eq!(res.stream_id(), 1);
            assert!(res.client_names().as_slice().is_empty());
        }

        // has users
        {
            let bytes = b"\xff\xff\xff\xffnqtvusers 12 \"[streambot]\" \"XantoM\"\n".as_slice();
            let res = QtvusersResponse::try_from(bytes)?;
            assert_eq!(res.stream_id(), 12);
            assert_eq!(
                res.client_names().as_slice(),
                &["[streambot]".to_string(), "XantoM".to_string()]
            );
        }

        Ok(())
    }
}
