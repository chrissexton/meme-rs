use std::{
    env,
    error::Error,
    fs::File,
    io,
    path::{Path, PathBuf},
};

use clap::{App, Arg};
use reqwest::Url;

fn main() {
    let matches = new_app().get_matches();

    let arg_input_file = matches
        .value_of("INPUT")
        .expect("failed decoding arguments");
    let arg_output_file = matches
        .value_of("OUTPUT")
        .expect("failed decoding arguments");
    let arg_text = matches.value_of("TEXT").expect("failed decoding arguments");

    let mut input_file = PathBuf::from(arg_input_file);
    if let Ok(url) = Url::parse(arg_input_file) {
        input_file = download(url).expect("failed downloading url");
    }

    meme_rs::write_meme(&input_file, &PathBuf::from(arg_output_file), arg_text)
}

fn download(url: Url) -> Result<PathBuf, Box<dyn Error>> {
    let filename = Path::new(url.path())
        .file_name()
        .expect("failed getting filename from url")
        .to_str()
        .expect("failed converting filename");

    let mut file_path = env::temp_dir();
    file_path.push(filename);

    let resp = reqwest::blocking::get(url)?;
    let bytes = resp.bytes().expect("failed getting bytes");

    let mut file = File::create(&file_path).expect("failed to create file");
    io::copy(&mut bytes.as_ref(), &mut file)?;
    Ok(file_path)
}

fn new_app<'a>() -> App<'a> {
    App::new("Meme Generator")
        .version("1.0")
        .author("Nisheeth Barthwal <nbaztec@gmail.com>")
        .about("Generates memes")
        .arg(
            Arg::new("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("OUTPUT")
                .help("Sets the output file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("TEXT")
                .help("Sets the text to use. Lines separated by '|' character")
                .required(true)
                .index(3),
        )
}
