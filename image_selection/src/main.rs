//demonstrates selecting a portion of an image, then drawing the selection to another frame
use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, draw::*};

mod maskable_image;
use maskable_image::MaskableImage;
use maskable_image::CropSegment;

pub mod utils;
use utils::*;

#[derive(Clone)]
pub enum Message {
    UpdatePreviewSegment(CropSegment),   
}

fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Image Mask, Crop, Copy");
    
    let miw = 300;
    let mih = 300;
    let mix = ((win.width()/2)-200) - miw/2;
    let miy = ((win.height()/2)) - mih/2;
    
    let my_img = MaskableImage::new(mix,miy,300,300, "imgs/tree_cop.png", s.clone());
    let mut prev = Frame::default().with_pos(mix+400, miy).with_size(300,300);

    // prev.set_frame(FrameType::FlatBox);
    prev.set_frame(FrameType::BorderFrame);
    prev.set_color(Color::Red);

    win.end();
    win.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg{
                UpdatePreviewSegment(mut prev_img) => {
                    prev.draw2(move |b|{
                        set_draw_color(Color::Red);
                        let mut prev  = RgbImage::new(&prev_img.img, prev_img.width as u32, prev_img.height as u32, 1).unwrap();
                        prev.draw(prev_img.x + (b.x()-(b.width()/2)), prev_img.y, prev_img.width, prev_img.height);
                        draw_rect(prev_img.x + (b.x()-(b.width()/2)), prev_img.y, prev_img.width, prev_img.height);
                        println!("x {}, y{}, w {}, h{}", prev_img.x, prev_img.y, prev_img.width, prev_img.height);
                    });
                    println!("{}", "shoudl update preview segment");
                    // prev.redraw();
                    app::redraw();
                }
            }
        }
    }
}

