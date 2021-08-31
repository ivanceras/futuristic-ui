use crate::sounds;
use sauron::jss::jss_ns;
use sauron::{
    html::attributes,
    html::{attributes::class, div, text},
    prelude::*,
    Node,
};
use web_sys::HtmlAudioElement;

const COMPONENT_NAME: &str = "nav_header";

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    StopAnimation,
    NextAnimation(f64, f64),
}

pub struct NavHeader {
    audio: HtmlAudioElement,
    hide: bool,
    content: String,
}

impl NavHeader {
    pub fn with_content(content: &str) -> Self {
        NavHeader {
            audio: sounds::preload("sounds/deploy.mp3"),
            hide: false,
            content: content.to_string(),
        }
    }
}

impl Component<Msg, ()> for NavHeader {
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
        header(
            vec![
                class(COMPONENT_NAME),
                classes_ns_flag([("hide", self.hide)]),
            ],
            vec![div(
                vec![class_ns("content_and_relief")],
                vec![
                    div(
                        vec![class_ns("text text-anim")],
                        vec![
                            self.child(),
                            div(vec![class_ns("border border-bottom")], vec![]),
                        ],
                    ),
                    div(
                        vec![class_ns("link_content")],
                        vec![div(
                            vec![class_ns("link")],
                            vec![a(
                                vec![href("#readmore")],
                                vec![text("Read more..")],
                            )],
                        )],
                    ),
                ],
            )],
        )
    }
}

impl NavHeader {
    fn child(&self) -> Node<Msg> {
        div(vec![], vec![text(&self.content)])
    }

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

    pub fn style(theme: &crate::Theme) -> String {
        let base = &theme;
        jss_ns! {COMPONENT_NAME,
            ".": {
                display: "block",
                padding: "1px",
                position: "relative",
                opacity: 1,
                color: base.secondary_color.clone(),
                font_family: base.primary_font.clone(),
            },

            ".content_and_relief": {
                width: percent(100),
                display: "flex",
            },

            ".hide": {
                opacity: 0,
            },

            ".text": {
                white_space: "nowrap",
                width: percent(100),
            },

            ".border": {
                border_color: base.controls.corner_color.clone(),
                box_shadow: format!("{} {} {} {}",0, 0, px(4), base.controls.border_shadow.clone()),
                z_index: 1,
                opacity: 1,
                position: "relative",
                transition: format!("all {}ms ease-in",250),
                border_style: "solid",
            },

            ".hide .border": {
              height: 0,
              width: 0,
            },

            ".border-bottom": {
                left: percent(50),
                width: percent(100),
                height: 0,
                bottom: 0,
                transform: format!("translate({}, {})",percent(-50), 0),
                border_width: format!("{} {} {} {}", px(2), 0, 0, 0),
            },

            ".text-anim": {
                color: base.accent_color.clone(),
                transition: format!("color {}ms ease-out", 250),
                font_family: base.secondary_font.clone(),
                text_shadow: format!("{} {} {} {}",0, 0, px(4), base.accent_shadow.clone()),
            },

            ".link_content": {
                transform: format!("skewX({}deg)",-45),
                border_color: base.controls.corner_color.clone(),
                border_style: "solid",
                border_width: format!("{} {} {} {}",px(2), 0, 0, px(16)),
                position: "relative",
            },

            ".link_content a": {
                font_size: px(12),
            },

            ".link": {
                padding_left: px(20),
                margin_top: px(10),
                transform: format!("skewX({}deg)", 45),
                white_space: "nowrap",
            }

        }
    }
}
