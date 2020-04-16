use {
    std::io,
    std::path::PathBuf,

    failure::Fail
};


#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "ls: cannot access '{:?}': No such file or directory", _1)]
    NF(#[fail(cause)] io::Error, PathBuf),
    #[fail(display = "ls: cannot open directory '{:?}': Permission denied", _1)]
    PD(#[fail(cause)] io::Error, PathBuf),
    #[fail(display = "error: an io error has accoured '{:?}':\n\n{:?}", _1, _0)]
    IO(#[fail(cause)] io::Error, PathBuf),
}