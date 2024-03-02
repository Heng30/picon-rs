use anyhow::{anyhow, Result};
use std::{env, fs};

#[allow(unused_imports)]
use std::path::PathBuf;

#[cfg(not(target_os = "android"))]
use platform_dirs::AppDirs;

#[cfg(target_os = "android")]
pub struct AppDirs {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
}

#[cfg(target_os = "android")]
impl AppDirs {
    pub fn new(name: Option<&str>, _: bool) -> Option<Self> {
        let root_dir = "/data/data";
        let name = name.unwrap();

        Some(Self {
            config_dir: PathBuf::from(&format!("{root_dir}/{name}/config")),
            data_dir: PathBuf::from(&format!("{root_dir}/{name}/data")),
        })
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub working_dir: PathBuf,

    #[serde(skip)]
    pub config_path: PathBuf,

    #[serde(skip)]
    pub db_path: PathBuf,

    #[serde(skip)]
    pub cache_dir: PathBuf,

    pub ui: UI,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UI {
    pub is_cn: bool,
}

impl Default for UI {
    fn default() -> Self {
        Self { is_cn: true }
    }
}

impl Config {
    pub fn init(&mut self) -> Result<()> {
        let app_name = if cfg!(not(target_os = "android")) {
            "picon"
        } else {
            "xyz.heng30.picon"
        };

        let app_dirs = AppDirs::new(Some(app_name), true).unwrap();

        self.init_app_dir(&app_dirs)?;
        self.load()?;
        log::debug!("{:?}", self);
        Ok(())
    }

    fn init_app_dir(&mut self, app_dirs: &AppDirs) -> Result<()> {
        self.config_path = app_dirs.config_dir.join("picoin.conf");
        self.db_path = app_dirs.data_dir.join("picoin.db");

        self.cache_dir = app_dirs.data_dir.join("cache");
        self.working_dir = {
            let mut dir = env::current_exe()?;
            dir.pop();
            dir
        };

        fs::create_dir_all(&app_dirs.config_dir)?;
        fs::create_dir_all(&app_dirs.data_dir)?;
        fs::create_dir_all(&self.cache_dir)?;

        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        match fs::read_to_string(&self.config_path) {
            Ok(text) => match serde_json::from_str::<Config>(&text) {
                Ok(c) => {
                    self.ui = c.ui;
                    Ok(())
                }
                Err(e) => Err(anyhow!("{e:?}")),
            },
            Err(_) => match serde_json::to_string_pretty(self) {
                Ok(text) => Ok(fs::write(&self.config_path, text)?),
                Err(e) => Err(anyhow!("{e:?}")),
            },
        }
    }

    pub fn save(&self) -> Result<()> {
        match serde_json::to_string_pretty(self) {
            Ok(text) => Ok(fs::write(&self.config_path, text)?),
            Err(e) => Err(anyhow!("{e:?}")),
        }
    }
}
