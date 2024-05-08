use crate::app::App;
use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, Config, CosmicConfigEntry},
    theme, Application,
};
use serde::{Deserialize, Serialize};

pub const CONFIG_VERSION: u64 = 1;

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize, CosmicConfigEntry)]
pub struct CosmicTasksConfig {
    pub app_theme: AppTheme,
}

impl CosmicTasksConfig {
    pub fn config_handler() -> Option<Config> {
        Config::new(App::APP_ID, CONFIG_VERSION).ok()
    }

    pub fn config() -> CosmicTasksConfig {
        match Self::config_handler() {
            Some(config_handler) => {
                let config = CosmicTasksConfig::get_entry(&config_handler).unwrap_or_else(
                    |(errs, config)| {
                        log::info!("errors loading config: {:?}", errs);
                        config
                    },
                );
                config
            }
            None => CosmicTasksConfig::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum AppTheme {
    Dark,
    Light,
    #[default]
    System,
}

impl AppTheme {
    pub fn theme(&self) -> theme::Theme {
        match self {
            Self::Dark => theme::Theme::dark(),
            Self::Light => theme::Theme::light(),
            Self::System => theme::system_preference(),
        }
    }
}
