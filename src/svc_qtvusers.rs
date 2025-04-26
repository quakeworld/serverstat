use crate::tokenize::tokenize;
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
        tinyudp::send_and_receive(address, &message, options).await?
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

impl QtvusersResponse {
    #[allow(dead_code)]
    pub fn stream_id(&self) -> usize {
        self.stream_id
    }

    pub fn client_names(&self) -> &[String] {
        &self.client_names
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

#[derive(Debug, Error)]
pub enum QtvusersResponseError {
    #[error("UDP error")]
    UdpError(#[from] tinyudp::TinyudpError),

    #[error("Invalid response header")]
    InvalidHeader,

    #[error("Invalid response body")]
    InvalidBody(String),

    #[error("Invalid stream id")]
    InvalidStreamId(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_try_from() -> Result<()> {
        {
            let bytes = b"\xff\xff\xff\xffnqtvusers 12 \"[streambot]\" \"XantoM\"\n".as_slice();
            let res = QtvusersResponse::try_from(bytes)?;
            assert_eq!(res.stream_id(), 12);
            assert_eq!(
                res.client_names(),
                &["[streambot]".to_string(), "XantoM".to_string()]
            );
        }
        {
            let bytes = b"\xff\xff\xff\xffnqtvusers 1\n".as_slice();
            let res = QtvusersResponse::try_from(bytes)?;
            assert_eq!(res.stream_id(), 1);
            assert!(res.client_names().is_empty());
        }

        Ok(())
    }
}
