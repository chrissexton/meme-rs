#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub input_dir: String,
    pub output_dir: String,
    pub common_texts: Vec<String>,
    pub images: Vec<ImageConfig>,
}

#[derive(Debug, serde::Deserialize)]
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
}