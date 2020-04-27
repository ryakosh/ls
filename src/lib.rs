pub mod error;

use {
    error::Error,
    futures::StreamExt,
    std::io::ErrorKind,
    std::path::{Path, PathBuf},
    std::cmp,
    tokio::fs::read_dir,
    std::os::unix::fs::MetadataExt,
    std::fmt,
    std::fs::Metadata,
    chrono::TimeZone
};

pub enum FileType {
    File,
    Dir,
    Sym,
    Unk,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_specifier = match self {
            FileType::File => "-",
            FileType::Dir => "d",
            FileType::Sym => "l",
            FileType::Unk => "?", // Unkown type TODO: Change this
        };

        write!(f, "{}", type_specifier)
    }
}
#[derive(Debug)]
pub struct Files(Vec<File>);

impl Files {
    pub async fn new(path: &Path) -> Result<Self, failure::Error> {
        match read_dir(path).await {
            Ok(mut stream) => {
                let mut files = vec![];
    
                while let Some(dir_entry) = stream.next().await {
                    files.push(File::new(dir_entry?.path())?);
                }
    
                files.sort();
                Ok(Self(files))
            }
            Err(e) => {
                let file = File::new(path.to_owned())?;
                match e.kind() {
                    ErrorKind::NotFound => Err(Error::NF(e, file).into()),
                    ErrorKind::PermissionDenied => Err(Error::PD(e, file).into()),
                    ErrorKind::Other if e.to_string().starts_with("Not a directory") => Ok(Self(vec![file])),
                    _ => Err(e.into()),
                }
            }
        }
    }

    pub fn filter_hidden(&mut self) {
        self.as_vec_mut().retain(|f| !f.is_hidden());
    }

    pub fn as_vec(&self) -> &Vec<File> {
        &self.0
    }

    pub fn as_vec_mut(&mut self) -> &mut Vec<File> {
        &mut self.0
    }

    pub fn long_fmt(&self) -> String {
        self
            .as_vec()
            .iter()
            .map(|f| format!("{} \n", f.long_fmt()))
            .collect::<String>()
            .trim_end()
            .to_string()
    }
}

impl fmt::Display for Files {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let files = self
            .as_vec()
            .iter()
            .map(|f| format!("{} ", f.to_string()))
            .collect::<String>()
            .trim_end()
            .to_string();

        write!(f, "{}", files)
    }
}
#[derive(Debug)]
pub struct File {
    pathbuf: PathBuf,
    metadata: Metadata,
    file_name: String,
}

impl File {
    pub fn new(pathbuf: PathBuf) -> Result<Self, failure::Error> {
        Ok(Self {
            metadata: pathbuf.metadata()?,
            file_name: pathbuf.file_name().unwrap().to_str().unwrap().to_string(),
            pathbuf,
        })
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn file_type(&self) -> FileType {
        let ft = self.metadata.file_type();

        if ft.is_file() {
            FileType::File
        } else if ft.is_dir() {
            FileType::Dir
        } else if ft.is_symlink() {
            FileType::Sym
        } else {
            FileType::Unk
        }
    }

    pub fn permissions(&self) -> umask::Mode {
        umask::Mode::from(self.metadata.mode() & 0b111111111) // Take only bits corresponding to permissions
    }

    pub fn hlink_num(&self) -> u64 {
        self.metadata.nlink()
    }

    pub fn user(&self) -> users::User {
        users::get_user_by_uid(self.metadata.uid()).unwrap() // TODO: Better error handling
    }

    pub fn group(&self) -> users::Group {
        users::get_group_by_gid(self.metadata.gid()).unwrap() // TODO: Better error handling
    }

    pub fn size(&self) -> u64 {
        self.metadata.size()
    }

    pub fn modified(&self) -> chrono::DateTime<chrono::Local> {
        chrono::Local.timestamp(self.metadata.mtime(), 0)
    }

    pub fn pathbuf(&self) -> &PathBuf {
        &self.pathbuf
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn long_fmt(&self) -> String {
        format!("{}{} {} {} {} {} {} {}",
            self.file_type(),
            self.permissions(),
            self.hlink_num(),
            self.user().name().to_str().unwrap(),
            self.group().name().to_str().unwrap(),
            self.size().to_string(),
            self.modified().format("%b %e %H:%M"),
            self.file_name(),
        )
    }

    fn is_hidden(&self) -> bool {
        self.file_name().starts_with('.')
    }
}

impl cmp::PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.pathbuf() == other.pathbuf()
    }
}
impl cmp::Eq for File {}

impl cmp::PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for File {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.pathbuf().cmp(other.pathbuf())
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.file_name())
    }
}