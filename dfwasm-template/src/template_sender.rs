use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{self, protocol::Message},
};

use crate::Template;

pub struct DFApiClient {
    state: ReaderWriter,
}

#[derive(Error, Debug)]
pub enum SenderError {
    #[error("error: codeclient rejected auth")]
    RejectedAuth,
    #[error("error: invalid state transition")]
    InvalidStateTransition,
    #[error("error: invalid permissions")]
    InvalidPermissions,
    #[error("error: player not in dev mode")]
    NotInDev,
    #[error("error: plot too small")]
    PlotTooSmall,
    #[error("{0}")]
    Connection(#[from] tungstenite::Error),
}

impl DFApiClient {
    pub async fn connect() -> Result<Self, SenderError> {
        let (socket, _) = connect_async("ws://localhost:31375")
            .await
            .map_err(SenderError::Connection)?;

        let (writer, reader) = socket.split();

        let mut this = Self {
            state: ReaderWriter { reader, writer },
        };

        this.inner_send().await?;
        Ok(this)
    }

    async fn inner_send(&mut self) -> Result<(), SenderError> {
        self.request_scopes(&[
            "inventory",
            "movement",
            "read_plot",
            "write_code",
            "clear_plot",
        ])
        .await
    }

    pub async fn send_templates(
        &mut self,
        templates: &[Template],
        clear: bool,
    ) -> Result<(), SenderError> {
        let mode = self.get_mode().await?;
        if mode != "code" {
            return Err(SenderError::NotInDev);
        }

        if !self.has_scope("write_code").await? {
            return Err(SenderError::InvalidPermissions);
        }

        if clear && self.has_scope("clear_plot").await? {
            self.clear_plot().await?;
        }

        self.write("place swap").await?;

        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // self.write("compact").await?;

        for template in templates {
            let data = format!("place {}", template.encode().expect("invalid template"));
            self.write(data).await?;
        }

        self.write("place go").await?;

        let next = self.next().await?;
        if next != "place done" {
            Err(SenderError::InvalidStateTransition)
        } else {
            Ok(())
        }
    }

    pub async fn get_plot_size(&mut self) -> Result<usize, SenderError> {
        self.write("size").await?;

        let next = self.next().await?;
        match next.as_str() {
            "BASIC" => Ok(51),
            "LARGE" => Ok(101),
            "MASSIVE" => Ok(301),
            "MEGA" => Ok(1001),
            _ => {
                dbg!(next.as_str());
                Err(SenderError::NotInDev)
            }
        }
    }

    pub async fn get_mode(&mut self) -> Result<String, SenderError> {
        self.write("mode").await?;

        self.next().await
    }

    pub async fn clear_plot(&mut self) -> Result<(), SenderError> {
        if !self.has_scope("clear_plot").await? {
            return Err(SenderError::InvalidPermissions);
        }

        self.write("clear").await?;

        Ok(())
    }

    async fn next(&mut self) -> Result<String, SenderError> {
        match self.state.reader.next().await {
            Some(Ok(Message::Ping(bytes))) => {
                self.write(Message::Pong(bytes)).await?;
                Box::pin(self.next()).await
            }
            Some(Ok(received_msg)) => Ok(received_msg.into_text().unwrap().to_string()),
            Some(Err(e)) => Err(SenderError::Connection(e)),
            None => Err(SenderError::Connection(
                tungstenite::Error::ConnectionClosed,
            )),
        }
    }

    pub async fn request_scopes(
        &mut self,
        scopes: &'static [&'static str],
    ) -> Result<(), SenderError> {
        let msg = format!("scopes {}", scopes.join(" "));
        self.write(msg).await?;

        let next = self.next().await?;
        if next != "auth" {
            Err(SenderError::RejectedAuth)
        } else {
            Ok(())
        }
    }

    pub async fn get_scopes(&mut self) -> Result<Vec<String>, SenderError> {
        self.write("scopes").await?;

        let next = self.next().await?;
        Ok(next.split_whitespace().map(|s| s.to_string()).collect())
    }

    pub async fn has_scope(&mut self, scope: &str) -> Result<bool, SenderError> {
        let scopes = self.get_scopes().await?;
        Ok(scopes.contains(&scope.to_string()))
    }

    async fn write(&mut self, msg: impl Into<Message>) -> Result<(), SenderError> {
        let msg = msg.into();
        self.state
            .writer
            .send(msg.clone())
            .await
            .map_err(SenderError::Connection)?;

        self.state
            .writer
            .flush()
            .await
            .map_err(SenderError::Connection)?;

        Ok(())
    }
}

//
// -- util types
//

pub struct ReaderWriter {
    reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}
