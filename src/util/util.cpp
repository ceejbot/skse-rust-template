#include "util.h"

#include "lib.rs.h"

namespace util
{
	// How you know I've been replaced by a pod person: if I ever declare that
	// I love dealing with strings in systems programming languages.

	// The Cxx bridge wants us to use Vec<u8> for transferring bags of bytes,
	// not a pointer to char with a null at the end.
	std::vector<uint8_t> chars_to_vec(const char* input)
	{
		if (!input) { return std::move(std::vector<uint8_t>()); }
		auto incoming_len = strlen(input);
		if (incoming_len == 0) { return std::move(std::vector<uint8_t>()); }

		std::vector<uint8_t> result;
		result.reserve(incoming_len + 1);  // null terminator
		for (auto* ptr = input; *ptr != 0; ptr++) { result.push_back(static_cast<uint8_t>(*ptr)); }
		result.push_back(0x00);  // there it is
		return std::move(result);
	}

	// Decode a wild-west item name to utf-8.
	std::string nameAsUtf8(const RE::TESForm* form)
	{
		auto name     = form->GetName();
		auto chonker  = chars_to_vec(name);
		auto safename = std::string(cstr_to_utf8(chonker));
		return safename;
	}

	// Post a text notification to the screen.
	void notifyPlayer(const std::string& message)
	{
		auto* msg = message.c_str();
		RE::DebugNotification(msg);
	}

	// Look up a scaleform translation by key.
	rust::String lookupTranslation(const std::string& key)
	{
		std::string translated = std::string();
		SKSE::Translation::Translate(key, translated);
		return translated;
	}
}
