use anyhow::{anyhow, Result};
use std::{
        fs::{self, File},
        io::{copy, BufWriter},
        path::Path,
};

use reqwest::Client;

use crate::music_server::token_manager::get_valid_token;

const IMAGE_PATH: &str = "./track_images/";

pub async fn download_track_image(image_url: &str, spoti_id: &str) -> Result<String, anyhow::Error> {
        let path = Path::new(IMAGE_PATH);

        fs::create_dir_all(path)?;

        let token = get_valid_token();
        let mut auth_type = String::new();
        let mut auth_id = String::new();

        if let Ok(valid_token) = token.await {
                auth_type = valid_token.token_type;
                auth_id = valid_token.access_token;
        }

        let client = Client::new();
        let result = client
                .get(image_url)
                .header("Authorization", format!("{} {}", auth_type, auth_id))
                .send()
                .await;

        if let Ok(response) = result {
                let mut file =
                        BufWriter::new(File::create(path.join(format!("{spoti_id}.jpg")))?);
                let content = response.bytes().await?;
                copy(&mut content.as_ref(), &mut file)?;

                return Ok(path.to_string_lossy().to_string());
        }
        Err(anyhow!("Failed downloading track_image"))
}
