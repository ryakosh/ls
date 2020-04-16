pub mod error;

use {
    std::path::{Path, PathBuf},
    std::io::ErrorKind,

    tokio::fs::read_dir,
    futures::StreamExt,

    error::Error,
};

pub async fn get_contents(path: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut stream = read_dir(path).await.map_err(|e| {
        match e.kind() {
            ErrorKind::NotFound => Error::NF(e, path.to_owned()),
            ErrorKind::PermissionDenied => Error::PD(e, path.to_owned()),
            _ => Error::IO(e, path.to_owned()),
        }
    })?;

    let mut contents = vec![];
    while let Some(dir_entry) = stream.next().await {
        let dir_entry = dir_entry.map_err(|e| {
            Error::IO(e, path.to_owned())
        });
        contents.push(dir_entry?.path());
    }

    Ok(contents)
}