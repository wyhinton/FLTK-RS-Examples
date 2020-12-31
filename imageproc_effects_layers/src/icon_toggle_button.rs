use fltk::frame::*;
use fltk::{app::*, image::*, button::*, group::*, draw::*};
use std::ops::{Deref, DerefMut};
use std::path::Path;
#[derive(Clone)]
pub struct IconToggleButton {
    pub pack: Pack, 
    wid: CheckButton,
}

impl IconToggleButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, true_icon_path: &str, false_icon_path: &str) -> IconToggleButton {
        let mut x = IconToggleButton {
            pack: Pack::new(x,y,w, h, ""),
            wid: CheckButton::new(x, y, w, h, ""),
        };
        let mut true_image = SvgImage::load(true_icon_path).unwrap();
        let mut false_image = SvgImage::load(false_icon_path).unwrap();
    
        let mut tic_1 = true_image.clone();
        x.wid.draw2(move|w|{
            draw_box(
                FrameType::GtkDownBox,
                w.x(),
                w.y(),
                w.width(),
                w.height(),
                Color::from_u32(0xFFC300),
            );
            tic_1.draw(w.x(), w.y(), 24, 24);
        });
        x.pack.end(); 
        x.wid.handle2(move |t, ev| match ev {
            Event::Push => {
                if t.is_checked(){
                    let mut t_c1 = t.clone();
                    // t.set_image(Some(true_image.clone()));
                    let mut tic = true_image.clone();

                    // t.redraw();
                    t.draw2(move|t|{
                        draw_box(
                            FrameType::GtkDownBox,
                            t.x(),
                            t.y(),
                            t.width(),
                            t.height(),
                            Color::from_u32(0xFFC300),
                        );
                        tic.draw(t.x(), t.y(), 24, 24);
                    });
                } else {
                    let mut fic = false_image.clone();
                    t.draw2(move|t|{
                    draw_box(
                        FrameType::GtkDownBox,
                        t.x(),
                        t.y(),
                        t.width(),
                        t.height(),
                        Color::from_u32(0xFFC300),
                    );
                        fic.draw(t.x(), t.y(), 24, 24);
                    });

                }
                true
            }
            _ => false,
        });
   
        x
    }

    // Overrides the draw function
    // fn draw(&mut self) {
    //     self.wid.draw2(move |b| {
    //         draw_box(
    //             FrameType::GtkDownBox,
    //             b.x(),
    //             b.y(),
    //             b.width(),
    //             b.height(),
    //             Color::from_u32(0xFFC300),
    //         );
    //         set_draw_color(Color::Black);
    //         draw_text_angled(-15, &b.label(), b.x() + 2, b.y() + 15);
    //     });
    // }
    

    // Overrides the handle function.
    // Notice the do_callback which allows the set_callback method to work
    // fn handle(&mut self, mut true_icon: SvgImage, mut false_icon: SvgImage) {
    // // fn handle(&mut self, true_icon: <Option<SvgImage>>, false_icon: <Option<SvgImage>>) {
    //     let mut wid = self.wid.clone();
    //     self.wid.handle2(move |t, ev| match ev {
    //         Event::Push => {
    //             if t.is_checked(){
    //                 let t_c1 = t.clone();
    //                 t_c1.draw2(move|x|{
    //                       //         true_icon.draw(t.x(), t.y(), 24, 24);
    //                     draw_rect(t.x(), t.y(), 24, 24);
    //                 });
    //             }
    //             // t.draw(move||{
    //             //     if t.is_checked(){
    //             //         true_icon.draw(t.x(), t.y(), 24, 24);
    //             //     } else {
    //             //         false_icon.draw(t.x(), t.y(), 24, 24);
    //             //     }
    //             // });
    //             // println!("{}", t.is_checked());
    //             true
    //         }
    //         _ => false,
    //     });
    // }
}

impl Deref for IconToggleButton {
    type Target = CheckButton;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for IconToggleButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}

