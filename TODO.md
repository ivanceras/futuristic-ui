# TODO

- [X] Paragraph typing animation should support all other element nodes
    - algorithm:
        - for each node that contains text, increment the text length based on the number of characters in the text content
        - if the node does not contain text, increment by one.
            - The animation for element node is appending that node to the parent.
- [X] centralize the color in a root css
    - Added Theme module
- [X] add support for header bar
- [X] take inspiration from https://robertsspaceindustries.com/starmap/ for some other controls
    - button with slanted highlights to the sides and bottom
- [X] Convert the string styles into jss
- [X] Add more style to button such as showing hightlight bar in the bottom, top, left, or right
    - [X] Add a hover highlight at the bottom
- [X] Support for scoped style
    - This is done by prepending namespace to the selectors and class names used in the component
- [ ] Fix issued with Paragraph throwing a runtime error and panics.
- [X] Fix issue with webkit not displaying the animation list
    - webkit does not affect opacity:0 to span
- [ ] Make the futuristic button be in array with permutation of fui_button flag features
- [X] Make an image component by using the Frame which wraps the img element
- [ ] Add more futuristic widgets
    - [ ] Searchbox
    - [ ] Navigation links, anchors
        - [X] cut out/chipped button links
    - [ ] Table and animation
    - [ ] Figure image
- [ ] use css-colors crate to manipulate colors in theme
    - Issue, crate `css-colors` can't parse hex colors, will need to add crate `css-color` to do that
    - crate `color_processing` seems to have both parsing and process of colors https://crates.io/crates/color_processing
- [ ] Add multiple animation options for frame
    - Animation names, the corner is like a robotic hand.
    - [ ] grip/grab - frame__corner-gripping : the corner starts from outward then move inwards to grip the edges of the frame
    - [ ] release - frame__corner-releasing: corner-expands, the corner starts from the edges of the frame then move outwards
    - [ ] grow - frame__corner-grow: the corner starts from inside of the frame and grips-outwards
    - [ ] crush - frame__corner-crush: the corners starts from the edge of the frame and then move in-wards to the inside of the frame.
- [ ] Extract the name of the component using https://doc.rust-lang.org/std/any/fn.type_name.html
    and string manipulation to extract just the base struct name and into lower case, it will be the namespace to be used for css classnames
- [X] Remove the use of qoutes in style names
- [ ] Change the theme using the url
- [ ] Unify frame and button.
    - There should be container that has the 4 corner clip, the children components can then be put into it
    such as the buttons, chip buttons.
- [X] Add animation to image where the image is subdivided into multiple squares and displayed from top-left to right bottom
- [ ] Design a Component macro where the Component is declared in the view. The state of the component
    wil be stored in the `Program` hidden from the app. An Msg wrapper for the component is added automatically to the App's Msg.
    ```rust
        enum Msg{
            BtnClicked,
            Other(..)
        }
    ```
    ```rust
        node!{
            <div class="container">
                <FuiButton on_click=|_|Msg::BtnClicked>
            </div>
        }
    ```

