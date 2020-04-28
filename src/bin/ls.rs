use {exitfailure::ExitFailure, ls::*, std::path, structopt::StructOpt};

#[derive(StructOpt)]
/// List information about the FILE (the current directory by default)
struct Opt {
    #[structopt(parse(from_os_str))]
    /// The file to list information about
    file: Option<path::PathBuf>,
    #[structopt(short, long)]
    /// do not ignore entries starting with .
    all: bool,
    #[structopt(short)]
    /// use a long listing format
    long: bool,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    let mut files = if let Some(file) = &opt.file {
        Files::new(file.as_path()).await?
    } else {
        Files::new(path::Path::new(".")).await?
    };

    if !opt.all {
        files.filter_hidden();
    }

    if !opt.long {
        println!("{}", files);
    } else {
        println!("{}", files.long_fmt());
    }

    Ok(())
}
