use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use gpui_kit::component::TitleBar;
use zopra::{component, view};

#[component]
pub fn app_title_bar() {
    view! {
        <TitleBar class="border-b border-[#272a2f] bg-[#141517]">
            <div class="flex items-center px-2">
                <div class="text-[#d9dbe0] text-sm font-semibold">"Zopra App"</div>
            </div>
            // spacer
            <div class="flex items-center flex-1 justify-end px-2 gap-2 text-[#d9dbe0]">
            </div>
        </TitleBar>
    }
}

