use std::path::Path;

use rustypipe::{client::RustyPipe, model::MusicPlaylistItem};

use crate::{
    error::{Error, Result},
    types::Playlist,
};

pub struct YoutubeMusicApi {
    client: RustyPipe,
}

impl YoutubeMusicApi {
    pub async fn new(cookies: String, config_dir: &Path) -> crate::Result<YoutubeMusicApi> {
        let client = RustyPipe::builder().storage_dir(config_dir).build()?;

        client.user_auth_set_cookie_txt(&cookies).await?;
        client.user_auth_check_cookie().await?;
        Ok(YoutubeMusicApi { client })
    }

    pub async fn logout(&self) -> bool {
        self.client.user_auth_remove_cookie().await.is_ok()
    }

    pub async fn get_all_playlists(&self) -> Result<Vec<Playlist>> {
        let query = self.client.query().authenticated();
        let mut paginator_playlists = query.music_saved_playlists().await?;
        let _ = paginator_playlists.extend_all(query).await?;
        Ok(paginator_playlists
            .items
            .iter()
            .map(|x| Playlist {
                id: x.id.clone(),
                name: x.name.clone(),
                tracks: x.track_count.unwrap_or(0),
                cover: x.thumbnail.iter().map(|i| i.url.clone()).collect(),
            })
            .collect())
    }

    // pub async fn play_playlist(&self) -> Result<()> {}
}
