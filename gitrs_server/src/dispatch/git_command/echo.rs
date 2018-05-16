use futures::{future, Future};
use message::protocol::git_command::echo as shared_echo_message;
use state;
use std::process::Command;
use std::str;
use tokio_process::CommandExt;
use types::DispatchFuture;
use util::transport::send_message;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Outbound {
    Result { output: String },
}

pub fn dispatch(
    connection_state: state::Connection,
    message: shared_echo_message::Inbound,
) -> DispatchFuture {
    use error::protocol::{Error, ProcessError::{Encoding, Failed}};

    Box::new(
        Command::new("echo")
            .arg(&message.input)
            .output_async()
            .map_err(|_| Error::Process(Failed))
            .and_then(|output| match str::from_utf8(&output.stdout) {
                Ok(output) => future::ok(String::from(output)),
                Err(_) => future::err(Error::Process(Encoding)),
            })
            .and_then(|output| send_message(connection_state, Outbound::Result { output })),
    )
}