use reqwest::r#async::Client;

use crate::network::{download_file, download_file_stream};
use crate::requests::get_file::GetFile;
use crate::{
    requests::{
        edit_message_live_location::EditMessageLiveLocation,
        forward_message::ForwardMessage, get_me::GetMe, send_audio::SendAudio,
        send_location::SendLocation, send_media_group::SendMediaGroup,
        send_message::SendMessage, send_photo::SendPhoto,
        stop_message_live_location::StopMessageLiveLocation, ChatId,
        RequestContext,
    },
    types::{InputFile, InputMedia},
};
use crate::DownloadError;
use reqwest::r#async::Chunk;
use tokio::io::AsyncWrite;
use tokio::stream::Stream;

pub struct Bot {
    token: String,
    client: Client,
}

impl Bot {
    pub fn new(token: &str) -> Self {
        Bot {
            token: String::from(token),
            client: Client::new(),
        }
    }

    pub fn with_client(token: &str, client: Client) -> Self {
        Bot {
            token: String::from(token),
            client,
        }
    }

    fn ctx(&self) -> RequestContext {
        RequestContext {
            token: &self.token,
            client: &self.client,
        }
    }
}

/// Telegram functions
impl Bot {
    /// Download file from telegram into `destination`.
    /// `path` can be obtained from [`get_file`] method.
    ///
    /// For downloading as Stream of Chunks see [`download_file_stream`].
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use async_telegram_bot::{
    ///     bot::Bot,
    ///     requests::Request,
    ///     types::File as TgFile,
    /// };
    /// use tokio::fs::File;
    /// # use async_telegram_bot::RequestError;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = Bot::new("TOKEN");
    /// let mut file = File::create("/home/waffle/Pictures/test.png").await?;
    ///
    /// let TgFile { file_path, .. } = bot.get_file("*file_id*").send().await?;
    /// bot.download_file(&file_path, &mut file).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`get_file`]: crate::bot::Bot::get_file
    /// [`download_file_stream`]: crate::bot::Bot::download_file_stream
    pub async fn download_file<D>(
        &self,
        path: &str,
        destination: &mut D,
    ) -> Result<(), DownloadError>
    where
        D: AsyncWrite + Unpin,
    {
        download_file(&self.client, &self.token, path, destination).await
    }

    /// Download file from telegram.
    ///
    /// `path` can be obtained from [`get_file`] method.
    ///
    /// For downloading into [`AsyncWrite`] (e.g. [`tokio::fs::File`])
    /// see  [`download_file`].
    ///
    /// [`get_file`]: crate::bot::Bot::get_file
    /// [`AsyncWrite`]: tokio::io::AsyncWrite
    /// [`tokio::fs::File`]: tokio::fs::File
    /// [`download_file`]: crate::bot::Bot::download_file
    pub async fn download_file_stream(
        &self,
        path: &str,
    ) -> Result<impl Stream<Item = Result<Chunk, reqwest::Error>>, reqwest::Error>
    {
        download_file_stream(&self.client, &self.token, path).await
    }

    pub fn get_me(&self) -> GetMe {
        GetMe::new(self.ctx())
    }

    pub fn send_message<C, T>(&self, chat_id: C, text: T) -> SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SendMessage::new(self.ctx(), chat_id.into(), text.into())
    }

    pub fn edit_message_live_location<Lt, Lg>(
        &self,
        latitude: Lt,
        longitude: Lg,
    ) -> EditMessageLiveLocation
    where
        Lt: Into<f64>,
        Lg: Into<f64>,
    {
        EditMessageLiveLocation::new(
            self.ctx(),
            latitude.into(),
            longitude.into(),
        )
    }

    pub fn forward_message<C, F, M>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: M,
    ) -> ForwardMessage
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
        M: Into<i32>,
    {
        ForwardMessage::new(
            self.ctx(),
            chat_id.into(),
            from_chat_id.into(),
            message_id.into(),
        )
    }

    pub fn send_audio<C, A>(&self, chat_id: C, audio: A) -> SendAudio
    where
        C: Into<ChatId>,
        A: Into<InputFile>,
    {
        SendAudio::new(self.ctx(), chat_id.into(), audio.into())
    }

    pub fn send_location<C, Lt, Lg>(
        &self,
        chat_id: C,
        latitude: Lt,
        longitude: Lg,
    ) -> SendLocation
    where
        C: Into<ChatId>,
        Lt: Into<f64>,
        Lg: Into<f64>,
    {
        SendLocation::new(
            self.ctx(),
            chat_id.into(),
            latitude.into(),
            longitude.into(),
        )
    }

    pub fn send_media_group<C, M>(&self, chat_id: C, media: M) -> SendMediaGroup
    where
        C: Into<ChatId>,
        M: Into<Vec<InputMedia>>,
    {
        SendMediaGroup::new(self.ctx(), chat_id.into(), media.into())
    }

    pub fn send_photo<C, P>(&self, chat_id: C, photo: P) -> SendPhoto
    where
        C: Into<ChatId>,
        P: Into<InputFile>,
    {
        SendPhoto::new(self.ctx(), chat_id.into(), photo.into())
    }

    pub fn stop_message_live_location(&self) -> StopMessageLiveLocation {
        StopMessageLiveLocation::new(self.ctx())
    }

    pub fn get_file<F>(&self, file_id: F) -> GetFile
    where
        F: Into<String>,
    {
        GetFile::new(self.ctx(), file_id.into())
    }
}
