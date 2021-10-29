//#![deny(warnings)]
#![recursion_limit = "256"]
use animate_list::AnimateList;
use button::{Button, Options};
use frame::Frame;
use image::Image;
use nav_header::NavHeader;
use paragraph::Paragraph;
use sauron::jss::jss;
use sauron::{
    html::units::em,
    html::{attributes::class, div, text},
    prelude::*,
    Application, Cmd, Node, Program,
};
use spinner::Spinner;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::rc::Rc;
use theme::Theme;

mod animate_list;
mod button;
mod frame;
mod image;
mod nav_header;
mod paragraph;
pub mod sounds;
mod spinner;
mod theme;

#[derive(Clone, Debug)]
pub enum Msg {
    HashChanged(String),
    ReAnimateFrame,
    ReAnimateHeader,
    ReAnimateParagraph,
    ReAnimateList,
    ButtonMsg(Rc<RefCell<button::Button<Msg>>>, button::Msg),
    FrameMsg(Box<frame::Msg<Msg>>),
    NavHeaderMsg(nav_header::Msg),
    ParagraphMsg(paragraph::Msg),
    AnimateListMsg(animate_list::Msg),
    ImageEffectsMsg(image::Msg),
    SetMeasurements(Measurements),
    StartAnimateImageEffects,
    ReAnimateAll,
    NoOp,
}

pub struct App {
    nav_header: NavHeader,
    frame: Frame<Msg>,
    paragraph: Paragraph<Msg>,
    spinner: Spinner<Msg>,
    animate_list: AnimateList<Msg>,
    image: Image,
    theme: Theme,
    btn_context: RefCell<Context<Button<Msg>, Msg, button::Msg>>,
    measurements: Option<Measurements>,
}

pub struct Context<COMP, MSG, CMSG> {
    components: BTreeMap<String, Rc<RefCell<COMP>>>,
    _phantom_msg: PhantomData<MSG>,
    _phantom_cmsg: PhantomData<CMSG>,
}

impl Default for App {
    fn default() -> Self {
        let frame_content = div(
            vec![styles([("padding", "20px 40px"), ("font-size", "32px")])],
            vec![text("Retro Futuristic UI in rust")],
        )
        .add_children(vec![Self::show_color_selection()]);

        App {
            frame: Frame::with_content(frame_content),
            nav_header: NavHeader::with_content("Navigation Header"),
            paragraph: Paragraph::new_with_markdown(MARKDOWN_EXAMPLE),
            spinner: Spinner::new(),
            animate_list: AnimateList::with_content(
                Self::animate_list_content(),
            ),
            image: Image::new("img/space.jpg"),
            theme: Theme::default(),
            btn_context: RefCell::new(Context::new()),
            measurements: None,
        }
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        let hash = sauron::window().location().hash().expect("must get hash");
        self.restyle(&hash);
        let cmd_hash_changed = Window::on_hashchange(Msg::HashChanged);
        Self::reanimate_all().append(vec![cmd_hash_changed])
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::HashChanged(hash) => {
                self.restyle(&hash);
                Self::reanimate_all().measure()
            }
            Msg::ReAnimateHeader => {
                let effects =
                    self.nav_header.update(nav_header::Msg::AnimateIn);
                Cmd::from(effects.map_msg(Msg::NavHeaderMsg)).measure()
            }
            Msg::NavHeaderMsg(header_msg) => {
                let effects = self.nav_header.update(header_msg);
                Cmd::from(effects.map_msg(Msg::NavHeaderMsg)).measure()
            }
            Msg::ReAnimateFrame => {
                let effects = self.frame.update(frame::Msg::AnimateIn);
                Cmd::from(
                    effects.localize(|fmsg| Msg::FrameMsg(Box::new(fmsg))),
                )
                .measure()
            }
            Msg::FrameMsg(frame_msg) => {
                let effects = self.frame.update(*frame_msg);
                Cmd::from(
                    effects.localize(|fmsg| Msg::FrameMsg(Box::new(fmsg))),
                )
                .measure()
            }
            Msg::ButtonMsg(btn, btn_msg) => {
                let effects = self.btn_context.borrow_mut().update_component(
                    btn,
                    btn_msg,
                    Msg::ButtonMsg,
                );
                Cmd::from(effects)
            }
            Msg::AnimateListMsg(animate_list_msg) => {
                let effects = self.animate_list.update(animate_list_msg);
                Cmd::from(effects.localize(Msg::AnimateListMsg)).measure()
            }
            Msg::ReAnimateList => {
                let effects =
                    self.animate_list.update(animate_list::Msg::AnimateIn);
                Cmd::from(effects.localize(Msg::AnimateListMsg)).measure()
            }
            Msg::ParagraphMsg(para_msg) => {
                let effects = self.paragraph.update(para_msg);
                Cmd::from(effects.localize(Msg::ParagraphMsg)).measure()
            }
            Msg::ImageEffectsMsg(effects_msg) => {
                let effects = self.image.update(effects_msg);
                Cmd::from(effects.map_msg(Msg::ImageEffectsMsg)).measure()
            }
            Msg::StartAnimateImageEffects => {
                let effects = self.image.update(image::Msg::AnimateIn);
                Cmd::from(effects.map_msg(Msg::ImageEffectsMsg)).measure()
            }
            Msg::ReAnimateParagraph => {
                let effects = self.paragraph.update(paragraph::Msg::AnimateIn);
                Cmd::from(effects.localize(Msg::ParagraphMsg)).measure()
            }
            Msg::SetMeasurements(measurements) => {
                self.measurements = Some(measurements);
                Cmd::none()
            }
            Msg::ReAnimateAll => Self::reanimate_all().measure(),
            Msg::NoOp => Cmd::none(),
        }
    }

    fn view(&self) -> Node<Msg> {
        let mut btn_context = self.btn_context.borrow_mut();
        div(
            vec![class("container")],
            vec![
                self.nav_header.view().map_msg(Msg::NavHeaderMsg),
                div(
                    vec![class("measurements")],
                    vec![if let Some(measurements) = &self.measurements {
                        text!("total patches: {}, node_count: {}, update took: {}ms", measurements.total_patches, measurements.view_node_count, measurements.total_time)
                    } else {
                        text("")
                    }],
                ),
                div(
                    vec![
                        style! {"padding":px(20), "position": "relative", "left": format!("calc({} - {})", percent(50), px(400 / 2))},
                    ],
                    vec![btn_context.map_view(
                        "button",
                        {
                            Button::<Msg>::with_label("Welcome")
                                .width(400)
                                .height(100)
                                .add_click_listener(|_| Msg::ReAnimateAll)
                                .with_options(Options::full())
                        },
                        Msg::ButtonMsg,
                    )],
                ),
                self.frame
                    .view()
                    .map_msg(|fmsg| Msg::FrameMsg(Box::new(fmsg))),
                div(
                    vec![class("futuristic-buttons")],
                    vec![
                        btn_context.map_view(
                            "reanimate",
                            {
                                Button::with_label("Re-Animate All")
                                    .add_click_listener(|_| Msg::ReAnimateAll)
                            },
                            Msg::ButtonMsg,
                        ),
                        btn_context.map_view(
                            "animate_frame",
                            {
                                Button::with_label("Animate Frame")
                                    .skewed()
                                    .add_click_listener(|_| Msg::ReAnimateFrame)
                            },
                            Msg::ButtonMsg,
                        ),
                        btn_context.map_view(
                            "error",
                            Button::with_label("Error").error(),
                            Msg::ButtonMsg,
                        ),
                        btn_context.map_view(
                            "success",
                            Button::with_label("Success").success(),
                            Msg::ButtonMsg,
                        ),
                        btn_context.map_view(
                            "info",
                            Button::with_label("Info").info(),
                            Msg::ButtonMsg,
                        ),
                        btn_context.map_view(
                            "warning",
                            Button::with_label("Warning").warning(),
                            Msg::ButtonMsg,
                        ),
                    ],
                ),
                btn_context.map_view(
                    "animate_image",
                    {
                        Button::<Msg>::with_label("Animate Image")
                            .chipped()
                            .add_click_listener(|_| {
                                Msg::StartAnimateImageEffects
                            })
                    },
                    Msg::ButtonMsg,
                ),
                btn_context.map_view(
                    "chip_error",
                    Button::with_label("Error").chipped().error(),
                    Msg::ButtonMsg,
                ),
                btn_context.map_view(
                    "chip_success",
                    Button::with_label("Success").chipped().success(),
                    Msg::ButtonMsg,
                ),
                btn_context.map_view(
                    "chip_info",
                    Button::with_label("Info").chipped().info(),
                    Msg::ButtonMsg,
                ),
                btn_context.map_view(
                    "chip_warning",
                    Button::with_label("Warning").chipped().warning(),
                    Msg::ButtonMsg,
                ),
                self.image.view().map_msg(Msg::ImageEffectsMsg),
                btn_context.map_view(
                    "animate_list",
                    Button::<Msg>::with_label("Animate List")
                        .add_click_listener(|_| Msg::ReAnimateList),
                    Msg::ButtonMsg,
                ),
                p(vec![], vec![self.animate_list.view()]),
                self.spinner.view(),
                btn_context.map_view(
                    "animate_paragraph",
                    {
                        Button::<Msg>::with_label("Animate Paragraph")
                            .add_click_listener(|_| Msg::ReAnimateParagraph)
                    },
                    Msg::ButtonMsg,
                ),
                self.paragraph.view(),
                footer(
                    vec![],
                    vec![a(
                        vec![href(
                            "https://github.com/ivanceras/futuristic-ui/",
                        )],
                        vec![text("code")],
                    )],
                ),
            ],
        )
    }

    fn measurements(&self, measurements: Measurements) -> Cmd<Self, Msg> {
        log::info!("measurements: {:#?}", measurements);
        Cmd::batch_msg(vec![Msg::SetMeasurements(measurements)])
    }
}

impl App {
    fn calculate_theme_from_url_hash(hash: &str) -> Theme {
        let hash = hash.trim_start_matches("#/");
        let splinters: Vec<&str> = hash.split('/').collect();
        if splinters.len() >= 2 {
            let primary = splinters[0];
            let background = splinters[1];
            if let Ok(theme) = Theme::from_str(primary, background) {
                theme
            } else {
                Theme::default()
            }
        } else {
            Theme::default()
        }
    }

    fn restyle(&mut self, hash: &str) {
        self.theme = Self::calculate_theme_from_url_hash(hash);
        let styles = self.style();
        Self::remove_style();
        Self::inject_style(&styles.join("\n"));
    }

    fn remove_style() {
        use sauron::wasm_bindgen::JsCast;
        let document = crate::document();
        if let Some(html_style) = document
            .query_selector(".futuristic-ui")
            .expect("must query")
        {
            let html_style: web_sys::Element = html_style.unchecked_into();
            html_style.remove();
        }
    }

    /// We are using a custom way to put style
    fn style(&self) -> Vec<String> {
        let base = &self.theme;
        let controls_content_background_color =
            base.controls.content_background_color.to_owned();
        let controls_button_text_color =
            base.controls.button_text_color.to_owned();
        let secondary_color = base.secondary_color.to_owned();

        let accent_shadow = base.accent_shadow.to_owned();
        let accent_color = base.accent_color.to_owned();

        let primary_font = base.primary_font.to_owned();
        let secondary_font = base.secondary_font.to_owned();
        let controls_border_color = base.controls.border_color.to_owned();
        let background_color = base.background_color.to_owned();

        let body_css = jss! {

            button: {
                color: controls_button_text_color.clone(),
                border: format!("{} solid {}",px(1), controls_border_color),
                z_index: 2,
                display: "inline-block",
                padding: format!("{} {}",px(10), px(20)),
                outline: "none",
                position: "relative",
                font_size: px(15.75),
                background_color: controls_content_background_color,
                line_height: 1,
                user_select: "none",
                vertical_align: "middle",
            },

            img: {
                display: "inline-block",
            },

            a: {
                color: controls_button_text_color,
                cursor: "pointer",
                transition: "color 250ms ease-out",
                text_shadow: format!("{} {} {} {}", 0, 0, px(4), accent_shadow),
                text_decoration: "none",
            },

            "a ::selection": {
                color: "#021114",
                text_shadow: "none",
                background_color: secondary_color.clone(),
            },

            table: {
                width: percent(100),
                border_collapse: "collapse",
                color: secondary_color.clone(),
            },

            thead: {
                color: accent_color,
                text_align: "left",
                font_family: secondary_font,
                font_weight: "bold",
                white_space: "nowrap",
            },

            tr: {
                border_bottom: format!("{} solid {}", px(1), controls_border_color),
            },

            td: {
                padding: px(5),
                vertical_align: "top",
            },
        };

        let container_css = jss! {
            ".container": {
                color: secondary_color.clone(),
                font_size: px(21),
                line_height: 1.5,
                font_family: primary_font,
                margin: "auto",
                background_color: background_color.clone(),
                max_width: em(50),
                padding: px(10),
            },

            ".container ::selection": {
                color: background_color,
                text_shadow: "none",
                background_color: secondary_color,
            },

            ".futuristic-buttons-array": {
                display: "flex",
                flex_wrap: "wrap",
                margin: format!("{} {}", px(20), px(10)),
            },

            ".more_colors": {
                display: "flex",
                flex_direction: "row",
            },

            ".more_colors .pick": {
                width: px(10),
                height: px(10),
                border_width: px(4),
                border_style: "solid",
            }
        };

        vec![
            body_css,
            container_css,
            NavHeader::style(&self.theme),
            Frame::<Msg>::style(&self.theme),
            Button::<Msg>::style(&self.theme),
            AnimateList::<Msg>::style(&self.theme),
            Spinner::<Msg>::style(&self.theme),
            self.image.style(&self.theme),
        ]
    }

    fn inject_style(style: &str) {
        use sauron::wasm_bindgen::JsCast;
        let document = crate::document();
        let html_style = document
            .create_element("style")
            .expect("must be able to create style element");
        html_style
            .set_attribute("class", "futuristic-ui")
            .expect("must set attribute");
        let html_style: web_sys::Node = html_style.unchecked_into();
        html_style.set_text_content(Some(style));
        let head = document.head().expect("must have a head");
        head.append_child(&html_style).expect("must append style");
    }

    fn show_color_selection<MSG>() -> Node<MSG> {
        let colors = vec![
            "#029dbb", "black", "green", "red", "white", "yellow", "purple",
        ];
        let backgrounds = vec!["white", "black"];
        let mut pairs: Vec<(&str, &str)> = vec![];
        for primary in colors.iter() {
            for background in backgrounds.iter() {
                if primary != background {
                    pairs.push((primary, background));
                }
            }
        }
        div(
            vec![class("more_colors")],
            pairs.into_iter().map(|(primary,background)|{
                a(vec![class("colors"), href(format!("#/{}/{}",primary,background))], vec![
                    div(vec![class("pick"), style!{background_color: primary, border_color: background}], vec![]),
                ])
            })
        )
    }

    fn animate_list_content() -> Node<Msg> {
        let long_txt = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam scelerisque purus faucibus urna venenatis, a elementum diam laoreet. Fusce eget enim justo. Pellentesque cursus metus elit, ut porttitor eros iaculis sit amet. Quisque varius felis id turpis iaculis, et viverra enim pulvinar. Curabitur vel lacus interdum, molestie purus ut, pretium nibh. Mauris commodo dolor magna, eget dignissim mauris semper vitae. Ut viverra nec ex quis semper. Sed sit amet tincidunt mauris. Mauris in imperdiet ipsum. Praesent pretium tortor ut felis posuere, sed lacinia nunc pretium. Morbi et felis nec neque accumsan tincidunt. In hac habitasse platea dictumst. Nulla sit amet elit sed purus posuere placerat ut quis metus. Etiam mattis interdum dui at ornare. Nunc sit amet venenatis lorem, sed eleifend mauris. Pellentesque eros sem, fermentum vel lacus at, congue rhoncus elit. ";
        div(
            vec![],
            vec![
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
                ]),
                p(vec![], vec![
                    text("This is an experimental demo showcasing usage of sauron[0] Application lifecycle to work alongside
                    css transition, animation and timed DOM manipulation. This is also an exploration on how to add theming to the web framework.
                    Sauron is a light-weight web framework designed to have you write least amount of code possible."),
                    a(vec![href("https://github.com/ivanceras/sauron")], vec![text("Link here")]),
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
            ],
        )
    }

    fn reanimate_all() -> Cmd<Self, Msg> {
        Cmd::from(Effects::with_local(vec![
            Msg::ReAnimateFrame,
            Msg::ReAnimateHeader,
            Msg::ReAnimateParagraph,
            Msg::ReAnimateList,
            Msg::StartAnimateImageEffects,
        ]))
        .measure()
    }
}

impl<COMP, MSG, CMSG> Context<COMP, MSG, CMSG>
where
    COMP: Component<CMSG, MSG> + std::fmt::Debug + 'static,
    MSG: 'static,
    CMSG: 'static,
{
    fn new() -> Self {
        Self {
            components: BTreeMap::new(),
            _phantom_msg: PhantomData,
            _phantom_cmsg: PhantomData,
        }
    }

    /// simultaneously save the component into context for the duration until the next update loop
    /// The comp_id is important such that the component is not re-created
    /// at every view call. This should unique such that it can re-use the existing
    /// component from previous view call. Don't use random unique, otherwise will be
    /// re-crated at every view call.
    fn map_view<F>(
        &mut self,
        comp_id: impl ToString,
        component: COMP,
        mapper: F,
    ) -> Node<MSG>
    where
        F: Fn(Rc<RefCell<COMP>>, CMSG) -> MSG + 'static,
    {
        log::trace!(
            "{} component_id: {:?}",
            comp_id.to_string(),
            component.get_component_id(),
        );
        if let Some(component) = self.components.get(&comp_id.to_string()) {
            let component_clone = component.clone();
            component
                .borrow()
                .view()
                .map_msg(move |cmsg| mapper(component_clone.clone(), cmsg))
        } else {
            let component = Rc::new(RefCell::new(component));
            let component_clone = component.clone();
            let view = component
                .borrow()
                .view()
                .map_msg(move |cmsg| mapper(component_clone.clone(), cmsg));
            self.components.insert(comp_id.to_string(), component);
            view
        }
    }

    fn update_component<F>(
        &mut self,
        component: Rc<RefCell<COMP>>,
        dmsg: CMSG,
        mapper: F,
    ) -> Effects<MSG, ()>
    where
        F: Fn(Rc<RefCell<COMP>>, CMSG) -> MSG + 'static,
    {
        let component_clone = component.clone();
        log::trace!(
            "component_id: {:?}",
            component.borrow().get_component_id()
        );
        component
            .borrow_mut()
            .update(dmsg)
            .localize(move |dmsg| mapper(component_clone.clone(), dmsg))
    }
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    let app_container = sauron::document()
        .get_element_by_id("app_container")
        .expect("must have the app_container in index.html");
    Program::replace_mount(App::default(), &app_container);
}

const MARKDOWN_EXAMPLE: &str = r#"
An h1 header
============

Paragraphs are separated by a blank line.

2nd paragraph. *Italic*, **bold**, and `monospace`. Itemized lists
look like:

  * this one
  * that one
  * the other one

Note that --- not considering the asterisk --- the actual text
content starts at 4-columns in.

> Block quotes are
> written like so.
>
> They can span multiple paragraphs,
> if you like.

Use 3 dashes for an em-dash. Use 2 dashes for ranges (ex., "it's all
in chapters 12--14"). Three dots ... will be converted to an ellipsis.
Unicode is supported. ☺



An h2 header
------------

Here's a numbered list:

 1. first item
 2. second item
 3. third item

Note again how the actual text starts at 4 columns in (4 characters
from the left side). Here's a code sample:

    # Let me re-iterate ...
    for i in 1 .. 10 { do-something(i) }

As you probably guessed, indented 4 spaces. By the way, instead of
indenting the block, you can use delimited blocks, if you like:

~~~
define foobar() {
    print "Welcome to flavor country!";
}
~~~

(which makes copying & pasting easier). You can optionally mark the
delimited block for Pandoc to syntax highlight it:

~~~python
import time
# Quick, count to ten!
for i in range(10):
    # (but not *too* quick)
    time.sleep(0.5)
    print(i)
~~~



### An h3 header ###

Now a nested list:

 1. First, get these ingredients:

      * carrots
      * celery
      * lentils

 2. Boil some water.

 3. Dump everything in the pot and follow
    this algorithm:

        find wooden spoon
        uncover pot
        stir
        cover pot
        balance wooden spoon precariously on pot handle
        wait 10 minutes
        goto first step (or shut off burner when done)

    Do not bump wooden spoon or it will fall.

Notice again how text always lines up on 4-space indents (including
that last line which continues item 3 above).

Here's a link to [a website](http://foo.bar), to a [local
doc](local-doc.html), and to a [section heading in the current
doc](#an-h2-header). Here's a footnote [^1].

[^1]: Some footnote text.

Tables can look like this:

Name           Size  Material      Color
------------- -----  ------------  ------------
All Business      9  leather       brown
Roundabout       10  hemp canvas   natural
Cinderella       11  glass         transparent

Table: Shoes sizes, materials, and colors.

(The above is the caption for the table.) Pandoc also supports
multi-line tables:

--------  -----------------------
Keyword   Text
--------  -----------------------
red       Sunsets, apples, and
          other red or reddish
          things.

green     Leaves, grass, frogs
          and other things it's
          not easy being.
--------  -----------------------

A horizontal rule follows.

***

Here's a definition list:

apples
  : Good for making applesauce.

oranges
  : Citrus!

tomatoes
  : There's no "e" in tomatoe.

Again, text is indented 4 spaces. (Put a blank line between each
term and  its definition to spread things out more.)

Here's a "line block" (note how whitespace is honored):

| Line one
|   Line too
| Line tree

and images can be specified like so:

![example image](img/space.jpg "An exemplary image")

Inline math equation: $\omega = d\phi / dt$. Display
math should get its own line like so:

$$I = \int \rho R^{2} dV$$

And note that you can backslash-escape any punctuation characters
which you wish to be displayed literally, ex.: \`foo\`, \*bar\*, etc.
"#;
