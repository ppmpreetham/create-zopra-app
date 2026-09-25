use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use zopra::{component, hooks::use_state, view, cn};

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
            "flex items-center justify-center py-4 px-8 rounded-md border font-semibold cursor-pointer",
            is_even.then_some("bg-white text-black border-transparent"),
            (!is_even).then_some("bg-black text-white border-white"),
        )}
        onClick={move |_, _, cx| set_count(count_val + 1, cx)}>
            {count_text}
        </div>
    }
}


#[component]
pub fn app() {
  view! {
    <div class="flex flex-col items-center justify-center size-full gap-4 bg-[#111] text-white">
      <img src="Zopra.svg" class="size-250"/>
      <div class="text-3xl font-bold">{"Zopra"}</div>
      <Incrementer number={10}/>
      <div>{"Edit src/app.rs to get started."}</div>
      <div>{"Click on the Zopra logo to learn more"}</div>
    </div>
  }
}
