use crate::{animate_list, AnimateList};
use sauron::{
    html::{attributes, div},
    jss::jss_ns,
    prelude::*,
    Node,
};

const COMPONENT_NAME: &str = "image_effects";

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    AnimateListMsg(animate_list::Msg),
    AnimationDone,
}

pub struct Properties {
    url: String,
    width: f32,
    height: f32,
    //slice_size size should be square
    slice_size: f32,
    gap: f32,
}

pub struct ImageEffects {
    animate_list: AnimateList<Msg>,
    properties: Properties,
    is_animating: bool,
}

impl ImageEffects {
    pub fn new(url: impl ToString) -> Self {
        let width = 1000.0;
        let height = 600.0;
        let slice_size = 50.0;
        let gap = 1.0;

        let properties = Properties {
            width,
            height,
            slice_size,
            gap,
            url: url.to_string(),
        };

        let mut animate_list =
            AnimateList::new_with_content(properties.slice_view());
        animate_list.add_stop_animation_listener(|_| Msg::AnimationDone);

        ImageEffects {
            animate_list,
            properties,
            is_animating: false,
        }
    }

    pub fn style(&self, theme: &crate::Theme) -> String {
        self.properties.style(theme)
    }
}

impl Component<Msg, ()> for ImageEffects {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::AnimateIn => {
                self.is_animating = true;
                let effects =
                    self.animate_list.update(animate_list::Msg::AnimateIn);
                effects.merge(Msg::AnimateListMsg)
            }
            Msg::AnimateListMsg(amsg) => {
                log::trace!("Got some msg mean to AnimateList: {:?}", amsg);
                let effects = self.animate_list.update(amsg);
                effects.merge(Msg::AnimateListMsg)
            }
            Msg::AnimationDone => {
                log::trace!("Animation is done...");
                self.is_animating = false;
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
            vec![classes_ns_flag([("animating", self.is_animating)])],
            vec![
                view_if(self.is_animating, self.animate_list.view()),
                img(vec![class_ns("img"), src(&self.properties.url)], vec![]),
            ],
        )
    }
}

impl Properties {
    /// slices on x and slices on y
    fn slices(&self) -> (usize, usize) {
        (
            (self.width / (self.slice_size + self.gap)).round() as usize,
            (self.height / (self.slice_size + self.gap)).round() as usize,
        )
    }

    fn slice_view(&self) -> Node<Msg> {
        let class_ns = |class_names| {
            attributes::class_namespaced(COMPONENT_NAME, class_names)
        };
        let mut cells = vec![];
        let (slice_x, slice_y) = self.slices();
        for y in 0..slice_y {
            let top = (self.slice_size + self.gap) * y as f32;
            for x in 0..slice_x {
                let left = (self.slice_size + self.gap) * x as f32;
                let cell = div(
                    vec![
                        class_ns("slice"),
                        style! {
                            left: px(left),
                            top: px(top),
                            background_position: format!("{} {}", px(-left), px(-top)),
                        },
                    ],
                    vec![],
                );
                cells.push(cell);
            }
        }
        div(vec![class(COMPONENT_NAME)], cells)
    }

    fn style(&self, theme: &crate::Theme) -> String {
        jss_ns! {COMPONENT_NAME,
            ".": {
                position: "relative",
                width: px(self.width),
                height: px(self.height),
            },
            ".img": {
                width: px(self.width),
                height: px(self.height),
                opacity: 1,
            },
            ".animating .img": {
                opacity: 0,
            },
            ".slice": {
              width: px(self.slice_size),
              height: px(self.slice_size),
              background_size: format!("{} {}", px(self.width), px(self.height)),
              position: "absolute",
              background_image: format!("linear-gradient({} 0, {} 25%, {} 75%, {} 100%), url({})"
                  ,theme.background_color, theme.primary_color, theme.accent_color, theme.background_color, self.url),
              background_repeat:"no-repeat no-repeat",
              background_attachment: "local, local",
              background_blend_mode: "color",
            }
        }
    }
}
