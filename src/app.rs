use bollard::Docker;
use cursive::Cursive;

use tokio::runtime::Runtime;
// use std::future::Future;
// use tokio::{self, task};
use dirs;

use std::default::Default;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::path::PathBuf;

const STYLE_PATH: &'static str = "style.toml";

#[derive(Default)]
struct Config {
    style_path: PathBuf,
}

pub struct App {
    pub runtime: Runtime,
    pub docker: Docker,
    config: Config,
}

impl App {
    pub fn new(ui: &mut Cursive) -> Self {
        let mut app = App {
            runtime: Runtime::new().unwrap(),
            docker: Docker::connect_with_local_defaults().unwrap(),
            config: Default::default(),
        };

        // Load config.
        app.config = app.get_or_create_config();
        let mut style_content = String::new();
        File::open(&app.config.style_path)
            .expect("Cannot find config file.")
            .read_to_string(&mut style_content)
            .expect("Cannot read config file.");
        ui.load_toml(style_content.as_str()).unwrap();

        app
    }

    /// Returns and if needed crates all the config
    /// paths needed for app.
    ///
    /// Counts on flat directory structure where all files are
    /// placed in the config directory directly. This means config
    /// directory doesn't contain any other directory.
    fn get_or_create_config(&self) -> Config {
        let cfg = self.get_config_dir_path();

        // Create config dir if doesn't exist.
        if !cfg.exists() {
            create_dir(&cfg).expect(
                format!("Couldn't create config dir in {}", cfg.to_str().unwrap()).as_str(),
            );
        }

        // Create config files if s doesn't exist.
        let style_path = cfg.join(STYLE_PATH);
        let all_paths = [&style_path];

        // Walk thru all paths and if some of them doesn't exist
        // create it.
        for path in all_paths.iter() {
            if !path.exists() {
                File::create(path).expect(
                    format!("Couldn't create file in path {}", &path.to_str().unwrap()).as_str(),
                );
            }
        }

        // Return all paths in a struct.
        Config { style_path }
    }

    /// Determines app config directory.
    fn get_config_dir_path(&self) -> PathBuf {
        dirs::config_dir()
            .expect("Couldn't find config dir.")
            .join("kosatka")
    }
}
