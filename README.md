# ⚙ FLTK-RS Examples 
NOT READY FOR USE YET
A small collection of modular UI features I'm creating while I'm developing MuTex, written with the [fltk-rs](https://github.com/MoAlyousef/fltk-rs) library.

Included Examples:
- Application Bundling
- Fuzzy Search Table
- Noise Generator (coming soon...)
- Image Zoom

## How to Use
- Install CMake, XCode/Windows Developer Tookit  
- cd to the example you would like 
- cargo run --releasae

## 🧺 Application Bundling
Build to a .app or .msi with [cargo-bundler](https://github.com/burtonageo/cargo-bundle)

---

## 🔍 Fuzzy Search Table
Fuzzy search a table of random strings, powered by [Sublime Fuzzy](https://crates.io/crates/sublime_fuzzy).
![Fuzzy Search](./images/fuzzy_search.png)

---
## ✂️ Image Clipping
Use the Image Crate to do crop and greyscale a selected portion of an image, then set it to a new frame. 
![Fuzzy Search](./images/image_select.png)