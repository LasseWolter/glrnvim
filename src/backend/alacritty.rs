use super::Functions;
use crate::config::Config;
use crate::error::GlrnvimError;
use crate::NVIM_NAME;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;
extern crate serde_yaml;

pub const ALACRITTY_NAME: &str = "alacritty";

struct Alacritty {
    exe_path: PathBuf,
    temp_file: Option<NamedTempFile>,
}

pub fn init(config: &Config) -> Result<Box<dyn Functions>, GlrnvimError> {
    let exe_path = super::exe_path(&config.exe_path, ALACRITTY_NAME)?;

    Ok(Box::new(Alacritty {
        exe_path,
        temp_file: None,
    }))
}

impl Alacritty {
    fn create_conf_file(&mut self, config: &Config) {
        let base_mapping = serde_yaml::Mapping::new();
        if !base_mapping.contains_key(&serde_yaml::to_value("font").unwrap()) {
        }
        base_mapping.insert()

        let mut file = tempfile::NamedTempFile::new().unwrap();
        writeln!(file, "font:").unwrap();
        writeln!(file, "  size: {}", config.font_size).unwrap();

        if !config.fonts.is_empty() {
            writeln!(file, "  normal:").unwrap();
        }
        for font in &config.fonts {
            writeln!(file, "    family: \"{}\"", font).unwrap();
            // TODO: Alacritty doesn't support fallback font well.
            // See https://github.com/jwilm/alacritty/issues/957
            break;
        }

        writeln!(file, "key_bindings:").unwrap();
        writeln!(file, "  - {{key: Z, mods: Control, action: None}} ").unwrap();
        file.flush().unwrap();

        file.path();
        self.temp_file = Some(file);
    }

    fn create_base_yaml_mapping() -> serde_yaml::Mapping {
        let base_confs:[String; 0] = [];
        let pri_confs:[String; 3] = [
            "$XDG_CONFIG_HOME/alacritty/alacritty.yml".to_string(),
            "$HOME/.config/alacritty/alacritty.yml".to_string(),
            "$XDG_CONFIG_DIRS/alacritty/alacritty.yml".to_string(),
        ];
        let confs = super::find_term_conf_files(&base_confs, &pri_confs);
        if confs.len() > 0 {
            let path = confs[0].to_owned();
            let file = std::fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(file);
            match serde_yaml::from_reader(reader) {
                Ok(mapping) => mapping,
                Err(_) => {
                    serde_yaml::Mapping::new()
                }
            }
        } else {
            serde_yaml::Mapping::new()
        }
    }
}

impl Functions for Alacritty {
    fn create_command(&mut self, config: &Config) -> std::process::Command {
        let base_conf = Alacritty::create_base_yaml_mapping();
        println!("{}", serde_yaml::to_string(&base_conf).unwrap());

        self.create_conf_file(config);
        let mut command = std::process::Command::new(self.exe_path.to_owned());
        command.arg("--config-file");
        command.arg(self.temp_file.as_ref().unwrap().path());
        command.arg("--class");
        command.arg("glrnvim");

        if let Ok(current_dir) = std::env::current_dir() {
            command.arg("--working-directory");
            command.arg(current_dir);
        }

        command.arg("-e");
        command.arg(NVIM_NAME);

        // Enable 24-bits colors
        command.arg("+set termguicolors");
        // Set title string
        command.arg("+set title");
        command
    }
}
