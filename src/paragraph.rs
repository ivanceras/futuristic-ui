use crate::{animate_list, AnimateList};
use sauron::{prelude::*, Node};

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    AnimateListMsg(animate_list::Msg),
}

/// accepts a markdown and animate the content
pub struct Paragraph<PMSG> {
    animated_list: AnimateList<PMSG>,
}

impl<PMSG> Paragraph<PMSG>
where
    PMSG: Clone,
{
    pub fn new_with_markdown(md: &str) -> Self {
        Paragraph {
            animated_list: AnimateList::new_with_content(sauron_markdown::markdown(md)),
        }
    }
}

impl<PMSG> Container<Msg, PMSG> for Paragraph<PMSG>
where
    PMSG: Clone,
{
    fn update(&mut self, msg: Msg) -> (Vec<Msg>, Vec<PMSG>) {
        match msg {
            Msg::AnimateIn => {
                let (follow_ups, effects) = self.animated_list.update(animate_list::Msg::AnimateIn);
                (
                    follow_ups.into_iter().map(Msg::AnimateListMsg).collect(),
                    effects,
                )
            }
            Msg::AnimateListMsg(amsg) => {
                let (follow_ups, effects) = self.animated_list.update(amsg);
                (
                    follow_ups.into_iter().map(Msg::AnimateListMsg).collect(),
                    effects,
                )
            }
        }
    }

    fn view(&self) -> Node<PMSG> {
        p(vec![], vec![self.animated_list.view()])
    }
}
