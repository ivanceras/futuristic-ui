use crate::{animate_list, AnimateList};
use sauron::{prelude::*, Node};

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    AnimateListMsg(animate_list::Msg),
}

/// accepts a markdown and animate the content
pub struct Paragraph<XMSG> {
    animated_list: AnimateList<XMSG>,
}

impl<XMSG> Paragraph<XMSG>
where
    XMSG: Clone,
{
    pub fn new_with_markdown(md: &str) -> Self {
        Paragraph {
            animated_list: AnimateList::with_content(
                sauron_markdown::markdown(md),
            ),
        }
    }
}

impl<XMSG> Container<Msg, XMSG> for Paragraph<XMSG>
where
    XMSG: Clone,
{
    fn update(&mut self, msg: Msg) -> Effects<Msg, XMSG> {
        match msg {
            Msg::AnimateIn => {
                let effects =
                    self.animated_list.update(animate_list::Msg::AnimateIn);
                effects.map_msg(Msg::AnimateListMsg)
            }
            Msg::AnimateListMsg(amsg) => {
                let effects = self.animated_list.update(amsg);
                effects.map_msg(Msg::AnimateListMsg)
            }
        }
    }

    fn view(&self) -> Node<XMSG> {
        p(vec![], vec![self.animated_list.view()])
    }
}
