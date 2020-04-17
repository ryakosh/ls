pub mod error;

use {
    std::path::{Path, PathBuf},
    std::io::ErrorKind,

    tokio::fs::read_dir,
    futures::StreamExt,

    error::Error,
};

pub type Contents = Vec<PathBuf>;

pub async fn get_contents(path: &Path, all: bool) -> Result<Contents, failure::Error> {
    match read_dir(path).await {
        Ok(mut stream) => {
            let mut contents = vec![];

            if all {
                while let Some(dir_entry) = stream.next().await {
                    let dir_entry = dir_entry?;

                    contents.push(dir_entry.path());
                }
            } else {
                while let Some(dir_entry) = stream.next().await {
                    let dir_entry = dir_entry?;

                    if !dir_entry.file_name().to_str().unwrap().starts_with('.') {
                        contents.push(dir_entry.path());
                    }
                }
            }

            Ok(contents)

        }
        Err(e) => {
            let path = path.to_owned();
            match e.kind() {
                ErrorKind::NotFound => Err(Error::NF(e, path).into()),
                ErrorKind::PermissionDenied => Err(Error::PD(e, path).into()),
                ErrorKind::Other if e.to_string().starts_with("Not a directory") => {
                    Ok(vec![path])
                }
                _ => Err(e.into())
            }
        }
    }
}