use anyhow::{bail, Result as AnyResult};
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use tokio_native_tls::{
    native_tls::TlsConnector, TlsConnector as AsyncTlsConnector, TlsStream as AsyncTlsStream,
};

use crate::types::State;

pub struct Session<S> {
    cmd_id: u64,
    state: State,
    stream: S,
}

pub struct SessionBuilder;

impl SessionBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn connect(
        self,
        addr: (&str, u16),
    ) -> AnyResult<(Session<BufReader<AsyncTlsStream<TcpStream>>>, String)> {
        let connector: AsyncTlsConnector = TlsConnector::builder().build()?.into();
        let stream = TcpStream::connect(addr).await?;
        let mut stream = BufReader::new(connector.connect(addr.0, stream).await?);

        let mut res = String::new();
        stream.read_line(&mut res).await?;

        Ok((
            Session {
                cmd_id: 0,
                state: State::NotAuthenticated,
                stream,
            },
            res,
        ))
    }
}

impl Default for SessionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: AsyncBufRead + AsyncWrite + Unpin> Session<S> {
    pub fn builder() -> SessionBuilder {
        SessionBuilder::new()
    }

    pub async fn send_command(&mut self, command: impl AsRef<[u8]>) -> AnyResult<String> {
        // Increase the command id for each command
        self.cmd_id += 1;

        // Writes the command id formatted as "C<cmd_id>" and terminates with a CRLF
        self.stream.write_all(b"C").await?;
        self.stream
            .write_all(self.cmd_id.to_string().as_bytes())
            .await?;
        self.stream.write_all(b" ").await?;
        self.stream.write_all(command.as_ref()).await?;
        self.stream.write_all(b"\r\n").await?;

        // I am not 100% sure what this does but sometimes the command doesn't work without this
        self.stream.flush().await?;

        // Write the response back
        let mut buf = String::new();
        self.stream.read_line(&mut buf).await?;

        Ok(buf)
    }

    pub async fn login(&mut self, email: &str, password: &str) -> AnyResult<String> {
        match self.state {
            State::NotAuthenticated => {
                let buf = self
                    .send_command(format!("LOGIN {} {}", email, password))
                    .await?;

                // self.state = State::Authenticated;

                Ok(buf)
            }
            _ => bail!("Invalid state"),
        }
    }

    pub async fn logout(&mut self) -> AnyResult<String> {
        self.send_command("LOGOUT".as_bytes()).await
    }

    pub async fn noop(&mut self) -> AnyResult<String> {
        self.send_command("NOOP".as_bytes()).await
    }

    pub async fn select(&mut self, name: &str) -> AnyResult<String> {
        self.send_command(format!("SELECT {}", name)).await
    }
}
