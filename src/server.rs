use std::{error::Error, fs::{self, File}, io::Read, path::{Path, PathBuf}};

use rand::{prelude::SliceRandom, Rng};
use warp::{http::HeaderValue, hyper::header::CONTENT_TYPE, reply::Response, Filter};

use crate::config::{Config, MemeIndex};

pub async fn start() {
    let cfg = Config::load();
    fs::create_dir_all(&cfg.output_dir).expect("faied creating output directory");
    let uniform_dist = cfg.create_uniform_distribution();
    let meme_msg = HttpMemeMessage::new(cfg, uniform_dist);
    let routes = warp::path::end()
        .and(warp::get())
        .map(move || meme_msg.clone());

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

#[derive(Clone)]
struct HttpMemeMessage {
    cfg: Config,
    uniform_dist: Vec<MemeIndex>,
}

impl HttpMemeMessage {
    pub fn new(cfg: Config, uniform_dist: Vec<MemeIndex>) -> Self {
        HttpMemeMessage { cfg, uniform_dist }
    }

    fn new_meme_file(&self) -> PathBuf {
        println!("{:?}", self.uniform_dist);
        let mut rng = rand::thread_rng();
        let meme = Meme::new_random(&mut rng, &self.cfg, &self.uniform_dist);
        println!("selected meme: {:?}", meme);
        meme.write(&self.cfg).expect("failed writing meme")
    }

    fn write_file(filename: PathBuf) -> Response {
        let mut f = File::open(filename).unwrap();
        let mut buf = vec![];
        f.read_to_end(&mut buf).expect("failed reading file");
        let mut res = Response::new(buf.into());
        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("image/jpeg"));
        res
    }
}

impl warp::Reply for HttpMemeMessage {
    fn into_response(self) -> warp::reply::Response {
        HttpMemeMessage::write_file(self.new_meme_file())
    }
}

#[derive(Debug)]
struct Meme {
    filename: String,
    text: String,
}

impl Meme {
    fn new_random<R>(rng: &mut R, cfg: &Config, uniform_dist: &Vec<MemeIndex>) -> Self
    where
        R: Rng,
    {
        let &MemeIndex {
            image_idx,
            text_idx,
        } = uniform_dist
            .choose(rng)
            .expect("failed choosing random meme");

        println!("({},{})", image_idx, text_idx);
        let img_cfg = &cfg.images[image_idx];
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

    pub fn write(&self, cfg: &Config) -> Result<PathBuf, Box<dyn Error + 'static>> {
        let id = crate::new_id(&self.filename, &self.text);

        let input_file = Path::new(&cfg.input_dir).join(self.filename.clone());
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
            crate::write_meme(&input_file, &output_file, &self.text);
            println!("wrote: {:?}", output_file);
        }

        Ok(output_file)
    }
}
