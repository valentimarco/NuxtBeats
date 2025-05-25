use rustypipe::{
    client::{RustyPipe, RustyPipeQuery},
    model::{MusicPlaylist, MusicPlaylistItem},
};
use tokio::fs;

#[derive(Clone)]
pub struct MusicApi {
    client: RustyPipeQuery,
    playlists: Vec<MusicPlaylistItem>,
}

impl MusicApi {
    pub async fn new() -> MusicApi {
        let rp = RustyPipe::builder()
            .storage_dir("$HOME/.config/nuxtbeats/data")
            .build()
            .unwrap();

        rp.user_auth_set_cookie_txt(fs::read_to_string("./cookies.txt").await.unwrap().as_str())
            .await
            .unwrap();

        let client = rp.query().authenticated();
        let playlists = client.music_saved_playlists().await.unwrap().items;

        MusicApi { client, playlists }
    }

    pub async fn get_all_playlists(&self) -> Vec<MusicPlaylistItem> {
        self.playlists.clone()
    }

    pub async fn get_playlist_by_id(&self, id: &str) -> Result<MusicPlaylist, String> {
        let playlist = self.playlists.iter().find(|x| x.id == id);

        match playlist {
            Some(p) => self
                .client
                .music_playlist(p.id.clone())
                .await
                .map_err(|x| x.to_string()),
            None => Err(String::from("Error retriving playlist")),
        }
    }
}
