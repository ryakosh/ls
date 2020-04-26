use {
    ls::*,
    std::path::{Path, PathBuf},
    structopt::StructOpt,
    exitfailure::ExitFailure,
};

#[derive(StructOpt)]
/// List information about the FILE (the current directory by default)
struct Opt {
    #[structopt(parse(from_os_str))]
    /// The file to list information about
    file: Option<PathBuf>,
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
        get_files(file.as_path()).await?
    } else {
        get_files(Path::new(".")).await?
    };

    if !opt.all {
        filter_hidden(&mut files);
    }

    if !opt.long {
        println!("{}", fmt(&files));
    } else {
        print!("{}", fmt_l(&files));
    }

    Ok(())
}

fn fmt(files: &RefFiles) -> String {
    files
        .iter()
        .map(File::file_name)
        .collect::<Vec<_>>()
        .join("  ")
}

fn fmt_l(files: &RefFiles) -> String {
    files
        .iter()
        .map(|f| format!("{} \n", f.long_fmt()))
        .collect()
}