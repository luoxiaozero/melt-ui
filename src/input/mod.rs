mod theme;
use crate::{
    theme::{use_theme, Theme},
    utils::mount_style::mount_style,
};
use leptos::*;
use stylers::style_sheet_str;
pub use theme::InputTheme;

#[derive(Default, Clone)]
pub enum InputType {
    #[default]
    TEXT,
    PASSWORD,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::TEXT => "text",
            InputType::PASSWORD => "password",
        }
    }
}

#[component]
pub fn Input(
    cx: Scope,
    #[prop(into)] value: RwSignal<String>,
    #[prop(optional, into)] type_: MaybeSignal<InputType>,
) -> impl IntoView {
    let theme = use_theme(cx, Theme::light);
    let class_name = mount_style("input", || style_sheet_str!("./src/input/input.css"));

    let input_ref = create_node_ref::<html::Input>(cx);
    input_ref.on_load(cx, move |input| {
        input.on(ev::input, move |ev| {
            value.set(event_target_value(&ev));
        });
    });

    let css_vars = create_memo(cx, move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let border_color_hover = theme.common.color_primary.clone();
        css_vars.push_str(&format!("--border-color-hover: {border_color_hover};"));
        let border_radius = theme.common.border_radius.clone();
        css_vars.push_str(&format!("--border-radius: {border_radius};"));
        css_vars
    });
    view! {
        cx, class=class_name,
        <div class:melt-input=true style=move || css_vars.get()>
            <input type=move || type_.get().as_str() prop:value=move || value.get() ref=input_ref class="melt-input__input-el"/>
        </div>
    }
}
