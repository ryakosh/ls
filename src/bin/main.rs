use {
    std::path::{Path, PathBuf},

    structopt::StructOpt,

    ls::{Contents, get_contents},
};

#[derive(StructOpt)]
/// List information about the FILE (the current directory by default)
struct Opt {
    #[structopt(parse(from_os_str))]
    /// The file to list information about
    file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    
    let contents = if let Some(file) = &opt.file {
        get_contents(file.as_path()).await?
    } else {
        get_contents(Path::new(".")).await?
    };

    println!("{}", fmt(&contents));


    Ok(())
}

fn fmt(contents: &Contents) -> String {
    let mut filenames = contents.iter()
        .map(|c| format!("{}", c.file_name().unwrap().to_str().unwrap()))
        .collect::<Vec<_>>();
    
    filenames.sort();
    filenames.join("  ")
}