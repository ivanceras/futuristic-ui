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

const COMPONENT_NAME: &str = "image_effects";

#[derive(Clone, Debug)]
pub enum Msg {
    FrameMsg(frame::Msg),
}

pub struct ImageEffects {
    frame: Frame,
    url: String,
    width: f32,
    height: f32,
    //slice_size size should be square
    slice_size: f32,
    gap: f32,
}

impl ImageEffects {
    pub fn new(url: impl ToString) -> Self {
        let width = 1000.0;
        let height = 600.0;
        let slice_size = 50.0;
        let gap = 4.0;

        let class_ns = |class_names| {
            attributes::class_namespaced(COMPONENT_NAME, class_names)
        };
        ImageEffects {
            frame: Frame::new_with_content(div(
                vec![class(COMPONENT_NAME)],
                vec![img(vec![class_ns("img"), src(url.to_string())], vec![])],
            )),
            url: url.to_string(),
            width,
            height,
            slice_size,
            gap,
        }
    }

    /// slices on x and slices on y
    fn slices(&self) -> (usize, usize) {
        (
            (self.width / self.slice_size).round() as usize,
            (self.height / self.slice_size).round() as usize,
        )
    }
}

impl Component<Msg, ()> for ImageEffects {
    fn update(&mut self, _: Msg) -> Effects<Msg, ()> {
        Effects::none()
    }

    fn view(&self) -> Node<Msg> {
        let class_ns = |class_names| {
            attributes::class_namespaced(COMPONENT_NAME, class_names)
        };
        let mut cells = vec![];
        let (slice_x, slice_y) = self.slices();
        for y in 0..slice_y {
            let top = (self.slice_size + self.gap) * y as f32;
            let bg_y = -(self.slice_size * y as f32);
            for x in 0..slice_x {
                let left = (self.slice_size + self.gap) * x as f32;
                let bg_x = -(self.slice_size * x as f32);
                let cell = div(
                    vec![
                        class_ns("slice"),
                        style! {
                            left: px(left),
                            top: px(top),
                            background_position: format!("{} {}", px(bg_x), px(bg_y)),
                        },
                    ],
                    vec![],
                );
                cells.push(cell);
            }
        }
        div(vec![class(COMPONENT_NAME)], cells)
    }

    fn style(&self) -> String {
        jss_ns! {COMPONENT_NAME,
            ".": {
                position: "relative",
                width: px(self.width),
                height: px(self.height),
                margin: px(10),
            },
            ".slice": {
              width: px(self.slice_size),
              height: px(self.slice_size),
              background_size: format!("{} {}", px(self.width), px(self.height)),
              position: "absolute",
              background_image: format!("url({})","img/space.jpg"),
              background_repeat:"no-repeat no-repeat",
              background_attachment: "local, local",
            }
        }
    }
}
