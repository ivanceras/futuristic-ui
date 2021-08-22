use sauron::jss::jss_ns;
use sauron::{
    html::attributes,
    html::{attributes::class, div},
    prelude::*,
    Node,
};
use std::marker::PhantomData;

const COMPONENT_NAME: &str = "spinner";

#[derive(Clone)]
pub struct Spinner<MSG> {
    _phantom: PhantomData<MSG>,
}

impl<MSG> Spinner<MSG> {
    pub fn new() -> Self {
        Spinner {
            _phantom: PhantomData,
        }
    }
}

impl<MSG> View<MSG> for Spinner<MSG> {
    fn view(&self) -> Node<MSG> {
        let class_ns = |class_names| {
            attributes::class_namespaced(COMPONENT_NAME, class_names)
        };

        div(
            vec![class(COMPONENT_NAME)],
            vec![
                div(vec![class_ns("circle circle1")], vec![]),
                div(vec![class_ns("circle circle2")], vec![]),
            ],
        )
    }
}

impl<MSG> Spinner<MSG> {
    pub fn style(&self, theme: &crate::Theme) -> Vec<String> {
        let base = &theme.controls;

        let base_css = jss_ns! {COMPONENT_NAME,
            ".": {
                top: 0,
                left: 0,
                right: 0,
                bottom: 0,
                position: "absolute",
                z_index: 1000,
                display: "block",
                opacity: 1,
                position: "relative",
                min_height: px(90),
                transition: "all 250ms ease-out",
            },

            ".circle": {
                border_top: format!("{} solid {}", px(5), base.border_color),
                border_bottom: format!("{} solid {}",px(5), base.border_color),
                box_shadow: format!("{} {} {} {}",0, 0, px(8), base.border_shadow),
                top: percent(50),
                left: percent(50),
                display: "block",
                position: "absolute",
                transition: format!("all {}ms ease-out", 250),
                border_left: format!("{} solid transparent", px(5)),
                border_right: format!("{} solid transparent", px(5)),
                border_radius: percent(50),
                background_color: "transparent",
            },

            ".circle1": {
                width: px(50),
                height: px(50),
                animation: format!("spinner-loading-circle1 {}ms infinite linear", 750),
                margin_top: px(-25),
                margin_left: px(-25),
            },

            ".circle2": {
                width: px(30),
                height: px(30),
                animation: format!("spinner-loading-circle2 {}ms infinite linear", 750),
                margin_top: px(-15),
                margin_left: px(-15),
            },

            "@keyframes spinner-loading-circle1": {
              "0%": {
                transform: "rotate(160deg)",
                opacity: 0,
              },

              "50%": {
                transform: "rotate(145deg)",
                opacity: 1,
              },

              "100%": {
                transform: "rotate(-320deg)",
                opacity: 0,
              },
            },

            "@keyframes spinner-loading-circle2": {
              "0%": {
                transform: "rotate(0deg)",
              },

              "100%": {
                transform: "rotate(360deg)",
              },
            },

        };

        vec![base_css]
    }
}
