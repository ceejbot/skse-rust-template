#include "papyrus.h"

#include "lib.rs.h"

namespace papyrus
{
	static const char* MCM_NAME = "skse-rust-template";

	void registerNativeFunctions()
	{
		const auto* papyrus = SKSE::GetPapyrusInterface();
		papyrus->Register(callback);
	}

	bool callback(RE::BSScript::IVirtualMachine* a_vm)
	{
		a_vm->RegisterFunction("StringToInt", MCM_NAME, stringToInt);
		return true;
	}

	int stringToInt(RE::TESQuest*, RE::BSFixedString number)
	{
		auto numstr = std::string(number);
		// Here we call a Rust function that we've pulled in from the bridge.
		return string_to_int(numstr);
	}
}
