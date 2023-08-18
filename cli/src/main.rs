//! todo
mod common;
mod opts;
use prdoclib::*;

use std::env;

use clap::{crate_name, crate_version, Parser};
use env_logger::Env;
use log::*;
use opts::*;
use serde_json::json;

/// Main entry point of the cli
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	color_eyre::install()?;

	let opts: Opts = Opts::parse();
	// debug!("opts: {opts:#?}");

	match opts.subcmd {
		Some(SubCommand::Generate(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			todo!();
			Ok(())
		}

		Some(SubCommand::Check(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			if let Some(file) = cmd_opts.file {
				debug!("Checking file {}", file.display());

				let result = Schema::check(file);
				if result {
					std::process::exit(exitcode::OK);
				} else {
					std::process::exit(exitcode::DATAERR);
				}
			}

			if let Some(dir) = cmd_opts.directory {
				debug!("Checking directory {}", dir.display());

			}

			if let Some(number) = cmd_opts.number {
				debug!("Checking PR #{}", number);


			}

			Ok(())
		}

		Some(SubCommand::Scan(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			let res = DocFile::find(&cmd_opts.directory, !cmd_opts.all);
			res.for_each(|hit| {
				println!("{}", hit.display());
			});
			Ok(())
		}

		Some(SubCommand::Load(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			todo!();
			Ok(())
		}

		Some(SubCommand::Schema(cmd_opts)) => {
			debug!("cmd_opts: {cmd_opts:#?}");
			let schema = Schema::get(true);
			println!("{schema}");
			Ok(())
		}

		None => {
			if opts.version {
				let name = crate_name!();
				let version = crate_version!();
				let commit_hash = env::var("PRDOC_CLI_GIT_COMMIT_HASH");
				let build_date = env::var("PRDOC_CLI_BUILD_DATE");

				if !opts.json {
					let commit_hash_str = if let Ok(s) = commit_hash { format!("-{s}") } else { String::from("") };
					let build_date_str = if let Ok(s) = build_date { format!(" built {s}") } else { String::from("") };
					println!("{name} v{version}{commit_hash_str}{build_date_str}");
				} else {
					let version_data = json!({
						"name": name,
						"version": version,
						"commit": commit_hash.unwrap_or_default(),
						"build_date": build_date.unwrap_or_default(),
					});
					let s = serde_json::to_string_pretty(&version_data).expect("serde_json ran into issues");
					println!("{s}");
				}
				Ok(())
			} else {
				unreachable!("We show help if there is no arg");
			}
		}
	}
}
