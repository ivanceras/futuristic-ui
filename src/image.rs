use crate::{frame, sounds, Frame};
use sauron::{
    html::{attributes, div},
    jss::jss_ns,
    prelude::*,
    Node,
};
use web_sys::HtmlAudioElement;

const COMPONENT_NAME: &str = "image_effects";

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    FrameMsg(Box<frame::Msg<Msg>>),
    AnimationDone,
    StopAnimation,
    NextAnimation(bool, f64, f64),
}

pub struct Properties {
    url: String,
    width: f32,
    height: f32,
    //slice_size size should be square
    slice_size: f32,
    gap: f32,
}

pub struct Image {
    audio: HtmlAudioElement,
    frame: Frame<Msg>,
    properties: Properties,
    is_animating: bool,
}

impl Image {
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

        Image {
            audio: sounds::preload("sounds/typing.mp3"),
            frame: Frame::with_content(properties.slice_view(None)),
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
impl Component<Msg, ()> for Image {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::AnimateIn => {
                self.is_animating = true;
                Effects::with_local(self.animate_in())
            }
            Msg::FrameMsg(fmsg) => {
                let effects = self.frame.update(*fmsg);
                effects.localize(|fmsg| Msg::FrameMsg(Box::new(fmsg)))
            }
            Msg::AnimationDone => {
                self.is_animating = false;
                Effects::none()
            }
            Msg::StopAnimation => {
                self.stop_animation();
                Effects::none()
            }
            Msg::NextAnimation(is_in, start, duration) => {
                let follow_ups = self.next_animation(is_in, start, duration);
                Effects::with_local(follow_ups)
            }
        }
    }

    fn view(&self) -> Node<Msg> {
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
                //on_mouseout(|_| Msg::AnimateIn),
            ],
            vec![self
                .frame
                .view()
                .map_msg(|fmsg| Msg::FrameMsg(Box::new(fmsg)))],
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

    fn content_len(&self) -> usize {
        let (w, h) = self.slices();
        w * h
    }

    fn slice_view(&self, limit: Option<usize>) -> Node<Msg> {
        let class_ns = |class_names| {
            attributes::class_namespaced(COMPONENT_NAME, class_names)
        };
        let mut cells = vec![];
        let (slice_x, slice_y) = self.slices();
        let max = slice_x * slice_y;
        let limit = if let Some(limit) = limit { limit } else { max };
        let mut index = 0;
        for y in 0..slice_y {
            let top = (self.slice_size + self.gap) * y as f32;
            for x in 0..slice_x {
                if index < limit {
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
                index += 1;
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
                position: "relative",
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

impl Image {
    pub fn animate_in(&mut self) -> Vec<Msg> {
        sounds::play(&self.audio);
        self.start_animation(true)
    }

    fn stop_animation(&mut self) -> Vec<Msg> {
        self.is_animating = false;
        let class_ns = |class_names| {
            attributes::class_namespaced(COMPONENT_NAME, class_names)
        };
        self.frame.set_content(div(vec![class_ns("img")], vec![]));
        vec![]
    }

    fn content_len(&self) -> usize {
        self.properties.content_len()
    }

    fn start_animation(&mut self, is_in: bool) -> Vec<Msg> {
        if self.content_len() == 0 {
            return vec![];
        }

        let interval = 1_000.0 / 60.0;
        let real_duration = interval * self.content_len() as f64;
        let timeout = 500.0;
        let duration = real_duration.min(timeout);
        let start = crate::dom::now();

        self.is_animating = true;

        vec![Msg::NextAnimation(is_in, start, duration)]
    }

    fn next_animation(
        &mut self,
        is_in: bool,
        start: f64,
        duration: f64,
    ) -> Vec<Msg> {
        let timestamp = crate::dom::now();

        let mut anim_progress = (timestamp - start).max(0.0);
        if !is_in {
            anim_progress = duration - anim_progress;
        }

        let new_length = (anim_progress * self.content_len() as f64 / duration)
            .round() as usize;

        let continue_animation = if is_in {
            new_length <= (self.content_len() - 1)
        } else {
            new_length > 0
        };

        if continue_animation {
            self.frame
                .set_content(self.properties.slice_view(Some(new_length)));
            vec![Msg::NextAnimation(is_in, start, duration)]
        } else {
            vec![Msg::StopAnimation]
        }
    }
}
