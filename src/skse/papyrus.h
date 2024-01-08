#pragma once

namespace papyrus
{
	void registerNativeFunctions();
	bool callback(RE::BSScript::IVirtualMachine* a_vm);

	// A contrived example.
	int stringToInt(RE::TESQuest*, RE::BSFixedString number);
};
