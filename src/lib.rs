pub mod error;

use {
    error::Error,
    futures::StreamExt,
    std::io::ErrorKind,
    std::path::{Path, PathBuf},
    tokio::fs::read_dir,
};

pub type Contents = Vec<PathBuf>;

pub async fn get_contents(path: &Path) -> Result<Contents, failure::Error> {
    match read_dir(path).await {
        Ok(mut stream) => {
            let mut contents = vec![];

            while let Some(dir_entry) = stream.next().await {
                contents.push(dir_entry?.path());
            }

            contents.sort();
            Ok(contents)
        }
        Err(e) => {
            let path = path.to_owned();
            match e.kind() {
                ErrorKind::NotFound => Err(Error::NF(e, path).into()),
                ErrorKind::PermissionDenied => Err(Error::PD(e, path).into()),
                ErrorKind::Other if e.to_string().starts_with("Not a directory") => Ok(vec![path]),
                _ => Err(e.into()),
            }
        }
    }
}

pub fn filter_hidden(contents: &mut Contents) {
    contents.retain(is_not_hidden);
}

fn is_not_hidden(p: &PathBuf) -> bool {
    !p.file_name().unwrap().to_str().unwrap().starts_with(".")
}
