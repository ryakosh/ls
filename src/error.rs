use {failure::Fail, std::io, crate::File};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "ls: cannot access '{:?}': No such file or directory", _1)]
    NF(#[fail(cause)] io::Error, File),
    #[fail(display = "ls: cannot open directory '{:?}': Permission denied", _1)]
    PD(#[fail(cause)] io::Error, File),
}
