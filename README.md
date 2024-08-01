# skse-rust-template

This is Ceej's template for setting up a repo for new Rust + CommonLibSSE-NG SKSE plugins. This is a hybrid approach, where you can mix and match C++ with Rust as you see fit. The template builds a working SKSE plugin that loads into the game and logs menu open/close events.

## Getting started

1. Install Rust using [rustup](https://rustup.rs).
2. Install [Visual Studio 2022](https://visualstudio.microsoft.com) with C++ compilers.
3. Install [CMake](https://cmake.org/download/). (The command runner does this for you on targets with homebrew.)
4. Install [ninja](https://ninja-build.org/) to do something or other involving project generation.
5. Set up [vcpkg](https://github.com/microsoft/vcpkg). Set `VCPKG_ROOT` in a user environment variable pointing to the directory where you cloned the repo.
6. Clone this template repo somewhere. `git init .` to start fresh with git.
7. `git submodule update --init --recursive` to pull in the commonlib fork and its submodules.

If you install the [just](https://just.systems) command runner, you can type `just full-build` to install some useful tools and compile the project. (If the project does not compile, please let me know so I can fix it.) There some recipes in the justfile that I find handy when developing. Note that I tend to keep a bash shell open for this, which you might not do.

To build by hand:

```sh
cmake --preset build-vs2022-windows # or just cmake
cmake --build --preset build-vs2022-windows --config Release  # or just build
```

CMake should trigger appropriate cargo builds, but it doesn't have the full list of rust source files so I usually run cargo as part of the `build` recipe. `cargo check` is also runnable separately. The `build.rs` file instructs cargo to build the `lib.rs.h` for the plugin specifically as well as `cxx.h` for the bridging types that Cxx uses. These are rebuilt when `lib.rs` changes, which should be the only trigger you need.

## Repo layout

* `src/main.cpp`: Registers with SKSE, sets up logging, calls all the setup code in `src/skse`.
* `src/lib.rs`: Defines the interface between C++ and Rust using [cxx](https://cxx.rs). Has examples of many of the common patterns you're likely to use.
* `src/bridge/`: Some conveniences in Rust for bridging C++ and Rust, most notably unified logging.
* `src/skse/`: SKSE responsibilities, in C++. Has examples of setting up hooks, event sinks, papyrus functions, and registering for cosave events, all of which are delegated to Rust.
* `src/util`: Some functions exposing useful game facilities (looking up scaleform translations, printing messages to the screen) as examples of how you might do this.

## Things to be noted

- I'm using the approach of making my own fork of clib-NG and using a branch of that fork as a git submodule. You probably want to make your own fork. The two repos of interest for this are [alandtse's fork](https://github.com/alandtse/CommonLibVR/) (use the `ng` branch) and the [CharmedBaryon fork](https://github.com/CharmedBaryon/CommonLibSSE-NG).
- I used [simplelog](https://lib.rs/crates/simplelog) for log-writing. You might want something else. This can easily be replaced in `logs.rs`.
- I provide some string wrangling in `bridge/strings.rs`. `cxx` assumes that any string data it's being given is a valid Rust string, aka a valid UTF-8 string, and this will not be true for all Skyrim data. I've seen crashes from ISO-8859-9 codepage characters. So before passing item names to Rust, decode them into UTF-8. (My code here can likely be improved. Removing a dep or two would trim down the code size.)
- Testing without the game running is possible if you do not pull in any functions that require the C++ side of the plugin. One trick is to implement live functions marked with `#[cfg(not(test))]` and then write a second test-specific version behind `#[cfg(test)]`. There are examples of this trick in `bridge/wrappers.rs`. Running `cargo test` runs the example tests, which are some shallow tests of the string functions.
- I pull some shenanigans so I can develop much of the Rust side of the plugin on my Mac laptop, where I have a very fast editor I like and can slouch with a cat on top of me while I work. You might not care about that. If you are Windows-only, you can remove those when you see them. (See the logging initialization function for an example.)
- The `CMakeLists.txt` file could probably be a lot better. I knew nothing about cmake when I started hitting it with a hammer to make it work.

## LICENSE

My intention is to license this template as free for open source and open for any modifications you wish to make. I do not want to constrain your licensing choices for your plugin beyond requiring that you also open-source it in some way that allows other people to learn from it. So to that end, I am using [The Parity Public License.](https://paritylicense.com) This license requires people who build on top of this source code to share their work with the community, too. In Skyrim modding language, this license allows "cathedral" modding, not "parlor" modding. Please see the text of the license for details.
