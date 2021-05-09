#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub input_dir: String,
    pub output_dir: String,
    #[serde(default)]
    pub common_texts: Vec<String>,
    pub images: Vec<ImageConfig>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ImageConfig {
    pub name: String,
    #[serde(default)]
    pub texts: Vec<String>,
    #[serde(default)]
    pub raw: bool,
}

impl Config {
    pub fn load() -> Self {
        let f = std::fs::File::open("config.yml").expect("failed opening config file");
        serde_yaml::from_reader(f).expect("failed parsing config file")
    }

    pub fn create_uniform_distribution(&self) -> Vec<MemeIndex> {
        let mut uniform_dist = vec![];
        for (idx, c) in self.images.iter().enumerate() {
            if c.raw {
                uniform_dist.push(MemeIndex::new(idx, 0));
            } else {
                for t_idx in 0..self.common_texts.len() {
                    uniform_dist.push(MemeIndex::new(idx, t_idx));
                }
                for t_idx in 0..c.texts.len() {
                    uniform_dist.push(MemeIndex::new(idx, self.common_texts.len() + t_idx));
                }
            }
        }
        uniform_dist
    }
}

#[derive(Debug, Clone)]
pub struct MemeIndex {
    pub image_idx: usize,
    pub text_idx: usize,
}

impl MemeIndex {
    pub fn new(image_idx: usize, text_idx: usize) -> Self {
        MemeIndex {
            image_idx,
            text_idx,
        }
    }
}
