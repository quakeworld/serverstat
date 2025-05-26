use super::tokenize::tokenize;
use crate::GenericClient;
use anyhow::Result;
use quake_serverinfo::Settings;
use quake_text::bytestr::to_unicode;
use std::io::{BufRead, Cursor};
use std::time::Duration;
use thiserror::Error;
use tinyudp::ReadOptions;

// see: https://github.com/QW-Group/mvdsv/blob/master/src/sv_main.c#L603-L610
// #define STATUS_OLDSTYLE                 0
// #define STATUS_SERVERINFO               1
// #define STATUS_PLAYERS                  2
// #define STATUS_SPECTATORS               4
// #define STATUS_SPECTATORS_AS_PLAYERS    8 //for ASE - change only frags: show as "S"
// #define STATUS_SHOWTEAMS                16
// #define STATUS_SHOWQTV                  32
// #define STATUS_SHOWFLAGS                64
// svc_status 119 = all except for STATUS_SPECTATORS_AS_PLAYERS

/// Sends a `status 119` query to the specified address and returns the parsed response.
const REQUEST_MESSAGE: &[u8] = b"\xff\xff\xff\xffstatus 119";
const REQUEST_BUFFER_SIZE: usize = 64 * 1024;

pub(super) fn query_status_119(
    address: &str,
    timeout: Duration,
) -> Result<Status119Response, Status119ResponseError> {
    let response = tinyudp::send_and_receive(
        address,
        REQUEST_MESSAGE,
        ReadOptions::new(timeout, REQUEST_BUFFER_SIZE),
    )?;
    Status119Response::parse(response.as_slice())
}

#[cfg(feature = "tokio")]
pub(super) async fn query_status_119_async(
    address: &str,
    timeout: Duration,
) -> Result<Status119Response, Status119ResponseError> {
    let response = tinyudp::send_and_receive_async(
        address,
        REQUEST_MESSAGE,
        ReadOptions::new(timeout, REQUEST_BUFFER_SIZE),
    )
    .await?;
    Status119Response::parse(response.as_slice())
}

/// Represents the response to a `status 119` query.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct Status119Response {
    settings: Settings,
    clients: Vec<GenericClient>,
    qtv_stream: Option<Status119QtvStream>,
}

#[allow(dead_code)]
impl Status119Response {
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn clients(&self) -> impl Iterator<Item = &GenericClient> {
        self.clients.iter()
    }

    pub fn qtv_stream(&self) -> &Option<Status119QtvStream> {
        &self.qtv_stream
    }

    pub(crate) fn parse(response: &[u8]) -> Result<Self, Status119ResponseError> {
        // validate header
        let header = vec![255, 255, 255, 255, 110];

        if !response.starts_with(&header) {
            return Err(Status119ResponseError::InvalidHeader);
        }

        // parse body
        let body = &response[header.len()..];
        let rows: Vec<Vec<u8>> = Cursor::new(body).split(10).filter_map(|l| l.ok()).collect();

        const MIN_SERVERINFO_LENGTH: usize = "hostname\\x".len();

        if rows.is_empty() || rows[0].len() < MIN_SERVERINFO_LENGTH {
            return Err(Status119ResponseError::InvalidBody);
        }

        // parse serverinfo
        let settings = quake_serverinfo::Settings::from(rows[0].as_slice());

        // parse clients and additional info
        let mut clients: Vec<GenericClient> = vec![];
        let mut qtv_stream: Option<Status119QtvStream> = None;

        for row in rows {
            if row.starts_with(b"qtv ") {
                qtv_stream = Status119QtvStream::parse(row.as_slice()).ok();
            } else if let Ok(client) = GenericClient::try_from(row.as_slice()) {
                clients.push(client);
            }
        }

        Ok(Status119Response {
            settings,
            clients,
            qtv_stream,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Status119QtvStream {
    id: u32,
    name: String,
    number: Option<u32>,
    address: Option<String>,
    client_count: u32,
}

impl Status119QtvStream {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn number(&self) -> Option<u32> {
        self.number
    }

    pub fn address(&self) -> Option<&str> {
        self.address.as_deref()
    }

    pub fn client_count(&self) -> u32 {
        self.client_count
    }

    pub fn url(&self) -> Option<String> {
        Some(format!("{}@{}", self.number()?, self.address()?))
    }

    pub(crate) fn parse(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        // todo validate number of parts
        let parts: Vec<String> = tokenize(to_unicode(bytes).as_str());
        let id = parts[1].parse::<u32>().unwrap_or_default();
        let name = parts[2].to_string();
        let url = parts[3].to_string();
        let (number, address) = url
            .split_once('@')
            .map(|(num_str, addr)| (num_str.parse::<u32>().ok(), Some(addr.to_string())))
            .unwrap_or((None, None));
        let client_count = parts[4].parse::<u32>().unwrap_or_default();

        Ok(Self {
            id,
            name,
            number,
            address,
            client_count,
        })
    }
}

#[derive(Debug, Error)]
pub enum Status119ResponseError {
    #[error("query error: {0}")]
    UdpError(#[from] tinyudp::Error),

    #[error("invalid response header")]
    InvalidHeader,

    #[error("invalid response body")]
    InvalidBody,
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_status119response_parse() -> Result<()> {
        // invalid response header
        {
            let res = Status119Response::parse([0].as_slice());
            assert_eq!(
                res.unwrap_err().to_string(),
                "invalid response header".to_string()
            );
        }

        // invalid resposne body
        {
            let res = Status119Response::parse([255, 255, 255, 255, 110, 0].as_slice());
            assert_eq!(
                res.unwrap_err().to_string(),
                "invalid response body".to_string()
            );
        }

        // with clients
        {
            let bytes = [
                255, 255, 255, 255, 110, 92, 109, 97, 120, 102, 112, 115, 92, 55, 55, 92, 112, 109,
                95, 107, 116, 106, 117, 109, 112, 92, 49, 92, 42, 118, 101, 114, 115, 105, 111,
                110, 92, 77, 86, 68, 83, 86, 32, 48, 46, 51, 54, 92, 42, 122, 95, 101, 120, 116,
                92, 53, 49, 49, 92, 42, 97, 100, 109, 105, 110, 92, 108, 111, 108, 101, 107, 32,
                60, 108, 111, 108, 101, 107, 64, 113, 117, 97, 107, 101, 49, 46, 112, 108, 62, 92,
                107, 116, 120, 118, 101, 114, 92, 49, 46, 52, 50, 92, 115, 118, 95, 97, 110, 116,
                105, 108, 97, 103, 92, 50, 92, 110, 101, 101, 100, 112, 97, 115, 115, 92, 52, 92,
                109, 97, 120, 115, 112, 101, 99, 116, 97, 116, 111, 114, 115, 92, 49, 50, 92, 42,
                103, 97, 109, 101, 100, 105, 114, 92, 113, 119, 92, 116, 101, 97, 109, 112, 108,
                97, 121, 92, 50, 92, 109, 111, 100, 101, 92, 50, 111, 110, 50, 92, 116, 105, 109,
                101, 108, 105, 109, 105, 116, 92, 49, 48, 92, 100, 101, 97, 116, 104, 109, 97, 116,
                99, 104, 92, 51, 92, 42, 113, 118, 109, 92, 115, 111, 92, 42, 112, 114, 111, 103,
                115, 92, 115, 111, 92, 109, 97, 120, 99, 108, 105, 101, 110, 116, 115, 92, 52, 92,
                109, 97, 112, 92, 122, 116, 110, 100, 109, 51, 92, 115, 101, 114, 118, 101, 114,
                100, 101, 109, 111, 92, 50, 111, 110, 50, 95, 114, 101, 100, 95, 118, 115, 95, 98,
                108, 117, 101, 91, 122, 116, 110, 100, 109, 51, 93, 50, 48, 50, 52, 48, 55, 49, 54,
                45, 49, 50, 52, 52, 46, 109, 118, 100, 92, 104, 111, 115, 116, 110, 97, 109, 101,
                92, 122, 97, 115, 97, 100, 122, 107, 97, 58, 50, 55, 53, 48, 49, 32, 40, 114, 101,
                100, 32, 118, 115, 46, 32, 98, 108, 117, 101, 41, 135, 92, 102, 112, 100, 92, 50,
                48, 54, 92, 115, 116, 97, 116, 117, 115, 92, 57, 32, 109, 105, 110, 32, 108, 101,
                102, 116, 10, 55, 53, 32, 49, 49, 32, 50, 32, 50, 53, 32, 34, 244, 105, 97, 108,
                108, 34, 32, 34, 34, 32, 52, 32, 52, 32, 34, 114, 101, 100, 34, 10, 56, 48, 32, 50,
                32, 50, 32, 49, 51, 32, 34, 114, 105, 107, 105, 34, 32, 34, 34, 32, 49, 51, 32, 49,
                51, 32, 34, 98, 108, 117, 101, 34, 10, 56, 52, 32, 52, 32, 50, 32, 53, 49, 32, 34,
                78, 76, 34, 32, 34, 34, 32, 52, 32, 52, 32, 34, 114, 101, 100, 34, 10, 55, 56, 32,
                45, 57, 57, 57, 57, 32, 50, 32, 45, 53, 54, 32, 34, 92, 115, 92, 98, 97, 100, 97,
                115, 115, 34, 32, 34, 98, 97, 100, 97, 115, 115, 34, 32, 49, 48, 32, 49, 49, 32,
                34, 109, 97, 122, 34, 10, 55, 57, 32, 45, 57, 57, 57, 57, 32, 50, 32, 45, 51, 56,
                32, 34, 92, 115, 92, 108, 111, 107, 101, 34, 32, 34, 34, 32, 52, 32, 52, 32, 34,
                114, 101, 100, 34, 10, 56, 49, 32, 45, 57, 57, 57, 57, 32, 50, 32, 45, 51, 56, 32,
                34, 92, 115, 92, 81, 117, 97, 107, 101, 34, 32, 34, 34, 32, 49, 51, 32, 49, 51, 32,
                34, 98, 108, 117, 101, 34, 10, 56, 53, 32, 51, 32, 50, 32, 52, 53, 32, 34, 72, 108,
                89, 34, 32, 34, 34, 32, 49, 51, 32, 49, 51, 32, 34, 98, 108, 117, 101, 34, 10, 56,
                54, 32, 45, 57, 57, 57, 57, 32, 50, 32, 45, 54, 54, 54, 32, 34, 92, 115, 92, 91,
                83, 101, 114, 118, 101, 77, 101, 93, 34, 32, 34, 34, 32, 49, 50, 32, 49, 49, 32,
                34, 108, 113, 119, 99, 34, 10, 113, 116, 118, 32, 49, 32, 34, 122, 97, 115, 97,
                100, 122, 107, 97, 32, 81, 116, 118, 32, 40, 50, 41, 34, 32, 34, 50, 64, 122, 97,
                115, 97, 100, 122, 107, 97, 46, 112, 108, 58, 50, 56, 48, 48, 48, 34, 32, 50, 10,
                0,
            ]
            .as_slice();

            let res = Status119Response::parse(bytes)?;

            {
                assert_eq!(
                    res.settings().hostname,
                    Some("zasadzka:27501 (red vs. blue)\u{87}".to_string())
                );

                let qtv_stream = res.qtv_stream.unwrap_or_default();
                assert_eq!(qtv_stream.id(), 1);
                assert_eq!(qtv_stream.name(), "zasadzka Qtv (2)".to_string());
                assert_eq!(qtv_stream.number(), Some(2));
                assert_eq!(qtv_stream.address(), Some("zasadzka.pl:28000"));
                assert_eq!(qtv_stream.client_count(), 2);

                assert_eq!(
                    res.clients,
                    vec![
                        GenericClient {
                            id: 75,
                            frags: 11,
                            ping: 25,
                            time: 2,
                            name: "ôiall".to_string(),
                            team: "red".to_string(),
                            skin: "".to_string(),
                            top_color: 4,
                            bottom_color: 4,
                            is_spectator: false,
                            is_bot: false,
                            auth_cc: "".to_string(),
                        },
                        GenericClient {
                            id: 80,
                            frags: 2,
                            ping: 13,
                            time: 2,
                            name: "riki".to_string(),
                            team: "blue".to_string(),
                            skin: "".to_string(),
                            top_color: 13,
                            bottom_color: 13,
                            is_spectator: false,
                            is_bot: false,
                            auth_cc: "".to_string(),
                        },
                        GenericClient {
                            id: 84,
                            frags: 4,
                            ping: 51,
                            time: 2,
                            name: "NL".to_string(),
                            team: "red".to_string(),
                            skin: "".to_string(),
                            top_color: 4,
                            bottom_color: 4,
                            is_spectator: false,
                            is_bot: false,
                            auth_cc: "".to_string(),
                        },
                        GenericClient {
                            id: 78,
                            frags: 0,
                            ping: 56,
                            time: 2,
                            name: "badass".to_string(),
                            team: "maz".to_string(),
                            skin: "badass".to_string(),
                            top_color: 10,
                            bottom_color: 11,
                            is_spectator: true,
                            is_bot: false,
                            auth_cc: "".to_string(),
                        },
                        GenericClient {
                            id: 79,
                            frags: 0,
                            ping: 38,
                            time: 2,
                            name: "loke".to_string(),
                            team: "red".to_string(),
                            skin: "".to_string(),
                            top_color: 4,
                            bottom_color: 4,
                            is_spectator: true,
                            is_bot: false,
                            auth_cc: "".to_string(),
                        },
                        GenericClient {
                            id: 81,
                            frags: 0,
                            ping: 38,
                            time: 2,
                            name: "Quake".to_string(),
                            team: "blue".to_string(),
                            skin: "".to_string(),
                            top_color: 13,
                            bottom_color: 13,
                            is_spectator: true,
                            is_bot: false,
                            auth_cc: "".to_string(),
                        },
                        GenericClient {
                            id: 85,
                            frags: 3,
                            ping: 45,
                            time: 2,
                            name: "HlY".to_string(),
                            team: "blue".to_string(),
                            skin: "".to_string(),
                            top_color: 13,
                            bottom_color: 13,
                            is_spectator: false,
                            is_bot: false,
                            auth_cc: "".to_string(),
                        },
                        GenericClient {
                            id: 86,
                            frags: 0,
                            ping: 666,
                            time: 2,
                            name: "[ServeMe]".to_string(),
                            team: "lqwc".to_string(),
                            skin: "".to_string(),
                            top_color: 12,
                            bottom_color: 11,
                            is_spectator: true,
                            is_bot: true,
                            auth_cc: "".to_string(),
                        },
                    ]
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_status119qtvstream_parse() -> Result<()> {
        // empty url
        {
            let bytes = br#"qtv 2 "QUAKE.SE KTX Qtv (1)" "" 0"#.as_slice();
            let stream = Status119QtvStream::parse(bytes)?;
            assert_eq!(stream.id(), 2);
            assert_eq!(stream.name(), "QUAKE.SE KTX Qtv (1)");
            assert_eq!(stream.number(), None);
            assert_eq!(stream.address(), None);
            assert_eq!(stream.client_count(), 0);
            assert_eq!(stream.url(), None);
        }

        // valid
        {
            let bytes = br#"qtv 2 "QUAKE.SE KTX Qtv (1)" "1@quake.se:28000" 0"#.as_slice();
            let stream = Status119QtvStream::parse(bytes)?;
            assert_eq!(stream.id(), 2);
            assert_eq!(stream.name(), "QUAKE.SE KTX Qtv (1)");
            assert_eq!(stream.number(), Some(1));
            assert_eq!(stream.address(), Some("quake.se:28000"));
            assert_eq!(stream.client_count(), 0);
            assert_eq!(stream.url(), Some("1@quake.se:28000".to_string()));
        }

        Ok(())
    }
}
