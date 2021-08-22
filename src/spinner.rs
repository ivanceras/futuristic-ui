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
                min_height: "90px",
                transition: "all 250ms ease-out",
            },

            ".circle": {
                border_top: format!("5px solid {}", base.border_color),
                border_bottom: format!("5px solid {}",base.border_color),
                box_shadow: format!("0 0 8px {}",base.border_shadow),
                top: "50%",
                left: "50%",
                display: "block",
                position: "absolute",
                transition: "all 250ms ease-out",
                border_left: "5px solid transparent",
                border_right: "5px solid transparent",
                border_radius: "50%",
                background_color: "transparent",
            },

            ".circle1": {
                width: "50px",
                height: "50px",
                animation: "spinner-loading-circle1 750ms infinite linear",
                margin_top: "-25px",
                margin_left: "-25px",
            },

            ".circle2": {
                width: "30px",
                height: "30px",
                animation: "spinner-loading-circle2 750ms infinite linear",
                margin_top: "-15px",
                margin_left: "-15px",
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
