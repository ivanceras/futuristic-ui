use crate::{frame, Frame};
use sauron::{
    html::{
        attributes,
        attributes::{empty_attr, title},
        div,
    },
    jss::jss_ns,
    prelude::*,
    Node,
};

const COMPONENT_NAME: &str = "image";

#[derive(Clone, Debug)]
pub enum Msg {
    FrameMsg(frame::Msg),
}

pub struct Image {
    frame: Frame,
    url: String,
}

impl Image {
    pub fn new(url: impl ToString, img_title: Option<impl ToString>) -> Self {
        let class_ns = |class_names| {
            attributes::class_namespaced(COMPONENT_NAME, class_names)
        };
        Image {
            url: url.to_string(),
            frame: Frame::new_with_content(div(
                vec![class(COMPONENT_NAME)],
                vec![img(
                    vec![
                        class_ns("img"),
                        src(url.to_string()),
                        if let Some(img_title) = img_title {
                            title(img_title.to_string())
                        } else {
                            empty_attr()
                        },
                    ],
                    vec![],
                )],
            )),
        }
    }

    pub fn style(&self, theme: &crate::Theme) -> String {
        jss_ns! {COMPONENT_NAME,
            ".": {
                background_image: format!("linear-gradient({} 0, {} 25%, {} 75%, {} 100%), url({})"
                        ,theme.background_color, theme.primary_color, theme.accent_color, theme.background_color, self.url),
                background_size: format!("{} auto",percent(100)),
                background_position: "center",
                background_repeat: "no-repeat",
                background_blend_mode: "color",
            },
            // hide the actual image, and show only the modified one with blended colors
            ".img": {
                opacity: 0,
                width: percent(100),
                max_width: percent(100),
                height: "auto",
            }

        }
    }
}

impl Component<Msg, ()> for Image {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::FrameMsg(fmsg) => {
                let effects = self.frame.update(fmsg);
                effects.map_msg(Msg::FrameMsg)
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![class(COMPONENT_NAME)],
            vec![self.frame.view().map_msg(Msg::FrameMsg)],
        )
    }
}
