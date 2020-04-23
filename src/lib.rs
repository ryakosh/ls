pub mod error;

use {
    error::Error,
    futures::StreamExt,
    std::io::ErrorKind,
    std::path::{Path, PathBuf},
    tokio::fs::read_dir,
    std::os::unix::fs::MetadataExt,
    std::fmt,
    std::fs::Metadata,
    users::{User, Group, get_user_by_uid, get_group_by_gid},
    chrono::{Local, DateTime, TimeZone},
};

pub type Contents = Vec<PathBuf>;
pub type RefContents = [PathBuf];

pub enum FileType {
    File,
    Dir,
    Sym,
    Unk,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FileType::*;

        let mut type_specifier = match self {
            File => "-",
            Dir => "d",
            Sym => "l",
            Unk => "?", // Unkown type TODO: Change this
        };

        write!(f, "{}", type_specifier)
    }
}

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
    !p.file_name().unwrap().to_str().unwrap().starts_with('.')
}

pub fn get_type(meta: &Metadata) -> FileType {
    use FileType::*;

    let file_type = meta.file_type();
    if file_type.is_file() {
        File
    } else if file_type.is_dir() {
        Dir
    } else if file_type.is_symlink() {
        Sym
    } else {
        Unk
    }
}

pub fn get_permissions(meta: &Metadata) -> umask::Mode {
    umask::Mode::from(meta.mode() & 0b111111111) // Take only bits corresponding to permissions
}

pub fn get_hlink_num(meta: &Metadata) -> u64 {
    meta.nlink()
}

pub fn get_user(meta: &Metadata) -> User {
    get_user_by_uid(meta.uid()).unwrap() // TODO: Better error handling
}

pub fn get_group(meta: &Metadata) -> Group {
    get_group_by_gid(meta.gid()).unwrap() // TODO: Better error handling
}

pub fn get_size(meta: &Metadata) -> u64 {
    meta.size()
}

pub fn get_modified(meta: &Metadata) -> DateTime<Local> {
    Local.timestamp(meta.mtime(), 0)
}

pub fn get_long(meta: &Metadata) -> String {
    format!("{}{} {} {} {} {} {}",
        get_type(meta),
        get_permissions(meta),
        get_hlink_num(meta),
        get_user(meta).name().to_str().unwrap(),
        get_group(meta).name().to_str().unwrap(),
        get_size(meta).to_string(),
        get_modified(meta).format("%b %e %H:%M"),
    )
}