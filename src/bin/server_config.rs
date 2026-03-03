#[cfg(not(target_arch = "wasm32"))]
mod native {
    use serde::Deserialize;
    use std::{env, fs, path::PathBuf};

    #[derive(Debug, Default, Deserialize)]
    struct Config {
        #[serde(default)]
        serve: ServeConfig,
    }

    #[derive(Debug, Deserialize)]
    struct ServeConfig {
        #[serde(default = "default_address")]
        address: String,
        #[serde(default = "default_port")]
        port: u16,
        #[serde(default = "default_open")]
        open: bool,
    }

    impl Default for ServeConfig {
        fn default() -> Self {
            Self {
                address: default_address(),
                port: default_port(),
                open: default_open(),
            }
        }
    }

    fn default_address() -> String {
        "127.0.0.1".to_string()
    }

    const fn default_port() -> u16 {
        8080
    }

    const fn default_open() -> bool {
        true
    }

    pub fn run() -> anyhow::Result<()> {
        let mut args = env::args().skip(1);
        let config_path = args
            .next()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("config/development.toml"));

        let config = match fs::read_to_string(&config_path) {
            Ok(contents) => toml::from_str::<Config>(&contents)?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Config::default(),
            Err(err) => return Err(err.into()),
        };

        println!("SERVE_ADDRESS={}", config.serve.address);
        println!("SERVE_PORT={}", config.serve.port);
        println!("SERVE_OPEN={}", config.serve.open);

        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> anyhow::Result<()> {
    native::run()
}

#[cfg(target_arch = "wasm32")]
fn main() {}
