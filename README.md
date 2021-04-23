# COVID Tracker India

A command line tool written in Rust to view COVID-19 statistics in India.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

### Installation:

1. Install the rust compiler `rustc` and package manager `cargo` from the [Rust website](https://www.rust-lang.org/tools/install).

1. Go to a directory where you wish to download the source code and enter this:

    ```bash
    $ git clone https://github.com/aadilshabier/covid-tracker-rs.git
    ```

1. Go to the installation directory and build the program using cargo with the release flag
    ```bash
    $ cargo build --release
    ```
1. The binary `covid` in 'target/release/' can now be run

1. On Unix machines you can run `install.sh` to copy the binary to `/usr/local/bin/`, and `uninstall.sh` to remove it

    __Note:__ Requires sudo permissions

### API:

https://api.covid19india.org

### TODO:

Better string similarity algorithm

