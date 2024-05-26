use clap::*;

//----------[ Args ]----------//
pub fn clap_parse() -> ArgMatches {
	let cmd = clap::Command::new("cargo")
		.bin_name("tetrs")
		.about("A tui Tetris game written in Rust!")
		//----------[ Version ]----------//
		.arg(
			Arg::new("version")
				.long("version")
				.short('v')
				.action(ArgAction::SetTrue),
		);
		//-------------------------------//
	cmd.get_matches()
}
//----------------------------//
