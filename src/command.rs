use crate::resp::Resp;
use anyhow::{bail, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Ping,
    Echo(String),
}

impl Command {
    pub fn from_resp(resp: Resp) -> Result<Self> {
        let data = match resp {
            Resp::Array(data) => data,
            _ => bail!("invalid command"),
        };

        let command = get(&data, 0)?;
        let command = command.to_lowercase();
        match command.as_str() {
            "ping" => Ok(Command::Ping),
            "echo" => {
                let param = get(&data, 1)?;
                Ok(Command::Echo(param))
            }
            _ => bail!("command not supported"),
        }
    }
}

fn get(data: &Vec<Resp>, index: usize) -> Result<String> {
    let Some(item) = data.get(index) else {
        bail!("invalid command");
    };

    let val = match item {
        Resp::BulkString(val) => val,
        _ => bail!("invalid command"),
    };
    Ok(String::from_utf8(val.to_vec())?)
}
