I'm using cargo-bundle to create a .app bundle for my rust application built with fltk-rs. 
There are assets in appliction like images. While I'm developing, accesing these applications is no problem:

```lang-rust
//main.rs
let mut my_img = SharedImage::load("imgs/smiley.png").unwrap();
```

```
.
├── Cargo.lock
├── Cargo.toml
├── app_icon_design.psd
├── imgs
│   └── smiley.PNG <--get this iamge
├── src
│   ├── app_icon.png
│   ├── app_icon@2x.png
│   ├── icon32x32.png
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    ├── debug
    └── release
```

I then bundle to a .app with ```cargo build --release``` which gives me the following directory structure inside of my .app: 

```
.
├── Info.plist
├── MacOS
│   └── build_to_app_bundle<--my executable
└── Resources
    └── imgs
        └── smiley.PNG
```

Now my application needs to get the image file from the ```/Resources folder```:
```lang-rust
println!("my resources folder is: {:?}", std::env::current_exe().unwrap().parent().unwrap().join(Path::new("Resources")));
let my_resources_path = std::env::current_exe().unwrap().parent().unwrap().join(Path::new("Resources"));

```

How to make it so that every time I want to load an image from a path I do not need to explicitly reference the ```/Resources``` value?

I'm using cargo-bundle to create a .app bundle for my rust application built with fltk-rs. 
There are assets in appliction like images. While I'm developing, accesing these applications is no problem:

```lang-rust
//main.rs
let mut my_img = SharedImage::load("imgs/smiley.png").unwrap();
```

```
.
├── Cargo.lock
├── Cargo.toml
├── app_icon_design.psd
├── imgs
│   └── smiley.PNG <--get this iamge
├── src
│   ├── app_icon.png
│   ├── app_icon@2x.png
│   ├── icon32x32.png
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    ├── debug
    └── release
```

I then bundle to a .app with ```cargo build --release``` which gives me the following directory structure inside of my .app: 

```
.
├── Info.plist
├── MacOS
│   └── build_to_app_bundle<--my executable
└── Resources
    └── imgs
        └── smiley.PNG
```

Now my application needs to get the image file from the ```/Resources folder```:
```lang-rust
println!("my resources folder is: {:?}", std::env::current_exe().unwrap().parent().unwrap().join(Path::new("Resources")));
let my_resources_path = std::env::current_exe().unwrap().parent().unwrap().join(Path::new("Resources"));

```

How to make it so that every time I want to load an image from a path I do not need to explicitly reference the ```/Resources``` value?

