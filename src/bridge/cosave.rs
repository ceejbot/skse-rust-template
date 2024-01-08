//! The rust side of cosave serialization.

use cxx::CxxVector;

const COSAVE_DATA_VERSION: u32 = 1;

/// Return the plugin's current cosave format version.
/// Versioning is left as an exercise for the reader (hint below).
pub fn cosave_data_version() -> u32 {
    COSAVE_DATA_VERSION
}

/// Do whatever you'd like to do to serialize your plugin's data
/// to bytes. I use bincode for this, but you might choose something
/// else.
pub fn cosave_data() -> Vec<u8> {
    todo!()
}

pub fn cosave_loaded(ffi_bytes: &CxxVector<u8>, version: u32) {
    // Gear-change from the bridge type to the native Rust type.
    let _bytes: Vec<u8> = ffi_bytes.iter().copied().collect();

    match version {
        1 => {
            // deserialize your plugin's data here and send it to whatever in
            // your plugin needs to have it.
        }
        _ => {
            log::warn!("Unknown cosave data version {version}! Cannot load save data.");
        }
    }
}
