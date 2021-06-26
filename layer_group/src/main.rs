use fltk::{app::*, frame::*, button::*, draw, window::*, prelude::*, enums::*, widget::*};
use std::ops::{Deref, DerefMut};

pub struct MyFrame{
    frame: Widget
}

impl MyFrame{
    pub fn new(x: i32, y: i32, w: i32, h: i32)->Self{

        let mut frame= Widget::new(x,y,w,h,"myframe");
        frame.set_frame(FrameType::FlatBox);
        frame.set_color(Color::Blue);
        let mut my_widget = MyFrame{
            frame: frame
            
        };
        my_widget.draw();
        my_widget.handle();
        my_widget
    }
    fn handle(&mut self) {
        let mut wid = self.frame.clone();
        self.frame.handle(move |_, ev| match ev {
            Event::Push => {
                //doesn't fire
                dbg!("got push");
                wid.set_color(Color::Red);
                wid.do_callback();
                true
            }
            _ => false,
        });
    }
    fn draw(&mut self) {
        self.frame.draw(move |b| {
            draw::draw_box(
                FrameType::FlatBox,
                b.x(),
                b.y(),
                b.width(),
                b.height(),
                Color::from_u32(0x304FFE),
            );
            draw::set_draw_color(Color::White);
            draw::draw_text2(
                &b.label(),
                b.x(),
                b.y(),
                b.width(),
                b.height(),
                Align::Center,
            );
        });
    }
    pub fn on_enter<F: FnMut(&mut Widget) + 'static>(&mut self, mut cb: F){
        self.frame.handle(move|widg, ev| return match ev{
            Event::Enter=>{
                dbg!("got enter");
                cb(widg);
                true
            },
            _=>false
        });
    }
}

impl Deref for MyFrame {
    type Target = Widget;
    fn deref(&self) -> &Self::Target {
        &self.frame
    }
}
impl DerefMut for MyFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frame
    }
}

fn main() {
    let app = App::default();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    
    let mut my_layer_1 = MyFrame::new(100,100,100,50);
    my_layer_1.on_enter(move|widg|{
        widg.set_color(Color::Red);
        widg.redraw();
    }); 

    win.end();
    win.show();

    app.run().unwrap();
}

