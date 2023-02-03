use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FooterWrapperProperties {
    pub children: Children,
}

#[styled_component]
pub fn FooterWrapper(properties: &FooterWrapperProperties) -> Html {
    html! {<footer class={css!(r#"color: red;"#)}>{ for properties.children.iter() }</footer>}
}

#[derive(Properties, PartialEq)]
pub struct ProfilePictureProperties {
    pub src: String,
}

#[styled_component]
pub fn ProfilePictureWrapper(properties: &ProfilePictureProperties) -> Html {
    let src = properties.src.clone();
    html! {
        <img class={css!(r#"color: red;"#)} {src}/>
    }
}
