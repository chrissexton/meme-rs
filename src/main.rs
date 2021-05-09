use std::{
    error::Error,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use meme_rs::config::Config;
use rand::{prelude::SliceRandom, Rng};
use warp::{http::HeaderValue, hyper::header::CONTENT_TYPE, reply::Response, Filter};

#[derive(Clone)]
struct Message {
    cfg: Config,
    uniform_dist: Vec<(usize, usize)>,
}

impl warp::Reply for Message {
    fn into_response(self) -> warp::reply::Response {
        println!("{:?}", self.uniform_dist);
        let mut rng = rand::thread_rng();
        let meme = gen(&mut rng, &self.cfg, &self.uniform_dist).expect("failed generating meme");
        let mut f = File::open(meme).unwrap();
        let mut buf = vec![];
        f.read_to_end(&mut buf).expect("failed reading file");
        let mut res = Response::new(buf.into());
        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("image/jpeg"));
        res
    }
}

#[tokio::main]
async fn main() {
    let cfg = Config::load();
    let mut uniform_dist = vec![];
    for (idx, c) in cfg.images.iter().enumerate() {
        if c.raw {
            uniform_dist.push((idx, 0));
        } else {
            for t_idx in 0..cfg.common_texts.len() {
                uniform_dist.push((idx, t_idx));
            }
            for t_idx in 0..c.texts.len() {
                uniform_dist.push((idx, cfg.common_texts.len() + t_idx));
            }
        }
    }

    let msg = Message { cfg, uniform_dist };

    let routes = warp::path::end().and(warp::get()).map(move || msg.clone());

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

fn gen<R>(
    rng: &mut R,
    cfg: &Config,
    uniform_dist: &Vec<(usize, usize)>,
) -> Result<PathBuf, Box<dyn Error + 'static>>
where
    R: Rng,
{
    let meme = select_meme(rng, &cfg, &uniform_dist);
    println!("selected meme: {:?}", meme);

    let id = meme_rs::new_id(&meme.filename, &meme.text);

    let input_file = Path::new(&cfg.input_dir).join(&meme.filename);
    let mut id_filename = Path::new(&id).to_path_buf();
    id_filename.set_extension(
        input_file
            .extension()
            .expect("failed retrieving input file extension"),
    );
    let output_file = Path::new(&cfg.output_dir).join(id_filename);

    println!("{:?}", input_file);
    if output_file.exists() {
        println!("skip generating: {:?}", output_file);
    } else {
        meme_rs::generate(&input_file, &output_file, &meme.text);
        println!("wrote: {:?}", output_file);
    }

    Ok(output_file)
}

fn select_meme<R>(rng: &mut R, cfg: &Config, uniform_dist: &Vec<(usize, usize)>) -> Meme
where
    R: Rng,
{
    let &(img_idx, text_idx) = uniform_dist
        .choose(rng)
        .expect("failed choosing random meme");

    println!("({},{})", img_idx, text_idx);
    let img_cfg = &cfg.images[img_idx];
    if img_cfg.raw {
        Meme {
            filename: img_cfg.name.clone(),
            text: "".to_string(),
        }
    } else {
        let mut texts = cfg.common_texts.clone();
        let mut img_texts = img_cfg.texts.clone();
        texts.append(&mut img_texts);
        let img_text = &texts[text_idx];

        Meme {
            filename: img_cfg.name.clone(),
            text: img_text.clone(),
        }
    }
}

#[derive(Debug)]
struct Meme {
    filename: String,
    text: String,
}
