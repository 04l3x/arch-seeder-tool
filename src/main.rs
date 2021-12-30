mod config;

use lib::{
	error::{Error, Result},
	*,
};

/**
 * verify partitions
 * mount filesystem
 * base installation
 * generate fstab
 * post configuration
 * install grub
 * dialog for setting password for root
 * dialog for user creation and setting password
 * */
fn main() {
	run(config::config());
}

fn run(config: config::Config) {
	let exit = |status: Result<()>| {
		match status {
			Ok(_) => println!("install success! :)"),
			Err(_) => println!("something went wrong dutring installation"),
		};
	};

	match config.mode {
		config::Mode::Cli => exit(cli_install()),
		config::Mode::File => exit(install(config.config.expect(""))),
	}
}

fn install(config: config::InstallConfig) -> Result<()> {
	println!("running tool based on config file");
	println!("current config is: \n {:?}", config);

	partitions::worker::mount_file_system(config.partitions);
	packages::worker::install(config.packages);
	system::worker::set_system_config(config.system);
	bootloader::install(config.bootloader).unwrap();
	users::worker::set_root(config.root).unwrap();
	users::worker::set_users(config.users).unwrap();
	crate::exit()
}

fn cli_install() -> Result<()> {
	println!("TODO: build the cli installer");
	println!("cli not available, currently only file config is supported :'(");

	Err(Box::new(Error))
}
