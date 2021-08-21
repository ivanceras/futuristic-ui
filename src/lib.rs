#![deny(warnings)]
#![recursion_limit = "256"]
use animate_list::AnimateList;
use frame::Frame;
use fui_button::{FuiButton, Options};
use image::Image;
use nav_header::NavHeader;
use paragraph::Paragraph;
use sauron::jss::jss;
use sauron::{
    html::{attributes::class, div, text},
    prelude::*,
    Application, Cmd, Node, Program,
};
use spinner::Spinner;
use theme::Theme;

mod animate_list;
mod frame;
mod fui_button;
mod image;
mod nav_header;
mod paragraph;
pub mod sounds;
mod spinner;
mod theme;

#[derive(Clone, Debug)]
pub enum Msg {
    ReAnimateFrame,
    ReAnimateHeader,
    ReAnimateParagraph,
    ReAnimateList,
    BtnMsg(usize, fui_button::Msg),
    FuiButtonMsg(fui_button::Msg),
    FrameMsg(frame::Msg),
    NavHeaderMsg(nav_header::Msg),
    ParagraphMsg(paragraph::Msg),
    AnimateListMsg(animate_list::Msg),
    ImageMsg(image::Msg),
    ReAnimateAll,
    NoOp,
}

pub struct App {
    nav_header: NavHeader,
    frame: Frame,
    paragraph: Paragraph<Msg>,
    button_array: Vec<FuiButton<Msg>>,
    fui_button: FuiButton<Msg>,
    spinner: Spinner<Msg>,
    animate_list: AnimateList<Msg>,
    image: Image,
}

impl Application<Msg> for App {
    fn init(&mut self, _: Program<Self, Msg>) -> Cmd<Self, Msg> {
        Self::reanimate_all()
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::ReAnimateHeader => {
                let follow_ups = self.nav_header.update(nav_header::Msg::AnimateIn);
                Cmd::batch_map(follow_ups, Msg::NavHeaderMsg)
            }
            Msg::NavHeaderMsg(header_msg) => {
                let follow_ups = self.nav_header.update(header_msg);
                Cmd::batch_map(follow_ups, Msg::NavHeaderMsg)
            }
            Msg::ReAnimateFrame => {
                let follow_ups = self.frame.update(frame::Msg::AnimateIn);
                Cmd::batch_map(follow_ups, Msg::FrameMsg)
            }
            Msg::FrameMsg(frame_msg) => {
                let follow_ups = self.frame.update(frame_msg);
                Cmd::batch_map(follow_ups, Msg::FrameMsg)
            }
            Msg::BtnMsg(index, btn_msg) => {
                let (follow_ups, effects) = self.button_array[index].update(btn_msg);
                Cmd::batch_msg(
                    effects
                        .into_iter()
                        .chain(
                            follow_ups
                                .into_iter()
                                .map(|follow_up| Msg::BtnMsg(index, follow_up)),
                        )
                        .collect(),
                )
            }
            Msg::FuiButtonMsg(fui_btn_msg) => {
                let (follow_ups, effects) = self.fui_button.update(fui_btn_msg);
                Cmd::batch_msg(
                    effects
                        .into_iter()
                        .chain(follow_ups.into_iter().map(Msg::FuiButtonMsg))
                        .collect(),
                )
            }
            Msg::AnimateListMsg(animate_list_msg) => {
                let (follow_ups, effects) = self.animate_list.update(animate_list_msg);
                Cmd::batch_msg(
                    effects
                        .into_iter()
                        .chain(follow_ups.into_iter().map(Msg::AnimateListMsg))
                        .collect(),
                )
            }
            Msg::ReAnimateList => {
                let (follow_ups, effects) = self.animate_list.update(animate_list::Msg::AnimateIn);
                Cmd::batch_msg(
                    effects
                        .into_iter()
                        .chain(follow_ups.into_iter().map(Msg::AnimateListMsg))
                        .collect(),
                )
            }
            Msg::ParagraphMsg(para_msg) => {
                let (follow_ups, effects) = self.paragraph.update(para_msg);
                Cmd::batch_msg(
                    effects
                        .into_iter()
                        .chain(follow_ups.into_iter().map(Msg::ParagraphMsg))
                        .collect(),
                )
            }
            Msg::ImageMsg(img_msg) => {
                let follow_ups = self.image.update(img_msg);
                Cmd::batch_map(follow_ups, Msg::ImageMsg)
            }
            Msg::ReAnimateParagraph => {
                let (follow_ups, effects) = self.paragraph.update(paragraph::Msg::AnimateIn);
                Cmd::batch_msg(
                    effects
                        .into_iter()
                        .chain(follow_ups.into_iter().map(Msg::ParagraphMsg))
                        .collect(),
                )
            }
            Msg::ReAnimateAll => Self::reanimate_all(),
            Msg::NoOp => Cmd::none(),
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![class("container")],
            vec![
                self.nav_header.view().map_msg(Msg::NavHeaderMsg),
                div(
                    vec![style! {"padding":px(20), "position": "relative", "left": percent(50)}],
                    vec![self.fui_button.view().map_msg(Msg::FuiButtonMsg)],
                ),
                self.frame.view().map_msg(Msg::FrameMsg),
                div(vec![class("futuristic-buttons-array")], {
                    self.button_array
                        .iter()
                        .enumerate()
                        .map(|(index, btn)| {
                            btn.view()
                                .map_msg(move |btn_msg| Msg::BtnMsg(index, btn_msg))
                        })
                        .collect::<Vec<_>>()
                }),
                self.paragraph.view(),
                p(vec![], vec![self.animate_list.view()]),
                self.spinner.view(),
                self.image.view().map_msg(Msg::ImageMsg),
                footer(
                    vec![],
                    vec![a(
                        vec![href("https://github.com/ivanceras/futuristic-ui/")],
                        vec![text("code")],
                    )],
                ),
            ],
        )
    }

    fn style(&self) -> Vec<String> {
        let base = crate::Theme::default();

        let controls_content_background_color = base.controls.content_background_color.to_owned();
        let controls_button_text_color = base.controls.button_text_color.to_owned();
        let secondary_color = base.secondary_color.to_owned();

        let accent_shadow = base.accent_shadow.to_owned();
        let accent_color = base.accent_color.to_owned();

        let primary_font = base.primary_font.to_owned();
        let secondary_font = base.secondary_font.to_owned();
        let controls_border_color = base.controls.border_color.to_owned();
        let background_color = base.background_color;

        let body_css = jss! {

            "button": {
                "color": controls_button_text_color.clone(),
                "border": format!("1px solid {}",controls_border_color),
                "z-index": 2,
                "display": "inline-block",
                "padding": "10px 20px",
                "outline": "none",
                "position": "relative",
                "font-size": "15.75px",
                "background-color": controls_content_background_color,
                "line-height": 1,
                "user-select": "none",
                "vertical-align": "middle",
            },

            "img": {
                "display": "inline-block",
            },

            "a": {
                "color": controls_button_text_color,
                "cursor": "pointer",
                "transition": "color 250ms ease-out",
                "text-shadow": format!("0 0 4px {}", accent_shadow),
                "text-decoration": "none",
            },

            "a ::selection": {
                "color": "#021114",
                "text-shadow": "none",
                "background-color": secondary_color.clone(),
            },

            "table": {
                "width": "100%",
                "border-collapse": "collapse",
                "color": secondary_color.clone(),
            },

            "thead": {
                "color": accent_color,
                "text-align": "left",
                "font-family": secondary_font,
                "font-weight": "bold",
                "white-space": "nowrap",
            },

            "tr": {
                "border-bottom": format!("1px solid {}", controls_border_color),
            },

             "td": {
                "padding": "5px",
                "vertical-align": "top",
            },
        };

        let container_css = jss! {
            ".container": {
                "color": secondary_color.clone(),
                "font-size": "21px",
                "line-height": "1.5",
                "font-family": primary_font,
                "margin": "auto",
                "background-color": background_color.clone(),
                "max-width": "50em",
                "padding": "10px",
            },

            ".container ::selection": {
                "color": background_color,
                "text-shadow": "none",
                "background-color": secondary_color,
            },

            ".futuristic-buttons-array": {
                "display": "flex",
                "flex-wrap": "wrap",
                "margin": "20px 10px",
            }
        };

        vec![
            body_css,
            container_css,
            self.nav_header.style().join("\n"),
            self.frame.style().join("\n"),
            self.fui_button.style().join("\n"),
            self.animate_list.style().join("\n"),
            self.spinner.style().join("\n"),
        ]
    }
}

impl Default for App {
    fn default() -> Self {
        let button_options = vec![
            ("ReAnimate All", Options::regular(), Msg::ReAnimateAll),
            (
                "Animate Paragraph",
                Options::regular(),
                Msg::ReAnimateParagraph,
            ),
            ("Animate List", Options::full(), Msg::ReAnimateList),
            (
                "Animate Frame",
                Options::simple().skewed(true),
                Msg::ReAnimateFrame,
            ),
            ("Spacer", Options::disabled().hidden(true), Msg::NoOp),
            ("Click", Options::regular(), Msg::NoOp),
            ("Disabled", Options::disabled(), Msg::NoOp),
            ("Muted", Options::muted(), Msg::NoOp),
        ];
        let button_array: Vec<FuiButton<Msg>> = button_options
            .into_iter()
            .map(|(label, options, msg)| {
                let mut btn = FuiButton::new_with_label(label);
                btn.set_options(options);
                btn.add_click_listener(move |_| msg.clone());
                btn
            })
            .collect();

        let paragraph_content = "This is an experimental demo showcasing usage of [Sauron](https://github.com/ivanceras/sauron). \
                    Application lifecycle to work alongside\
                    css transition, animation and timed DOM manipulation. This is also an exploration on how to add theming to the web framework.\
                    Sauron is a light-weight web framework designed to have you write least amount of code possible.";

        let frame_content = div(
            vec![styles([("padding", "20px 40px"), ("font-size", "32px")])],
            vec![text("Retro Futuristic UI in rust")],
        );

        let mut fui_button = FuiButton::<Msg>::new_with_label("Welcome");
        fui_button.add_click_listener(|_| Msg::ReAnimateAll);
        fui_button.set_options(Options::regular());

        App {
            frame: Frame::new_with_content(frame_content),
            nav_header: NavHeader::new_with_content("Navigation Header"),
            paragraph: Paragraph::new_with_markdown(paragraph_content),
            button_array,
            fui_button,
            spinner: Spinner::new(),
            animate_list: AnimateList::new_with_content(Self::animate_list_content()),
            image: Image::new("img/space.jpg", Some("Space as seen from space")),
        }
    }
}

impl App {
    fn animate_list_content() -> Node<Msg> {
        let long_txt = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam scelerisque purus faucibus urna venenatis, a elementum diam laoreet. Fusce eget enim justo. Pellentesque cursus metus elit, ut porttitor eros iaculis sit amet. Quisque varius felis id turpis iaculis, et viverra enim pulvinar. Curabitur vel lacus interdum, molestie purus ut, pretium nibh. Mauris commodo dolor magna, eget dignissim mauris semper vitae. Ut viverra nec ex quis semper. Sed sit amet tincidunt mauris. Mauris in imperdiet ipsum. Praesent pretium tortor ut felis posuere, sed lacinia nunc pretium. Morbi et felis nec neque accumsan tincidunt. In hac habitasse platea dictumst. Nulla sit amet elit sed purus posuere placerat ut quis metus. Etiam mattis interdum dui at ornare. Nunc sit amet venenatis lorem, sed eleifend mauris. Pellentesque eros sem, fermentum vel lacus at, congue rhoncus elit. ";
        div(
            vec![],
            vec![
                p(vec![], vec![
                    text("This is an experimental demo showcasing usage of sauron[0] Application lifecycle to work alongside
                    css transition, animation and timed DOM manipulation. This is also an exploration on how to add theming to the web framework.
                    Sauron is a light-weight web framework designed to have you write least amount of code possible."),
                    a(vec![href("https://github.com/ivanceras/sauron")], vec![text("Link here")]),
                    img(vec![styles([("width","600px"),("height", "auto"),("display","block")]),src("img/space.jpg")], vec![]),
                ]),
                li(vec![], vec![text(long_txt)]),
                li(vec![], vec![text("List 2")]),
                ul(
                    vec![],
                    vec![
                        li(vec![], vec![text("SubList 3")]),
                        li(vec![], vec![text("Not too long txt here... trying to see if it is correctly animated")]),
                    ],
                ),
                div(vec![],vec![
                    table(vec![],vec![
                        thead(vec![],vec![
                            tr(vec![],vec![
                                th(vec![],vec![text("Prop name")]),
                                th(vec![],vec![text("Type")]),
                                th(vec![],vec![text("Default")]),
                                th(vec![],vec![text("Description")]),
                            ]),
                        ]),
                        tbody(vec![],vec![
                            tr(vec![],vec![
                                td(vec![],vec![text("name")]),
                                td(vec![],vec![text("string")]),
                                td(vec![],vec![text("''")]),
                                td(vec![],vec![text("The base name of the component")]),
                            ]),
                            tr(vec![],vec![
                                td(vec![],vec![text("age")]),
                                td(vec![],vec![text("number")]),
                                td(vec![],vec![text("0")]),
                                td(vec![],vec![text("The age of the component")]),
                            ]),
                            tr(vec![],vec![
                                td(vec![],vec![text("married")]),
                                td(vec![],vec![text("bool")]),
                                td(vec![],vec![text("false")]),
                                td(vec![],vec![text("If the component is married")]),
                            ]),
                        ]),
                    ]),
                ])
            ],
        )
    }

    fn reanimate_all() -> Cmd<Self, Msg> {
        Cmd::new(|program| {
            program.dispatch(Msg::ReAnimateFrame);
            program.dispatch(Msg::ReAnimateHeader);
            program.dispatch(Msg::ReAnimateParagraph);
            program.dispatch(Msg::ReAnimateList);
        })
    }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_log")]
    console_log::init_with_level(log::Level::Trace).unwrap();
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    let app_container = sauron::document()
        .get_element_by_id("app_container")
        .expect("must have the app_container in index.html");
    Program::replace_mount(App::default(), &app_container);
}
