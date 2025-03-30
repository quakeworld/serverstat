use crate::tokenize::tokenize;
use anyhow::{Result, anyhow as e};
use quake_text::bytestr;
use std::time::Duration;
use tinyudp;

pub async fn qtvusers(address: &str, timeout: Duration) -> Result<QtvusersResponse> {
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

#[derive(Debug, Default, Eq, PartialEq)]
pub struct QtvusersResponse {
    pub stream_id: usize,
    pub client_names: Vec<String>,
}

impl TryFrom<&[u8]> for QtvusersResponse {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        // validate header
        let header = b"\xff\xff\xff\xffnqtvusers ".to_vec();

        if !bytes.starts_with(&header) {
            return Err(e!("Invalid response header"));
        }

        // extract body
        let body = {
            let end_pos = bytes
                .iter()
                .position(|&b| b == b'\n')
                .ok_or(e!("Invalid response body"))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_try_from() -> Result<()> {
        {
            let bytes = b"\xff\xff\xff\xffnqtvusers 12 \"[streambot]\" \"XantoM\"\n".as_slice();
            assert_eq!(
                QtvusersResponse::try_from(bytes)?,
                QtvusersResponse {
                    stream_id: 12,
                    client_names: vec!["[streambot]".to_string(), "XantoM".to_string()]
                }
            );
        }
        {
            let bytes = b"\xff\xff\xff\xffnqtvusers 1\n".as_slice();
            assert_eq!(
                QtvusersResponse::try_from(bytes)?,
                QtvusersResponse {
                    stream_id: 1,
                    client_names: vec![]
                }
            );
        }

        Ok(())
    }
}
