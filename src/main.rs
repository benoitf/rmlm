// SPDX-License-Identifier: Apache-2.0

use std::env;

mod cli;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let exit_code = cli::run_cli(args);
    std::process::exit(exit_code);
}
