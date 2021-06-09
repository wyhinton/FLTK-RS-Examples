# FLTK-RS Examples

A small collection of modular UI features made with the [fltk-rs](https://github.com/MoAlyousef/fltk-rs) library.

# Table of Contents

- [FLTK-RS Examples](#fltk-rs-examples)
- [Table of Contents](#table-of-contents)
  - [Custom Widget Examples](#custom-widget-examples)
    - [Fuzzy Search Table](#fuzzy-search-table)
    - [Image Selection](#image-selection)
    - [Speedy2d Pan/Zoom Canvas](#speedy2d-panzoom-canvas)
  - [Style Demos](#style-demos)
    - [Background Fill for Group](#background-fill-for-group)
  - [Event Handling](#event-handling)
    - [Crossbeam Channels](#crossbeam-channels)
    - [Ordinal Custom Events](#ordinal-custom-events)

---

## Custom Widget Examples

### Fuzzy Search Table

Fuzzy search a table of random strings, powered by [Sublime Fuzzy](https://crates.io/crates/sublime_fuzzy).
![Fuzzy Search](./.github/images/fuzzy_search.png)

### Image Selection

Select a portion of an image from one frame and draw it in another frame.
![Fuzzy Search](./.github/images/image_select.png)

### Speedy2d Pan/Zoom Canvas

A hardware accelerated pan/zoom canvas using [Speedy2d](https://github.com/QuantumBadger/Speedy2D) as the render engine.
![Speedy2d Canvas](./.github/images/speedy2d_canvas.gif)

<!-- ![Speedy2d Canvas](./images/Speedy_2d_Pan_Zoom_Canvas.png) -->

## Style Demos

### Background Fill for Group

Demonstrates creating a group that has a solid background color over which child widgets are drawn.

![Background Fill Group](./.github/images/background_fill_group.PNG)

## Event Handling

### Crossbeam Channels

Simple app demonstrating demonstrating handling channeled events (which have the advantage of carrying data) as well as custom events, all the while keeping fn main() clean.

![crossbeam channels](./.github/images/crossbeam_channels.png)

### Ordinal Custom Events

using `app::handle_main` can result in more ergonomic code, allowing any widget to handle the event, keeping `main` more organized. However, custom event values are hard coded. [enum_ordanlize](https://docs.rs/enum-ordinalize/3.1.10/enum_ordinalize/) lets us define our custom event values relative to the number of total enum values. Thus we can insert or remove custom events without worrying about disjoint values.

```rust
#[derive(Debug, PartialEq, Eq, Ordinalize)]
#[repr(i32)]
enum CustomEvents{
    AddOne = 41,
    AddTwo,
    AddThree,
}
//..
win.handle(move |_, ev|
    if ev.bits() == CustomEvents::AddOne.ordinal(){
    *counter.borrow_mut() += 1;
    true
} else if ev.bits() == CustomEvents::AddTwo.ordinal(){
    *counter.borrow_mut() += 2;
    true
} else if ev.bits() == CustomEvents::AddThree.ordinal(){
    *counter.borrow_mut() += 3;
    true
} else {
    false
});

```
