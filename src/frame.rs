use crate::sounds;
use sauron::jss::jss_ns;
use sauron::{
    html::attributes,
    html::{attributes::class, div},
    prelude::*,
    Node,
};
use web_sys::HtmlAudioElement;

const COMPONENT_NAME: &str = "frame";

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    StopAnimation,
    HoverIn,
    HoverOut,
    NextAnimation(f64, f64),
}
pub struct Frame {
    audio: HtmlAudioElement,
    hide: bool,
    hover: bool,
    content: Node<Msg>,
}

impl Frame {
    pub fn new_with_content(content: Node<Msg>) -> Self {
        Frame {
            audio: sounds::preload("sounds/deploy.mp3"),
            hide: false,
            hover: false,
            content,
        }
    }
}

impl Component<Msg, ()> for Frame {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::AnimateIn => {
                self.hide = true;
                Effects::with_local(self.start_animation())
            }
            Msg::StopAnimation => {
                self.hide = false;
                Effects::none()
            }
            Msg::HoverIn => {
                self.hover = true;
                Effects::none()
            }
            Msg::HoverOut => {
                self.hover = false;
                Effects::none()
            }
            Msg::NextAnimation(start, duration) => {
                Effects::with_local(self.next_animation(start, duration))
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let class_ns = |class_names| {
            attributes::class_namespaced(COMPONENT_NAME, class_names)
        };

        let classes_ns_flag = |class_name_flags| {
            attributes::classes_flag_namespaced(
                COMPONENT_NAME,
                class_name_flags,
            )
        };

        div(
            vec![
                class(COMPONENT_NAME),
                classes_ns_flag([
                    ("hide", self.hide),
                    ("expand_corners", true),
                    ("hovered", self.hover),
                ]),
                on_mouseover(|_| Msg::HoverIn),
                on_mouseout(|_| Msg::HoverOut),
            ],
            vec![
                div(vec![class_ns("border border-left")], vec![]),
                div(vec![class_ns("border border-right")], vec![]),
                div(vec![class_ns("border border-top")], vec![]),
                div(vec![class_ns("border border-bottom")], vec![]),
                div(vec![class_ns("corner corner__top-left")], vec![]),
                div(vec![class_ns("corner corner__bottom-left")], vec![]),
                div(vec![class_ns("corner corner__top-right")], vec![]),
                div(vec![class_ns("corner corner__bottom-right")], vec![]),
                div(vec![class_ns("content")], vec![self.content.clone()]),
            ],
        )
    }
}

impl Frame {
    fn start_animation(&mut self) -> Vec<Msg> {
        let duration = 200.0;
        let start = crate::dom::now();
        sounds::play(&self.audio);
        vec![Msg::NextAnimation(start, duration)]
    }

    fn next_animation(&mut self, start: f64, duration: f64) -> Vec<Msg> {
        let timestamp = crate::dom::now();
        let elapsed = timestamp - start;
        let continue_animation = elapsed < duration;
        if continue_animation {
            vec![Msg::NextAnimation(start, duration)]
        } else {
            vec![Msg::StopAnimation]
        }
    }

    pub fn style(&self, theme: &crate::Theme) -> Vec<String> {
        let base = &theme.controls;
        let border_width = 2;

        let css = jss_ns! {COMPONENT_NAME,
            ".": {
                display: "block",
                padding: px(1),
                position: "relative",
                opacity: 1,
            },

            ".border": {
                border_color: base.border_color.clone(),
                box_shadow: format!("{} {} {} {}", 0, 0, px(4), base.border_shadow.clone()),
                z_index: 1,
                opacity: 1,
                position: "absolute",
                transition: format!("all {}ms ease-in", 250),
                border_style: "solid",
            },

            ".hide": {
                opacity: 0,
            },

            ".hide .border": {
                height: 0,
                width: 0,
            },

            ".border-left": {
                top: percent(50),
                left: 0,
                height: percent(100),
                transform: format!("translate({}, {})",0, percent(-50)),
                border_width: format!("{} {} {} {}",0, 0, 0, px(border_width)),
            },

            ".border-right": {
                top: percent(50),
                right: 0,
                height: percent(100),
                transform: format!("translate({}, {})",0, percent(-50)),
                border_width: format!("{} {} {} {}",0, 0, 0, px(border_width)),
            },

            ".border-top": {
                top: 0,
                left: percent(50),
                width: percent(100),
                transform: format!("translate({}, {})", percent(-50), 0),
                border_width: format!("{} {} {} {}",px(border_width), 0, 0, 0),
            },

            ".border-bottom": {
                left: percent(50),
                width: percent(100),
                bottom: 0,
                transform: format!("translate({}, {})",percent(-50), 0),
                border_width: format!("{} {} {} {}", px(border_width), 0, 0, 0),
            },

            ".corner": {
                width: px(24),
                height: px(24),
                border_color: base.corner_color.clone(),
                box_shadow: format!("{} {} {} {} {}", 0, 0, px(4), px(-2), base.corner_shadow.clone()),
                z_index: 2,
                opacity: 1,
                position: "absolute",
                transition: format!("all {}ms ease-in",250),
                border_style: "solid",
            },

            ".hide .corner": {
                width: 0,
                height: 0,
                opacity: 0,
            },

            ".corner__top-left": {
                left: px(-2),
                top: px(-2),
                border_width: format!("{} {} {} {}",px(2), 0, 0, px(2)),
            },

            ".corner__bottom-left": {
                left: px(-2),
                bottom: px(-2),
                border_width: format!("{} {} {} {}",0, 0, px(2), px(2)),
            },

            ".corner__top-right": {
                right: px(-2),
                top: px(-2),
                border_width: format!("{} {} {} {}",px(2), px(2), 0, 0),
            },

            ".corner__bottom-right": {
                right: px(-2),
                bottom: px(-2),
                border_width: format!("{} {} {} {}",0, px(2), px(2), 0),
            },

            ".content": {
                background_color: base.content_background_color.clone(),
                z_index: 3,
                display: "block",
                position: "relative",
                overflow: "hidden",
                transition: format!("background-color {}ms ease-in", 250),
            },

            ".hide .content": {
                background_color: "transparent",
            },

        };

        // if expand_corners is enabled
        // the fui_button corners will EXPAND when hovered.
        //
        // CSS Notes:
        // - `.class1.class2 child` means if both class1 and class2 is specified in the
        // parent, the properties will be applied to this child element
        //
        //  - `.class1,.class2 child` means either if either class1 or class2 is specified in the
        // parent, the properties will be applied to this child element
        //
        let expand_corner_css = jss_ns! {COMPONENT_NAME,
            ".expand_corners.hovered .corner__top-left": {
                left: px(-8),
                top: px(-8),
            },

            ".expand_corners.hovered .corner__bottom-left": {
                left: px(-8),
                bottom: px(-8),
            },

            ".expand_corners.hovered .corner__top-right": {
                right: px(-8),
                top: px(-8),
            },

            ".expand_corners.hovered .corner__bottom-right": {
                right: px(-8),
                bottom: px(-8),
            },
        };

        vec![css, expand_corner_css]
    }
}
