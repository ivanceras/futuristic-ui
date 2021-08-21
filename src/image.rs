use crate::{frame, Frame};
use sauron::{
    html::{
        attributes::{empty_attr, title},
        div,
    },
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
}

impl Image {
    pub fn new(url: impl ToString, img_title: Option<impl ToString>) -> Self {
        Image {
            frame: Frame::new_with_content(img(
                vec![
                    src(url.to_string()),
                    if let Some(img_title) = img_title {
                        title(img_title.to_string())
                    } else {
                        empty_attr()
                    },
                ],
                vec![],
            )),
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
