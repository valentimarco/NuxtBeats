use std::sync::Arc;

use circular_buffer::CircularBuffer;
use rustypipe::model::{ArtistItem, MusicPlaylist};
use tokio::sync::RwLock;

use crate::{
    error::{Error, Result},
    handler::YtHandler,
};

#[derive(Clone)]
pub struct MusicPlayer {
    buf: Arc<RwLock<Box<CircularBuffer<3, Vec<u8>>>>>,
    handler: YtHandler,
}

impl MusicPlayer {
    fn new(yt_handler: YtHandler) -> MusicPlayer {
        MusicPlayer {
            buf: Arc::new(RwLock::new(CircularBuffer::<3, Vec<u8>>::boxed())),
            handler: yt_handler,
        }
    }

    async fn push_song(&self, song: Vec<u8>) {
        self.buf.write().await.push_back(song);
    }

    async fn get_song(&self) -> Vec<u8> {
        self.buf
            .read()
            .await
            .front()
            .expect("the buffer goes brrr")
            .to_vec()
    }

    pub async fn play_playlist(p: &MusicPlaylist) {}

    //TODO: check correct obj to pass
    pub async fn play_artist(a: &ArtistItem) {}

    pub async fn play_music(&self, id: &str) -> Result<Vec<u8>> {
        let url = String::from("https://www.youtube.com/watch?v=") + id;
        let output_handler = self.handler.execute(url).await?;

        // Command can exit with 0 code but with some errors...
        if !output_handler.stderr.is_empty() {
            return Err(Error::Command(output_handler.stderr));
        }

        return Ok(output_handler.stdout);
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, time::Duration};

    use super::*;

    #[tokio::test]
    async fn test_add_and_get_data() {
        let player = MusicPlayer::new(YtHandler::new(
            PathBuf::from("./yt-dlp"),
            Duration::from_secs(60),
        ));
        let test_data = vec![1, 2, 3, 4];

        // Write data
        player.push_song(test_data.clone()).await;

        // Read data
        let result = player.get_song().await;
        assert_eq!(result, test_data);
    }

    #[tokio::test]
    async fn test_play_music() -> Result<()> {
        let player = MusicPlayer::new(YtHandler::new(
            PathBuf::from("./yt-dlp"),
            Duration::from_secs(60),
        ));

        // Awake Hoshimachi Suisei
        player.play_music("fUbMYZobOV0").await?;

        return Ok(());
    }
}
