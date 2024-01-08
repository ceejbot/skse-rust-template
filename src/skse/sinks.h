#pragma once

void registerEventSinks();

// I am hilarious. Truly hilarious.
class KitchenSink final : public RE::BSTEventSink<RE::MenuOpenCloseEvent>
{
public:
	static KitchenSink* singleton(void);

	// It's a programmer error to have more than one.
	KitchenSink(const KitchenSink&) = delete;
	KitchenSink(KitchenSink&&)      = delete;

	KitchenSink& operator=(const KitchenSink&) = delete;
	KitchenSink& operator=(KitchenSink&&)      = delete;

protected:
	// make this class inherit from as many sink types as you want,
	// and implement the process methods for each
	/*
		RE::BSEventNotifyControl ProcessEvent(const RE::TESEquipEvent* event,
			[[maybe_unused]] RE::BSTEventSource<RE::TESEquipEvent>* source) override;

		RE::BSEventNotifyControl ProcessEvent(RE::InputEvent* const* eventList,
			[[maybe_unused]] RE::BSTEventSource<RE::InputEvent*>* source) override;
	*/

	RE::BSEventNotifyControl ProcessEvent(const RE::MenuOpenCloseEvent* a_event,
		RE::BSTEventSource<RE::MenuOpenCloseEvent>* a_eventSource) override;

private:
	KitchenSink()           = default;
	~KitchenSink() override = default;
};
