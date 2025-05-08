// SPDX-License-Identifier: Apache-2.0

/// Checks if the arguments match the `--version` pattern
pub fn handle_version(args: &[String]) -> Option<i32> {
    if args.len() == 1 && args[0] == "--version" {
        println!("{}", env!("CARGO_PKG_VERSION"));
        Some(0)
    } else {
        None
    }
}
