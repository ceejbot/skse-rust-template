#pragma once

#define WIN32_LEAN_AND_MEAN
#define NOMINMAX

// add extern/CommonLibSSE-NG/ to project include paths

#include <RE/Skyrim.h>
#include <SKSE/SKSE.h>

#include <algorithm>
#include <cctype>
#include <locale>
#include <windows.h>
#include <winuser.h>

// Templates for sending C++ logging to Rust.
#include "util/log.h"

using namespace std::literals;

namespace stl
{
	using namespace SKSE::stl;

	template <class T>
	void write_thunk_call()
	{
		auto& trampoline = SKSE::GetTrampoline();
		const REL::Relocation<std::uintptr_t> hook{ T::id, T::offset };
		T::func = trampoline.write_call<5>(hook.address(), T::thunk);
	}
}  // namespace stl

#define EXTERN_C extern "C"

#include "Version.h"
