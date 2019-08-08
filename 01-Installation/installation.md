# Installation

## Windows

If behind a proxy, start by defining the **power shell proxy setup**:

`$proxy='http://10.144.1.10:8080'`

`$ENV:HTTP_PROXY=$proxy`

`$ENV:HTTPS_PROXY=$proxy`

Installation made by running [downlodable](<https://www.rust-lang.org/tools/install>):

`.\rustup-init.exe`

This will download and install the official compiler for the Rust programming language, and its package manager, **Cargo**.

It will add the **cargo**, **rustc**, **rustup** and other commands to Cargo's bin directory, located at:

`C:\Users\pt101492\.cargo\bin`

This path will is added to PATH environment variable by modifying the HKEY_CURRENT_USER/Environment/PATH registry key.

Uninstall with **rustup** self uninstall.

Check installation with:

`rustc --version`

**NOTE**: Install Visual Studio  2019 with C++ components for the linker.

## Linux and Mac OS

~In termina lexecue command:

`curl https://sh.rustup.rs -sSf | sh`

Load environment with:

`source ~/.cargo/env`

## Call Documentation

rustup doc