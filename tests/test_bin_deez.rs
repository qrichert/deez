// deezconfigs — Manage deez config files.
// Copyright (C) 2025  Quentin Richert
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

mod conf;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use conf::HOME;

const DEEZ: &str = env!("CARGO_BIN_EXE_deez");

struct Output {
    exit_code: i32,
    stdout: String,
    stderr: String,
}

fn run(args: &[&str]) -> Output {
    let mut command = Command::new(DEEZ);

    for arg in args {
        command.arg(arg);
    }

    let output = command.output().unwrap();

    Output {
        exit_code: output.status.code().unwrap(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    }
}

fn file_exists_in_home(file_path: &str) -> bool {
    let file = PathBuf::from(HOME).join(file_path);
    file.is_file()
}

// fn symlink_exists_in_home(symlink_path: &str) -> bool {
//     let symlink = PathBuf::from(HOME).join(symlink_path);
//     symlink.is_symlink()
// }

#[test]
fn help() {
    let output = run(&["--help"]);

    assert_eq!(output.exit_code, 0);
    assert!(output.stdout.contains("-h, --help"));
    assert!(output.stdout.contains("-v, --version"));
}

#[test]
fn no_args_shows_help() {
    let output = run(&[]);

    assert_eq!(output.exit_code, 0);
    assert!(output.stdout.contains("-h, --help"));
}

#[test]
fn version() {
    let output = run(&["--version"]);

    assert_eq!(output.exit_code, 0);
    assert!(output.stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn bad_argument() {
    let output = run(&["--bad-argument"]);

    assert_eq!(output.exit_code, 2);
    assert!(output.stderr.contains("'--bad-argument'"));
}

#[test]
fn sync_regular() {
    conf::init();

    conf::create_file(".gitconfig");
    conf::create_file(".config/nvim/init.lua");
    conf::create_file(".config/fish/config.fish");
    conf::create_symlink(".config/ghostty/config");

    let output = run(&["sync", &conf::root()]);

    assert_eq!(output.exit_code, 0);
    dbg!(output.stdout);
    dbg!(output.stderr);

    assert!(file_exists_in_home(".gitconfig"));
    assert!(file_exists_in_home(".config/nvim/init.lua"));
    assert!(file_exists_in_home(".config/fish/config.fish"));
    assert!(file_exists_in_home(".config/ghostty/config"));
}

#[test]
fn sync_ignores_special_files() {
    conf::init();

    // OK.
    conf::create_file("subdir/.git/config");
    conf::create_file("subdir/.gitignore");
    // NOT OK.
    conf::create_file(".gitignore");
    conf::create_file(".git/config");

    let output = run(&["sync", &conf::root()]);

    assert_eq!(output.exit_code, 0);
    dbg!(output.stdout);
    dbg!(output.stderr);

    // OK in sub-directories.
    assert!(file_exists_in_home("subdir/.git/config"));
    assert!(file_exists_in_home("subdir/.gitignore"));

    // NOT OK in root.
    assert!(!file_exists_in_home(".gitignore"));
    assert!(!file_exists_in_home(".git/config"));
}
