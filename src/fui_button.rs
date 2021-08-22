use crate::sounds;
use sauron::jss::jss_ns;
use sauron::{
    dom::Callback,
    html::attributes,
    html::{attributes::class, div, events::on_click, text},
    prelude::*,
    Node,
};
use web_sys::HtmlAudioElement;
use web_sys::MouseEvent;

const COMPONENT_NAME: &str = "fui_button";

#[derive(Clone, Debug)]
pub enum Msg {
    Click(MouseEvent),
    HoverIn,
    HoverOut,
    HighlightEnd,
}

pub struct FuiButton<PMSG> {
    audio: HtmlAudioElement,
    options: Options,
    label: String,
    click: bool,
    hover: bool,
    click_listeners: Vec<Callback<MouseEvent, PMSG>>,
}

pub struct Options {
    pub hidden: bool,
    /// enable sound
    pub sound: bool,
    /// enable click effect, which changes the background color
    /// of the button with the highlight color
    pub click_highlights: bool,
    /// the button is slanted 45 degree to the right
    pub skewed: bool,
    /// has corners
    pub has_corners: bool,
    /// enable/disable hover effect
    pub has_hover: bool,
    /// expand corners when hovered
    pub expand_corners: bool,
    /// the button is disabled
    pub disabled: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl<PMSG> FuiButton<PMSG>
where
    PMSG: 'static,
{
    pub fn new_with_label(label: &str) -> Self {
        let options = Options::regular();
        FuiButton {
            audio: sounds::preload("sounds/click.mp3"),
            options,
            click: false,
            hover: false,
            label: label.to_string(),
            click_listeners: vec![],
        }
    }
}

impl<PMSG> Component<Msg, PMSG> for FuiButton<PMSG>
where
    PMSG: 'static,
{
    fn update(&mut self, msg: Msg) -> Effects<Msg, PMSG> {
        match msg {
            Msg::Click(mouse_event) => {
                if self.options.sound {
                    sounds::play(&self.audio);
                }
                self.click = true;
                let pmsg_list = self
                    .click_listeners
                    .iter()
                    .map(|listener| listener.emit(mouse_event.clone()))
                    .collect();
                Effects::with_effects(pmsg_list)
            }
            Msg::HoverIn => {
                self.hover = true;
                Effects::none()
            }
            Msg::HoverOut => {
                self.hover = false;
                Effects::none()
            }
            Msg::HighlightEnd => {
                self.click = false;
                Effects::none()
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
                    ("clicked", self.click),
                    ("click_highlights", self.options.click_highlights),
                    ("expand_corners", self.options.expand_corners),
                    ("has_hover", self.options.has_hover),
                    ("hovered", self.hover),
                    ("skewed", self.options.skewed),
                    // setting this will also disable the div, therefore will not activate the
                    // events on it
                    ("disabled", self.options.disabled),
                    ("hidden", self.options.hidden),
                ]),
                // normally click should be attached to the actual button element
                on_click(Msg::Click),
                // the mouseover events are attached here since the hover element z-index is
                // higher than the actual button, which will cause a janky animation
                // when the mouse is triggering alt hover in and out, since covered by the hover
                // layer effect
                on_mouseover(|_| Msg::HoverIn),
                on_mouseout(|_| Msg::HoverOut),
            ],
            vec![
                // hover
                view_if(
                    self.options.has_hover,
                    div(vec![class_ns("hover hover-bottom")], vec![]),
                ),
                //borders
                div(vec![class_ns("border border-bottom")], vec![]),
                div(vec![class_ns("border border-left")], vec![]),
                div(vec![class_ns("border border-right")], vec![]),
                div(vec![class_ns("border border-top")], vec![]),
                div(vec![class_ns("border border-bottom")], vec![]),
                // corners
                view_if(
                    self.options.has_corners,
                    div(vec![class_ns("corner corner__top-left")], vec![]),
                ),
                view_if(
                    self.options.has_corners,
                    div(vec![class_ns("corner corner__bottom-left")], vec![]),
                ),
                view_if(
                    self.options.has_corners,
                    div(vec![class_ns("corner corner__top-right")], vec![]),
                ),
                view_if(
                    self.options.has_corners,
                    div(vec![class_ns("corner corner__bottom-right")], vec![]),
                ),
                div(
                    vec![],
                    vec![
                        div(
                            vec![class_ns("button_wrap")],
                            vec![button(
                                vec![
                                    class_ns("button"),
                                    disabled(self.options.disabled),
                                    maybe_attr("width", self.options.width),
                                    maybe_attr("height", self.options.height),
                                ],
                                vec![text(&self.label)],
                            )],
                        ),
                        div(
                            vec![
                                class_ns("highlight"),
                                on_transitionend(|_| Msg::HighlightEnd),
                            ],
                            vec![],
                        ),
                    ],
                ),
            ],
        )
    }
}

impl<PMSG> FuiButton<PMSG>
where
    PMSG: 'static,
{
    pub fn set_options(&mut self, options: Options) {
        self.options = options;
    }

    pub fn add_click_listener<F>(&mut self, f: F)
    where
        F: Fn(MouseEvent) -> PMSG + 'static,
    {
        let cb = Callback::from(f);
        self.click_listeners.push(cb);
    }

    pub fn style(&self, theme: &crate::Theme) -> Vec<String> {
        let base = &theme.controls;

        let base_css = jss_ns! {COMPONENT_NAME,

            // the ROOT component style
            ".": {
                display: "inline-block",
                padding: px(1),
                position: "relative",
                margin: format!("{} {}",px(10), px(10)),
            },

            ".hidden" : {
                visibility: "hidden",
            },

            // HOVER at the lower  part of the button
            ".hover": {
                border_color: base.hover_color.clone(),
                box_shadow: format!("{} {} {} {} {}", 0, px(-2), px(4), 0, base.hover_shadow.clone()),
                z_index: 4,
                opacity: 1,
                position: "absolute",
                transition: format!("width {}ms ease-in",100),
                border_style: "solid",
            },

            ".has_hover.hovered .hover": {
                width: percent(96),
            },

            ".hover-bottom": {
                width: 0,
                left: percent(50),
                bottom: px(2),
                transform: format!("translate({}, {})",percent(-50), 0),
                border_width: format!("{} {} {} {}", px(4), 0, 0, 0),
            },


            // BORDERS these are styled divs wrapping the buttons
            ".border": {
                border_color: base.border_color.clone(),
                box_shadow: format!("{} {} {} {}",0, 0, px(4), base.border_shadow.clone()),
                z_index: 1,
                opacity: 1,
                position: "absolute",
                transition: format!("all {}ms ease-in",250),
                border_style: "solid",
            },


            ".border-left": {
                top: percent(50),
                left: 0,
                height: percent(100),
                transform: format!("translate({}, {})", 0,percent(-50)),
                border_width: "0 0 0 1px",
            },

            ".border-right": {
                top: percent(50),
                right: 0,
                height: percent(100),
                transform: format!("translate({}, {})",0,percent(-50)),
                border_width: "0 0 0 1px",
            },

            ".border-top": {
                top: 0,
                left: percent(50),
                width: percent(100),
                transform: format!("translate({}, {})",percent(-50), 0),
                border_width: "1px 0 0 0",
            },

            ".border-bottom": {
                left: percent(50),
                width: percent(100),
                bottom: 0,
                transform: format!("translate({}, {})",percent(-50), 0),
                border_width: format!("{} {} {} {}",px(1), 0, 0, 0),
            },

            // CORNERS - the fancy divs which clips the button
            ".corner": {
                width: px(8),
                height: px(8),
                border_color: base.corner_color.clone(),
                box_shadow: format!("{} {} {} {} {}",0, 0, px(4), px(-2), base.corner_shadow.clone()),
                z_index: 2,
                opacity: 1,
                position: "absolute",
                transition: format!("all {}ms ease-in",250),
                border_style: "solid",
            },

            ".corner__top-left": {
                left: px(-2),
                top: px(-2),
                border_width: format!("{} {} {} {}", px(2), 0, 0, px(2)),
            },

            ".corner__bottom-left": {
                left: px(-2),
                bottom: px(-2),
                border_width: format!("{} {} {} {}", 0, 0, px(2), px(2)),
            },

            ".corner__top-right": {
                right: px(-2),
                top: px(-2),
                border_width: format!("{} {} {} {}", px(2), px(2), 0, 0),
            },

            ".corner__bottom-right": {
                right: px(-2),
                bottom: px(-2),
                border_width: format!("{} {} {} {}", 0, px(2), px(2), 0),
            },

            ".button_wrap": {
                background_color: base.content_background_color.clone(),
                z_index: 3,
                display: "block",
                position: "relative",
                overflow: "hidden",
                transition: format!("background-color {}ms ease-in", 250),
            },

            // The actual button
            ".button": {
                color: base.button_text_color.clone(),
                cursor: "pointer",
                margin: 0,
                border: "none",
                z_index: 2,
                display: "inline-block",
                padding: format!("{} {}", px(10), px(20)),
                outline: "none",
                position: "relative",
                font_size: px(15.75),
                background: "transparent",
                transition: format!("all {}ms ease-out", 250),
                line_height: 1,
                user_select: "none",
                vertical_align: "middle",
            },

            // highlight when clicked and fades out shortly
            ".highlight": {
                  z_index: 1,
                  position: "absolute",
                  left: 0,
                  right: 0,
                  top: 0,
                  bottom: 0,
                  background_color: base.highlight_color.clone(),
                  opacity: 0,
                  transition: format!("all {}ms ease-out", 50),
            },

            ".clicked .highlight": {
                opacity: 1,
            },

        };

        let skewed_css = jss_ns! {COMPONENT_NAME,
            ".skewed": {
                transform: format!("skewX({}deg)", -45),
                transform_origin: "bottom left",
            },

            ".skewed .button": {
                transform: format!("skewX({}deg)", 45),
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
                left: px(-6),
                top: px(-6),
            },

            ".expand_corners.hovered .corner__bottom-left": {
                left: px(-6),
                bottom: px(-6),
            },

            ".expand_corners.hovered .corner__top-right": {
                right: px(-6),
                top: px(-6),
            },

            ".expand_corners.hovered .corner__bottom-right": {
                right: px(-6),
                bottom: px(-6),
            },
        };

        vec![base_css, skewed_css, expand_corner_css]
    }
}

impl Options {
    /// bare minimum button
    /// no sound
    #[allow(unused)]
    pub fn bare() -> Self {
        Options {
            sound: false,
            click_highlights: false,
            skewed: false,
            has_corners: false,
            expand_corners: false,
            has_hover: false,
            disabled: false,
            hidden: false,
            width: None,
            height: None,
        }
    }

    /// full effect, skewed
    #[allow(unused)]
    pub fn full() -> Self {
        Options {
            sound: true,
            click_highlights: true,
            skewed: true,
            has_corners: true,
            expand_corners: true,
            has_hover: true,
            disabled: false,
            hidden: false,
            width: None,
            height: None,
        }
    }

    /// regular futuristic button
    #[allow(unused)]
    pub fn regular() -> Self {
        Options {
            sound: true,
            click_highlights: true,
            skewed: false,
            has_corners: true,
            expand_corners: true,
            has_hover: true,
            disabled: false,
            hidden: false,
            width: None,
            height: None,
        }
    }

    /// just like regular but muted
    /// sound off
    #[allow(unused)]
    pub fn muted() -> Self {
        Options {
            sound: false,
            click_highlights: true,
            skewed: false,
            has_corners: true,
            expand_corners: true,
            has_hover: true,
            disabled: false,
            hidden: false,
            width: None,
            height: None,
        }
    }

    /// no corners, no hover
    #[allow(unused)]
    pub fn simple() -> Self {
        Options {
            sound: true,
            click_highlights: true,
            skewed: false,
            has_corners: false,
            expand_corners: false,
            has_hover: false,
            disabled: false,
            hidden: false,
            width: None,
            height: None,
        }
    }

    ///does not interact
    #[allow(unused)]
    pub fn disabled() -> Self {
        Options {
            sound: false,
            click_highlights: false,
            skewed: false,
            has_corners: false,
            expand_corners: false,
            has_hover: false,
            disabled: true,
            hidden: false,
            width: None,
            height: None,
        }
    }

    pub fn skewed(mut self, skewed: bool) -> Self {
        self.skewed = skewed;
        self
    }

    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
}
