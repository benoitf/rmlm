// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::process::exit;

mod cli;
mod version;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if let Some(code) = version::handle_version(&args) {
        exit(code);
    }

    let exit_code = cli::run_cli(args);
    std::process::exit(exit_code);
}
