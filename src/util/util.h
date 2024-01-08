#pragma once

// Pull in Cxx's rust types (not the plugin types).
#include "rust/cxx.h"

// Sorry about the file name.
namespace util
{
	void notifyPlayer(const std::string& message);
	rust::String lookupTranslation(const std::string& key);

	std::vector<uint8_t> chars_to_vec(const char* input);
	std::string nameAsUtf8(const RE::TESForm* form);
}
