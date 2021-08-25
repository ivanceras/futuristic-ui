use crate::sounds;
use sauron::{
    html::{attributes::class, div, text},
    jss::jss,
    prelude::*,
    Node,
};
use web_sys::HtmlAudioElement;

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    StopAnimation,
    NextAnimation(bool, f64, f64),
}

pub struct AnimateList<PMSG> {
    audio: HtmlAudioElement,
    animated_layer: Option<Node<PMSG>>,
    children: Node<PMSG>,
    animating: bool,
    content_len: usize,
    /// these are listeners that will be called when the anination is done
    on_stop_animation: Vec<Callback<(), PMSG>>,
}

impl<PMSG> AnimateList<PMSG>
where
    PMSG: Clone,
{
    pub fn new_with_content(children: Node<PMSG>) -> Self {
        let content_len = children.node_count();
        AnimateList {
            audio: sounds::preload("sounds/typing.mp3"),
            animating: false,
            animated_layer: None,
            children,
            content_len,
            on_stop_animation: vec![],
        }
    }
}

impl<PMSG> Container<Msg, PMSG> for AnimateList<PMSG>
where
    PMSG: Clone,
{
    fn update(&mut self, msg: Msg) -> Effects<Msg, PMSG> {
        match msg {
            Msg::AnimateIn => Effects::with_follow_ups(self.animate_in()),
            Msg::StopAnimation => {
                log::trace!("animation is stopped in animate list...");
                log::trace!(
                    "There are follow ups? : {:#?}",
                    self.on_stop_animation
                );
                self.stop_animation();
                let pmsg_list = self
                    .on_stop_animation
                    .iter()
                    .map(|listener| listener.emit(()))
                    .collect();
                Effects::with_effects(pmsg_list)
            }
            Msg::NextAnimation(is_in, start, duration) => {
                let follow_ups = self.next_animation(is_in, start, duration);
                Effects::with_follow_ups(follow_ups)
            }
        }
    }

    // Note: opacity: 0 on span will have no effect on webkit browser
    // however, it has an effect on firefox
    fn view(&self) -> Node<PMSG> {
        div(
            vec![],
            vec![div(
                vec![
                    class("animate_list"),
                    classes_flag([("animating", self.animating)]),
                ],
                vec![
                    div(
                        vec![class("animate_list_children")],
                        vec![self.children.clone()],
                    ),
                    view_if(
                        self.animating,
                        div(
                            vec![class("animated_layer_wrapper")],
                            vec![div(
                                vec![class("animated_layer")],
                                if let Some(animated_layer) =
                                    &self.animated_layer
                                {
                                    vec![animated_layer.clone()]
                                } else {
                                    vec![]
                                },
                            )],
                        ),
                    ),
                ],
            )],
        )
    }
}

impl<PMSG> AnimateList<PMSG>
where
    PMSG: Clone,
{
    pub fn animate_in(&mut self) -> Vec<Msg> {
        sounds::play(&self.audio);
        self.start_animation(true)
    }

    fn stop_animation(&mut self) -> Vec<Msg> {
        self.animating = false;
        vec![]
    }

    pub fn add_stop_animation_listener<F>(&mut self, f: F)
    where
        F: Fn(()) -> PMSG + 'static,
    {
        let cb = Callback::from(f);
        log::trace!("Adding a stop animation listener here..");
        self.on_stop_animation.push(cb);
    }

    fn start_animation(&mut self, is_in: bool) -> Vec<Msg> {
        if self.content_len == 0 {
            return vec![];
        }

        let interval = 1_000.0 / 60.0;
        let real_duration = interval * self.content_len as f64;
        let timeout = 500.0;
        let duration = real_duration.min(timeout);
        let start = crate::dom::now();

        self.animating = true;
        if is_in {
            self.animated_layer = None;
        }

        vec![Msg::NextAnimation(is_in, start, duration)]
    }

    /// include the the element from the src to dest
    /// as long as its current_cnt is less than the chars_limit
    fn include_node(
        dest: &mut Node<PMSG>,
        src: &Node<PMSG>,
        chars_limit: usize,
    ) {
        let mut current_cnt = 0;
        Self::include_node_recursive(dest, src, chars_limit, &mut current_cnt);
    }

    /// recursively include the element from src to dest
    /// until all of the current_cnt that is lesser than chars_limit is added.
    fn include_node_recursive(
        dest: &mut Node<PMSG>,
        src: &Node<PMSG>,
        chars_limit: usize,
        current_cnt: &mut usize,
    ) {
        match src {
            Node::Element(element) => {
                if *current_cnt < chars_limit {
                    let shallow_src = html_element(
                        element.tag,
                        element.attrs.clone(),
                        vec![],
                    );
                    dest.add_children_ref_mut(vec![shallow_src]);
                    let children_len = element.children.len();
                    let truncate_len = if chars_limit > *current_cnt {
                        std::cmp::min(children_len, chars_limit - *current_cnt)
                    } else {
                        0
                    };

                    let last_index = dest
                        .as_element_ref()
                        .expect("this is an element")
                        .children
                        .len()
                        - 1;

                    let mut just_added_child = dest
                        .children_mut()
                        .expect("must have children, since just added 1")
                        .get_mut(last_index)
                        .expect("must get the last child");

                    for child in &element.children[0..truncate_len] {
                        Self::include_node_recursive(
                            &mut just_added_child,
                            child,
                            chars_limit,
                            current_cnt,
                        );
                    }
                    *current_cnt += truncate_len;
                }
            }
            Node::Text(txt) => {
                let txt_len = txt.text.len();
                let truncate_len = if chars_limit > *current_cnt {
                    std::cmp::min(txt_len, chars_limit - *current_cnt)
                } else {
                    0
                };

                if truncate_len > 0 {
                    let truncated_txt = &txt.text[0..truncate_len];
                    let text_node = Node::Text(Text::new(truncated_txt));
                    dest.add_children_ref_mut(vec![text_node]);
                    // we append the blinking character to the end of the text
                    // here, and only when this node has not yet finish animating..
                    if truncate_len < txt_len {
                        let blink = span(vec![class("blink")], vec![text("█")]);
                        dest.add_children_ref_mut(vec![blink]);
                    }
                }
                *current_cnt += truncate_len;
            }
        }
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

        let new_length = (anim_progress * self.content_len as f64 / duration)
            .round() as usize;

        let mut dest: Node<PMSG> = div(vec![], vec![]);

        Self::include_node(&mut dest, &self.children, new_length);
        self.animated_layer = Some(dest);

        let continue_animation = if is_in {
            new_length <= (self.content_len - 1)
        } else {
            new_length > 0
        };

        if continue_animation {
            vec![Msg::NextAnimation(is_in, start, duration)]
        } else {
            vec![Msg::StopAnimation]
        }
    }

    pub fn style(&self, theme: &crate::Theme) -> Vec<String> {
        vec![jss! {

            "hr": {
                color: theme.primary_color.clone(),
            },
            ".animate_list": {
                display: "inline-block",
                position: "relative",
            },

            ".animated_layer_wrapper": {
                position: "absolute",
                left: 0,
                right: 0,
                top: 0,
                overflow: "hidden",
                display: "inline-block",
                opacity: 0,
            },

            ".animate_list img": {
                width: percent(100),
                max_width: percent(100),
                height: "auto",
            },

            ".blink": {
                position: "relative",
                width: 0,
                height: 0,
                display: "inline-block",
                animation: format!("animate_list_blink-anim {}ms step-end infinite", 250),
            },

            ".animating .animate_list_children": {
                opacity: 0,
             },

            ".animating .animated_layer_wrapper": {
                opacity: 1,
            },

            "@keyframes animate_list_blink-anim": {
                "0%, 100%": {
                  color: "transparent",
                },

                "50%": {
                  color: "inherit",
                },
            },
        }]
    }
}
