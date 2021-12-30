use lib::{bootloader, packages, partitions, system, users};
use serde::Deserialize;
use std::{fs, path::PathBuf};
use structopt::StructOpt;
use toml;

#[derive(Debug, StructOpt)]
struct Opt {
	#[structopt(short, long)]
	config_file: Option<PathBuf>,
}

pub struct Config {
	pub mode: Mode,
	pub config: Option<InstallConfig>,
}

pub enum Mode {
	Cli,
	File,
}

#[derive(Debug, Deserialize)]
pub struct InstallConfig {
	#[serde(rename = "partition")]
	pub partitions: Vec<partitions::Partition>,
	pub packages: packages::Packages,
	pub system: system::System,
	pub bootloader: bootloader::BootOptions,
	#[serde(rename = "user")]
	pub users: Vec<users::User>,
	pub root: users::Root,
}

pub fn config() -> Config {
	match Opt::from_args().config_file {
		Some(path) => match path.extension() {
			Some(ext) => {
				if ext.eq("toml") {
					match fs::read_to_string(path) {
						Ok(content) => Config {
							mode: Mode::File,
							config: Some(
								toml::from_str(content.as_str())
									.expect("error reading toml config file"),
							),
						},
						Err(e) => panic!("Error reading config file... \n {}", e),
					}
				} else {
					panic!("the tool only supports toml config files");
				}
			}
			None => {
				panic!("the tool works with toml files");
			}
		},
		None => Config {
			mode: Mode::Cli,
			config: None,
		},
	}
}
