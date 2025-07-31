use std::{fs, path::Path};

const IMAGE_PATH: &str = "./track_images/";

pub async fn download_track_image(image_url: &str, spoti_id: &str) -> String {
        let path = Path::new(IMAGE_PATH);

        if let Some(parent) = path.parent() {
                fs::create_dir_all(parent);
        }



        "hi".to_string()
}
