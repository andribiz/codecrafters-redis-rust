use std::io::Cursor;

use anyhow::{bail, Result};
use bytes::Buf;

#[derive(Debug, PartialEq, Eq)]
pub enum Resp {
    String(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<Self>),
    NullBulk,
    NullAray,
}

// TODO: Change to bit operation instead string
impl ToString for Resp {
    fn to_string(&self) -> String {
        match self {
            Resp::String(data) => format!("+{}\r\n", data),
            Resp::Error(data) => format!("-{}\r\n", data),
            Resp::Integer(data) => format!(":{}\r\n", data),
            Resp::BulkString(data) => {
                let len = data.len();
                let words = String::from_utf8(data.to_vec()).unwrap();
                format!("${}\r\n{}\r\n", len, words)
            }
            Resp::Array(data) => {
                todo!();
            }
            Resp::NullAray => String::from("_\r\n"),
            Resp::NullBulk => String::from("$-1\r\n"),
        }
    }
}

// impl ToString<Resp> for String {
//     type Error = &'static str;
//
//     fn try_from(value: Resp) -> Result<Self, &'static str> {
//         match value {
//             Resp::String(data) => Ok(format!("+{}\r\n", data)),
//             Resp::Error(data) => Ok(format!("-{}\r\n", data)),
//             Resp::Integer(data) => Ok(format!(":{}\r\n", data)),
//         }
//     }
// }

impl Resp {
    pub fn decode(src: &mut Cursor<&[u8]>) -> Result<Self> {
        match get_u8(src)? {
            b'+' => {
                let str = get_line(src)?.to_vec();
                let res = String::from_utf8(str)?;
                Ok(Resp::String(res))
            }
            b'-' => {
                let str = get_line(src)?.to_vec();
                let res = String::from_utf8(str)?;
                Ok(Resp::Error(res))
            }
            b':' => {
                let res = get_decimal(src)?;
                Ok(Resp::Integer(res))
            }
            b'$' => {
                let len = get_decimal(src)?;
                if len == -1 {
                    return Ok(Resp::NullBulk);
                } else if len == 0 {
                    return Ok(Resp::BulkString(vec![]));
                }
                let line = get_line(src)?;
                Ok(Resp::BulkString(line[..len as usize].to_vec()))
            }
            b'*' => {
                let len = get_decimal(src)?;
                if len == -1 {
                    Ok(Resp::NullAray)
                } else if len == 0 {
                    Ok(Resp::BulkString(vec![]))
                } else {
                    let mut res = vec![];
                    for _ in 0..len {
                        let data = Self::decode(src)?;
                        res.push(data);
                    }
                    Ok(Resp::Array(res))
                }
            }
            _ => bail!("unknown format"),
        }
    }
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8> {
    if !src.has_remaining() {
        bail!("error incomplete");
    }
    Ok(src.get_u8())
}

fn get_decimal(src: &mut Cursor<&[u8]>) -> Result<i64> {
    let line = get_line(src)?.to_vec();
    let str = String::from_utf8(line)?;
    let res = str.parse::<i64>()?;
    Ok(res)
}

fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8]> {
    let start = src.position() as usize;
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            src.set_position((i + 2) as u64);
            return Ok(&src.get_ref()[start..i]);
        }
    }
    bail!("error incomplete")
}
