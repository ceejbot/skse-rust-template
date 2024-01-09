#![deny(future_incompatible, clippy::unwrap_used)]
#![warn(rust_2018_idioms, trivial_casts)]

//! The Rust library.

pub mod bridge;

use bridge::cosave::*;
use bridge::logs::*;
use bridge::strings::*;

/// This is the cxx bridge submodule. This is where all shared data types and
/// functions are declared, so that the cxx macros can generate code for them.
/// To see the generated code, look in `target/cxxbridge/skse-rust-template/src/`.
/// There are more comments in the source file than show up here in the docs.
/// See https://cxx.rs for full documentation on the cxx crate.
#[cxx::bridge]
pub mod plugin {
    // Shared data structures

    // If you want a type to be transparent to BOTH Rust and C++,
    // put it here above the Rust block. There are limitations on how
    // expressive you can be with those types. No traits, no enum
    // tuples or structs (only simple C-style enums). The compiler will
    // warn you if you are trying to use something Cxx can't express
    // to C++. See the docs.

    // Color shows most of the traits you're allowed to derive. You can't derive
    // `Default`, however. Serde's `Deserialize` and `Serialize` *are* allowed.

    /// Color as rgba between 0 and 255. The default is white at full alpha.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TerseColor {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    }

    extern "Rust" {
        // Things Rust exposes to C++. Practical examples follow for things you are likely to want in your plugin.
        // Cosave support. Called by cosave.cpp.

        /// Report the current version of the cosave data format.
        fn cosave_data_version() -> u32;
        /// Generate cosave data to be stored in the save file.
        fn cosave_data() -> Vec<u8>;
        /// Handle data read from a cosave file tagged with our id.
        /// It's our responsibility to handle backwards compatibility with
        /// older save formats.
        fn cosave_loaded(bytes: &CxxVector<u8>, version: u32);

        // Next are shims for sending C++-originated log lines to the Rust logging
        // facilities. This lets us have a single log file without having to interleave
        // log lines and formats.

        /// Tell Rust where to log. A wide string that we'll pass to the path library
        /// as an OsString, so no utf8-encoding needed.
        fn initialize_logging(logdir: &CxxVector<u16>);

        // These functions are consumed by the templates in log.h on the C++ side.
        // You'll want to use the templates via `rlog::info()` etc instead of calling these
        // directly.

        /// Log at error level. Use this level for unrecoverable problems. Consider crashing.
        fn log_error(message: String);
        /// Log at warn level. Use this level for problems the player might want to fix.
        fn log_warn(message: String);
        /// Log at info level. Use this level for normal operations and to proactively
        /// inform the player about their settings and data. Confirm the results of actions.
        fn log_info(message: String);
        /// Log at debug level. Use this level to help players debug problems: put things in here
        /// that you'll find useful if somebody sends you a log file.
        fn log_debug(message: String);
        /// Log at trace level. Use this level for debugging your programming problems.
        fn log_trace(message: String);

        /// Decode a null-terminated C string from whatever it is to utf-8.
        fn cstr_to_utf8(bytes_ffi: &CxxVector<u8>) -> String;

        /// From the papyrus example.
        fn string_to_int(number: String) -> i32;

        // If we had a rust type called Ferris, with a function implemented on it
        // called "scuttle()" that returns a distance scuttled, we would export it
        // here in two steps. First, we declare that Ferris is a Rust type that is opaque
        // to C++. Then we declare the function's signature.
        // type Ferris;
        // fn scuttle(self: &Ferris) -> f32;
    }

    unsafe extern "C++" {
        // Each of these blocks declares things C++ exposes to Rust.
        // All of the C++ is quarantined inside unsafe blocks, aka an instruction
        // to rustc to just trust us, bro.
    }

    // This block takes the functions we defined in our util file and makes
    // them available to Rust to be called. Cxx benefits from having the header
    // to consult to generate bridge code behind the scenes. These are functions
    // you're likely to want to re-use.
    #[namespace = "util"]
    unsafe extern "C++" {
        include!("util.h");

        /// Display a notification on the screen. You must format and translate in advance.
        fn notifyPlayer(message: &CxxString);
        /// Look up a translation for a format string.
        fn lookupTranslation(key: &CxxString) -> String;
    }

    // Here is an example of pulling in game types and exposing some of
    // their methods to Rust. You might not need these specific types in your plugin,
    // but this is the pattern to follow.
    #[namespace = "RE"]
    unsafe extern "C++" {
        include!("PCH.h");

        /// The form object declared as a C++ type that is opaque to Rust.
        type TESForm;
        /// Get the id for this form. This is the pattern for how C++ methods are
        /// expressed in the bridge: a parallel to how Rust functions on struct types work.
        fn GetFormID(self: &TESForm) -> u32;

        /// The equip slot for an item.
        type BGSEquipSlot;
        /// A keyboard, mouse, or gamepad button event. Imported from CommonLibSE.
        type ButtonEvent;
        /// Check if this is a button-down event.
        fn IsDown(self: &ButtonEvent) -> bool;
        /// Check if this is a button-up event.
        fn IsUp(self: &ButtonEvent) -> bool;
        /// Check if this button is pressed.
        fn IsPressed(self: &ButtonEvent) -> bool;
    }
}
