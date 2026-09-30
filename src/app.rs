use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use zopra::{component, hooks::use_state, view, cn};
use crate::components::titlebar::AppTitleBar;

#[component]
pub fn incrementer(number: i32) {
    let (count, set_count) = use_state(number);
    let count_val = count();
    let count_text = format!("Count is {}", count_val);
    let is_even = count_val % 2 == 0;

    view! {
        <div
        id="increment"
        class={cn!(
            "flex items-center justify-center border w-40 bg-[#ededed] text-[#0a0a0a] px-6 py-3 rounded-full cursor-pointer hover:bg-[#d4d4d8]",
            (!is_even).then_some("bg-white text-black border-white"),
            (is_even).then_some("bg-black text-white border-white"),
        )}
        onClick={move |_, _, cx| set_count(count_val + 1, cx)}>
            {count_text}
        </div>
    }
}


#[component]
pub fn app() {
    view! {
        <div class="flex flex-col size-full bg-[#0a0a0a] text-[#ededed]">
            <AppTitleBar />
            <div class="flex-1 flex flex-col items-center justify-center px-24">
                <div class="flex items-center justify-center gap-6 mb-4">
                    <img src="Zopra.svg" class="size-20"/>
                    <div class="text-6xl font-bold">
                        "ZOPRA"
                    </div>
                </div>
                <div class="flex items-center mb-8 text-xl">
                    <span>"To get started, edit the"</span>
                    <div class="bg-[#181818] rounded-2xl mx-2 px-2">
                        "src/app.rs"
                    </div>
                    <span>"file"</span>
                </div>
                <div class="flex items-center gap-6">
                    <div class="flex gap-[0.75rem] items-center justify-center w-40 border border-[#ededed] bg-[#ededed] text-[#0a0a0a] px-6 py-3 rounded-full cursor-pointer hover:bg-[#d4d4d8]">
                        <span>"꩜"</span>
                        <span>"Read Docs"</span>
                    </div>
                    <Incrementer number={0} />
                </div>
            </div>
        </div>
    }
}

