use crate::Theme;
use sauron::jss;
use sauron::prelude::*;
use sauron::units::px;

pub(crate) fn style(theme: &Theme) -> String {
    jss! (
        "blockquote": {
            border_left: format!("{} solid {}",px(6.0), &theme.controls.border_color),
            padding: format!("{} {} {} {}", px(10.0), px(20.0), px(10.0), px(26.0)),
            background_color: theme.controls.content_background_color.clone(),
        },

        "code": {
            border_top: format!("{} solid {}",px(1.0), &theme.controls.border_color),
            border_bottom: format!("{} solid {}",px(1.0), &theme.controls.border_color),
            background_color: theme.controls.content_background_color.clone(),
        },

        "table td, table th":  {
            border: format!("{} solid {}",px(1.0), &theme.controls.border_color),
            padding: "5px 10px",
            background_color: theme.controls.content_background_color.clone(),
        },

        "ul li": {
            list_style_image: format!("url('data:image/svg+xml;base64,{}')", base64::encode(bullet_icon(theme).render_to_string())),
        }
    )
}

fn bullet_icon(theme: &Theme) -> Node<()> {
    node! {
        <svg height="24" width="24" xmlns="http://www.w3.org/2000/svg">
            <style>
            {
                text(
                    jss!(
                        "line, path, circle, rect, polygon": {
                              stroke: theme.controls.border_color.clone(),
                              stroke_width: 2,
                              stroke_opacity: 1,
                              fill: theme.primary_color.clone(),
                              stroke_linecap: "round",
                              stroke_linejoin: "miter",
                        }
                    )
                )
            }
             </style>
            <path d="M4 8 l4 0 l3 4 l-3 4 l-4 0 z"/>
        </svg>
    }
}
