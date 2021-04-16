For the purposes of drawing images into a GUI, I need to determine if an image is rgb or rgba. I've asked a similar question in regards to getting the bit depth of an ```image::DynamicImage```,
but was wondering if the image crate has any other means of detecting the presence of an alpha channel an image.

```
image = "0.23.14"
```
```lang-rust
fn main(){
    let loaded_img: DynamicImage = ImageReader::open(path).unwrap().decode().unwrap();  
    dbg!(is_rgba(loaded_img));
}

fn is_rgba(img: DynamicImage) -> bool{
    match img {
        DynamicImage::ImageLumaA8(_) => true,
        DynamicImage::ImageRgba8(_) => true,
        DynamicImage::ImageBgra8(_) => true,
        DynamicImage::ImageLumaA16(_) => true,
        DynamicImage::ImageRgba16(_) => true,
        _=>false
    }
}
```


Hello,
I was wondering if you had anythought how one might impliment as masking 


```
trait ShopItemExt{
    fn price()->f32;
}

struct Chips{
    price: 
}

struct Chips{
    
}

impl ShopeItemExt for Chips {
    price()->f32{
        self.price
    }
}
impl ShopeItemExt for Soda {
    price()->f32{
        self.price
    }
}
struct SnackShop{
    snacks: Vec<ShopItem>
}



```
What is the state of anti-aliasing in rust? I'm currently developing a photo editing app in Rust, but I'm running into a big wall when it comes
to implimenting any kind of performant AA for an OpenGL context (FXAA, MLAA, MSAA). Searching Crates.io for anything related to anti-aliasing returns
some results for 3D Engines, Game Engines/Frameworks, and some specilized path rendering libraries, but seeing as I've already dedicated my project to
a certain graphics and UI library,  (Speedy2D and FLTK-RS, respectivley) and my application is a 2D, not 3D, the first two options are out of the question. 
While libraries like ochre could be useful, their scope and api is fairly limited.

What I think I want is a process similar to this:

Graphics Renderer -> FrameBuffer -> AA Alorythm -> Final Output in Window 

I want the AA to be applied as a kind of post processing effect. Is this a reasonable approach?

Here's a minimal example of targeting targetin an OpenGL Window:

```lang-rust
  
use fltk::{
    app,
    enums::Event,
    prelude::{WidgetBase, WidgetExt, GroupExt, WindowExt},
    window::{GlutWindow, Window},
    utils
};
use speedy2d::GLRenderer;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageDataType, ImageSmoothingMode};

fn main() {
    let app = app::App::default();
    let mut main_win = Window::default().with_size(800, 600);
    let mut win = GlutWindow::default().with_size(300, 300).center_of(&main_win);
    win.end();
    main_win.end();
    main_win.show();
    win.make_current();

    win.handle(|ev| match ev {
        Event::Push => {
            println!("Pushed");
            true
        },
        _ => false,
    });

    gl::load_with(|s| win.get_proc_address(s));

    let mut renderer = unsafe { GLRenderer::new_for_current_context((300, 300)) }.unwrap();

    renderer.draw_frame(|graphics| {
        graphics.draw_circle(Vector2::new(0., 0.), 10.0, Color::RED);
    });

    app.run().unwrap();
}
```





#![deny(warnings)]

use std::rc::Rc;

use speedy2d::color::Color;
use speedy2d::font::{Font, FormattedTextBlock, TextLayout, TextOptions};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main()
{
    simple_logger::SimpleLogger::new().init().unwrap();

    let window = Window::new_centered("Speedy2D: Hello World", (640, 240)).unwrap();

    let font = Font::new(include_bytes!("../assets/fonts/NotoSans-Regular.ttf")).unwrap();

    let text = font.layout_text("Hello world!", 64.0, TextOptions::new());

    window.run_loop(MyWindowHandler { text })
}

struct MyWindowHandler
{
    text: Rc<FormattedTextBlock>
}

impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::WHITE);

        graphics.draw_circle((150.0, 120.0), 75.0, Color::from_rgb(0.8, 0.9, 1.0));

        graphics.draw_text((290.0, 90.0), Color::BLACK, &self.text);

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }
}