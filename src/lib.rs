pub mod error;

use {
    std::path::{Path, PathBuf},
    std::io::ErrorKind,

    tokio::fs::read_dir,
    futures::StreamExt,

    error::Error,
};

pub async fn get_contents(path: &Path) -> Result<Vec<PathBuf>, Error> {
    match read_dir(path).await {
        Ok(mut stream) => {
            let mut contents = vec![];
            while let Some(dir_entry) = stream.next().await {
                let dir_entry = dir_entry.map_err(|e| {
                    Error::IO(e, path.to_owned())
                });
                contents.push(dir_entry?.path());
            }

            Ok(contents)

        }
        Err(e) => {
            let path = path.to_owned();
            match e.kind() {
                ErrorKind::NotFound => Err(Error::NF(e, path)),
                ErrorKind::PermissionDenied => Err(Error::PD(e, path)),
                ErrorKind::Other if e.to_string().starts_with("Not a directory") => {
                    Ok(vec![path])
                }
                _ => Err(Error::IO(e, path))
            }
        }
    }
}