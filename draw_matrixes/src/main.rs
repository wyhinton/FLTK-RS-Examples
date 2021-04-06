//demonstrates a fuzzy search bar 
use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, draw::*, button::*, input::*, group::*};
use rand::{distributions::Alphanumeric, Rng};


#[derive(Debug, Clone)]
pub enum Message {
    Test,   
}

fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let pack_width = 100;
    let pack_height = 200;
    
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    let centered_pack = Pack::new((win.width() - pack_width)/2, (win.height() - pack_height)/2, pack_width, pack_height, "");
        let mut frame = Frame::new(100,100, 100, 100, "");
        frame.set_frame(FrameType::FlatBox);
        frame.set_color(Color::Dark3);
        frame.draw2(move|widg|{
            //NOTE THAT YOU CAN NOT USE TRANSFORMS ON THE FAST SHAPES
            set_draw_color(Color::Yellow);
            push_matrix();
                begin_line();
                vertex(widg.x() as f64 -10.0, widg.y() as f64 +10.0);
                vertex(widg.x() as f64 -20.0, widg.y() as f64 +20.0);
                end_line();
                rotate(20.0);
                rotate(100.0);
            pop_matrix();
        });
    centered_pack.end();

    win.end();
    win.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg{
                Test => {
                    println!("{}", "got test message");
                    app::redraw();
                }
            }
        }
    }
}

