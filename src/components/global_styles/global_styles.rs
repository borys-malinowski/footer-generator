use stylist::yew::{styled_component, Global};
use yew::prelude::*;

#[styled_component]
pub fn GlobalStyles() -> Html {
    html! {<Global css={css!(r#"
    *,
    *::after,
    *::before {
      box-sizing: border-box;
      margin: 0;
    }
    body {
      font-family: "Roboto", sans-serif;
      -webkit-font-smoothing: antialiased;
      -moz-osx-font-smoothing: grayscale;
      background-color: gray;
    }
    "#)}/>}
}
