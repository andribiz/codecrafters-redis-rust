use crate::{db::ArcDB, resp::Resp};
use anyhow::{bail, Result};
use bytes::Bytes;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Ping,
    Echo(String),
    Set(String, Bytes),
    Get(String),
}

impl Command {
    pub fn from_resp(resp: Resp) -> Result<Self> {
        let data = match resp {
            Resp::Array(data) => data,
            _ => bail!("invalid command"),
        };

        let command = get(&data, 0)?;
        let command = String::from_utf8(command)?;
        let command = command.to_lowercase();
        match command.as_str() {
            "ping" => Ok(Command::Ping),
            "echo" => {
                let param = get(&data, 1)?;
                let param = String::from_utf8(param)?;
                Ok(Command::Echo(param))
            }
            "set" => {
                let key = get(&data, 1)?;
                let key = String::from_utf8(key)?;
                let value = get(&data, 2)?;
                let value = Bytes::from(value);
                Ok(Command::Set(key, value))
            }
            "get" => {
                let key = get(&data, 1)?;
                let key = String::from_utf8(key)?;
                Ok(Command::Get(key))
            }
            _ => bail!("command not supported"),
        }
    }

    pub async fn execute(self, db: &ArcDB) -> Resp {
        match self {
            Command::Ping => Resp::String(String::from("PONG")),
            Command::Echo(param) => Resp::String(param.to_string()),
            Command::Set(key, value) => {
                db.set(key, value).await;
                Resp::String(String::from("OK"))
            }
            Command::Get(key) => match db.get(key).await {
                Ok(val) => Resp::BulkString(val.to_vec()),
                Err(e) => Resp::Error(e.to_string()),
            },
        }
    }
}

fn get(data: &[Resp], index: usize) -> Result<Vec<u8>> {
    let Some(item) = data.get(index) else {
        bail!("invalid command");
    };

    let val = match item {
        Resp::BulkString(val) => val,
        _ => bail!("invalid command"),
    };
    Ok(val.to_vec())
}
