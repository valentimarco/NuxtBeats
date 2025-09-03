use std::path::Path;

use rustypipe::{client::RustyPipe, model::MusicPlaylistItem};
use time::Duration;

use crate::{
    error::{Error, Result},
    types::{Album, Artist, Playlist, Song},
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
            .into_iter()
            .map(|x| Playlist {
                id: x.id,
                name: x.name,
                tracks: x.track_count.unwrap_or(0),
                cover: x.thumbnail.into_iter().map(|i| i.url).collect(),
            })
            .collect())
    }

    pub async fn get_songs_from_playlist_music(
        &self,
        id: String,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Vec<Song>> {
        let query = self.client.query().authenticated();
        let mut paginator = query.music_playlist(id).await?;
        match offset {
            Some(of) => {
                let _ = paginator.tracks.extend_pages(query, of);
            }
            None => {
                let _ = paginator.tracks.extend_all(query);
            }
        };

        Ok(paginator
            .tracks
            .items
            .into_iter()
            .map(|x| Song {
                name: x.name,
                id: x.id,
                artists: x
                    .artists
                    .into_iter()
                    .map(|y| Artist {
                        name: y.name,
                        id: y.id.unwrap_or(String::from("None???")),
                    })
                    .collect(),
                album: x
                    .album
                    .map(|y| Album {
                        name: y.name,
                        id: y.id,
                    })
                    .unwrap_or(Album::default()),
                time: Duration::seconds(x.duration.unwrap() as i64),
            })
            .collect())
    }

    pub async fn get_songs_from_playlist_yt(
        &self,
        id: String,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Vec<Song>> {
        let query = self.client.query().authenticated();
        let playlist = query.playlist(id).await?;
        let mut paginator = playlist.videos;
        match offset {
            Some(of) => {
                let _ = paginator.extend_pages(query, of);
            }
            None => {
                let _ = paginator.extend_all(query);
            }
        };

        Ok(paginator
            .items
            .into_iter()
            .map(|x| Song {
                id: x.id,
                name: x.name,
                artists: x
                    .channel
                    .map(|y| {
                        vec![Artist {
                            name: y.name,
                            id: y.id,
                        }]
                    })
                    .unwrap(),
                album: Album {
                    name: String::new(),
                    id: String::new(),
                },
                time: Duration::seconds(x.duration.unwrap() as i64),
            })
            .collect())
    }
    // pub async fn play_playlist(&self) -> Result<()> {}
}
