use yew::prelude::*;
#[path = "../footer/mod.rs"]
mod footer;
use crate::app::footer::footer_styles::*;
#[path = "../global_styles/global_styles.rs"]
mod global_styles;
use global_styles::GlobalStyles;

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <GlobalStyles />
            <FooterWrapper>
            {"123"}
            <ProfilePictureWrapper src="/footerTest.png" />
            </FooterWrapper>
        </>
    }
}
