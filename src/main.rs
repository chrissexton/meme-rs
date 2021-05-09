use std::{env, error::Error, path::Path, process::exit};

use meme_rs::config::{Config, ImageConfig};
use rand::{prelude::SliceRandom, Rng};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let text = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        "LGTM".to_string()
    };

    let cfg = Config::load();

    let mut rng = rand::thread_rng();
    let meme = select_meme(&mut rng, &cfg);

    // let images = meme_rs::read_images(&cfg.input_dir);
    // if images.is_empty() {
    //     eprintln!("no images were found");
    //     exit(1);
    // }

    // println!("found {} images: {:?}", images.len(), images);
    
    let filename = images
        .choose(&mut rng)
        .expect("failed choosing random image");

    let id = meme_rs::new_id(filename, &text);

    let input_file = Path::new(&cfg.input_dir).join(filename);
    let mut id_filename = Path::new(&id).to_path_buf();
    id_filename.set_extension(
        input_file
            .extension()
            .expect("failed retrieving input file extension"),
    );
    let output_file = Path::new(&cfg.output_dir).join(id_filename);

    if output_file.exists() {
        println!("skip generating: {:?}", output_file);
    } else {
        meme_rs::generate(&input_file, &output_file, &text);
        println!("wrote: {:?}", output_file);
    }

    Ok(())
}

fn select_meme<R>(rng: &mut R, cfg: &Config) -> Meme
where
    R: Rng,
{
    let img_cfg = cfg.images.choose(rng).expect("failed choosing image");
    if img_cfg.raw {
        Meme {
            filename: img_cfg.name.clone(),
            raw: true,
            text: "".to_string(),
        }
    } else {
        let mut texts = cfg.common_texts.clone();
        let mut img_texts = img_cfg.texts.clone();
        texts.append(&mut img_texts);
        let img_text = texts.choose(rng).expect("failed choosing text");

        Meme {
            filename: img_cfg.name.clone(),
            raw: true,
            text: img_text.clone(),
        }
    }
}

struct Meme {
    filename: String,
    raw: bool,
    text: String,
}
