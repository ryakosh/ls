use {
    ls::{filter_hidden, get_contents, RefContents},
    std::path::{Path, PathBuf},
    structopt::StructOpt,
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
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let opt = Opt::from_args();

    let mut contents = if let Some(file) = &opt.file {
        get_contents(file.as_path()).await?
    } else {
        get_contents(Path::new(".")).await?
    };

    if !opt.all {
        filter_hidden(&mut contents);
    }

    println!("{}", fmt(&contents));

    Ok(())
}

fn fmt(contents: &RefContents) -> String {
    contents
        .iter()
        .map(|c| c.file_name().unwrap().to_str().unwrap())
        .collect::<Vec<_>>()
        .join("  ")
}
