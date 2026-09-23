use gpui_kit::*;
use zopra::{component, hooks::use_state};

#[component]
pub fn app() {
  let (user, set_user) = use_state("Hello User, Welcome to Zopra!");
  set_user("HIII");
  div().child(user())
}
