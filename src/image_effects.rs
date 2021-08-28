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
        let slice_size = 40.0;
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

///TODO: create a copy of AnimateList effects here, so we can customize it specific to the image.
/// Wrap this with frame as well
impl Component<Msg, ()> for ImageEffects {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::AnimateIn => {
                self.is_animating = true;
                let effects =
                    self.animate_list.update(animate_list::Msg::AnimateIn);
                effects.localize(Msg::AnimateListMsg)
            }
            Msg::AnimateListMsg(amsg) => {
                let effects = self.animate_list.update(amsg);
                effects.localize(Msg::AnimateListMsg)
            }
            Msg::AnimationDone => {
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
            vec![
                class(COMPONENT_NAME),
                classes_ns_flag([("animating", self.is_animating)]),
                on_mouseout(|_| Msg::AnimateIn),
            ],
            vec![
                view_if(self.is_animating, self.animate_list.view()),
                div(vec![class_ns("img")], vec![]),
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
        div(vec![class_ns("effects_slices")], cells)
    }

    fn style(&self, theme: &crate::Theme) -> String {
        jss_ns! {COMPONENT_NAME,
            ".": {
                display: "inline-block",
                width: px(self.width),
                height: px(self.height),
                position: "relative",
            },
            ".effects_slices": {
                display: "inline-block",
                width: px(self.width),
                height: px(self.height),
                position: "relative",
            },
            ".img": {
                width: px(self.width),
                height: px(self.height),
                position: "absolute",
                opacity: 1,
                background_size: format!("{} {}", px(self.width), px(self.height)),
                background_image: format!("linear-gradient({} 0, {} 25%, {} 75%, {} 100%), url({})"
                        ,theme.background_color, theme.primary_color, theme.accent_color, theme.background_color, self.url),
                background_blend_mode: "color",
            },
            ".animating .img": {
                opacity: 0,
            },
            ".slice": {
                  width: px(self.slice_size),
                  height: px(self.slice_size),
                  position: "absolute",
                  background_size: format!("{} {}", px(self.width), px(self.height)),
                  background_image: format!("linear-gradient({} 0, {} 25%, {} 75%, {} 100%), url({})"
                      ,theme.background_color, theme.primary_color, theme.accent_color, theme.background_color, self.url),
                  background_repeat:"no-repeat no-repeat",
                  background_attachment: "local, local",
                  background_blend_mode: "color",
            }
        }
    }
}
