#[derive(Debug)]
pub(crate) enum DownloaderErrors {
        PlaceHolder,
        FailedDownloadingTrack(Option<i32>),
}

