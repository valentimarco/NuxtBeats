
use std::path::Path;

use rustypipe::client::RustyPipe;

pub struct YoutubeMusicApi {
    client: RustyPipe,
}

impl YoutubeMusicApi {
    pub async fn new(cookies: String, config_dir: &Path) -> crate::Result<YoutubeMusicApi> {
        let client = RustyPipe::builder()
        .storage_dir(config_dir)
        .build()?;

        client.user_auth_set_cookie_txt(&cookies).await?;
        client.user_auth_check_cookie().await?;
        Ok(YoutubeMusicApi { client })
    }

    pub async fn logout(&self) -> bool {
        self.client.user_auth_remove_cookie().await.is_ok()
    }
}
