# skse-rust-template

This is Ceej's template for setting up a repo for new Rust + CommonLibSSE-NG SKSE plugins.

## Getting started

1. Install Rust using [rustup](https://rustup.rs).
2. [Visual Studio 2022](https://visualstudio.microsoft.com) with C++ compilers installed
3. Install [CMake](https://cmake.org/download/). (The command runner does this for you on targets with homebrew.)
4. Set up [vcpkg](https://github.com/microsoft/vcpkg). Set `VCPKG_ROOT` in a user environment variable pointing to the directory where you cloned the repo.
5. Clone this repo somewhere. `git init .` to start fresh with git.

If you install the [just](https://just.systems) command runner, you can then type `just full-build` and the project should compile.

```sh
cmake --preset build-vs2022-windows
cmake --build --preset build-vs2022-windows --config Release
```

CMake should trigger appropriate cargo builds, but you can always run those separately yourself. `cargo check` is also runnable separately. The `build.rs` file instructs cargo to build the `lib.rs.h` for the plugin specifically as well as `cxx.h` for the bridging types that Cxx uses. These are rebuilt when `lib.rs` changes, but you can change that by editing build.rs.

## Repo layout

* `src/main.cpp`: Registers with SKSE, sets up logging, hooks, papyrus native functions, and event sinks.
* `src/lib.rs`: Defines the interface between C++ and Rust using [cxx](https://cxx.rs).
* `src/bridge/`: Some conveniences in Rust for bridging C++ and Rust, most notably unified logging.
* `src/skse/`: SKSE responsibilities, in C++. Has examples of setting up hooks, event sinks, papyrus functions, and registering for cosave events.
* `src/util`: Some conveniences in C++ for bridging to Rust.

## Things to be noted

- I'm using the approach of making my own fork of clib-NG and using a branch of that fork as a git submodule. You probably want to make your own fork. The two repos of interest for this are [alandtse's fork](https://github.com/alandtse/CommonLibVR/) (use the `ng` branch) and the [CharmedBaryon fork](https://github.com/CharmedBaryon/CommonLibSSE-NG).
- I used [simplelog](https://lib.rs/crates/simplelog) for log-writing. You might want something else. This can easily be ripped out from `logs.rs`.
- I provide some string decoding in `bridge/strings.rs`. `cxx` assumes that any string data it's being given is a valid Rust string, and this will not be true for all Skyrim data. I've seen crashes from ISO-8859-9 codepage characters. So before passing names to Rust, decode into utf-8. My code can likely be improved.
- Testing without the game running is possible if you do not pull in any functions that require the C++ side of the plugin. One trick is to implement live functions marked with `#[cfg(not(test))]` and then write a second test-specific version behind `#[cfg(test)]`. There are examples in `bridge/wrappers.rs`.
- I pull some shenanigans so I can develop much of the Rust side of the plugin on my Mac laptop, where I have a very fast editor I like and can slouch with a cat on top of me while I work. You might not care about that. If you are Windows-only, you can rip those out. (See the logging initialization function for an example.)

## LICENSE

What I want to do is license this template as free for open source and open for any modifications you wish to make. I do not want to constrain your licensing choices for your plugin beyond requiring that you also open-source it in some way that allows other people to learn from it. So to that end, I am using [The Parity Public License.](https://paritylicense.com) This license requires people who build on top of this source code to share their work with the community, too. In Skyrim modding language, this license allows "cathedral" modding, not "parlor" modding. Please see the text of the license for details.
