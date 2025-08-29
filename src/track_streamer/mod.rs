use axum_range::{KnownSize, Ranged};
use tokio::fs::File;

pub(crate) async fn stream(path: &String, range: Option<axum_extra::headers::Range>) -> Ranged<KnownSize<File>> {
        let file = File::open(&path).await.expect("file not found");
        let metadata = file.metadata().await.unwrap();
        let file_size = metadata.len();

        let body = KnownSize::file(file).await.expect("Could not create body");
        Ranged::new(range, body)
}
