//demonstrates a fuzzy search bar 
use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, table::*, draw::*, button::*, input::*, group::*};
use rand::{distributions::Alphanumeric, Rng};
// use noise::{Fbm, NoiseFn};
use noise::{utils::*, Fbm, Billow};

#[derive(Debug, Clone)]
pub enum Message {
    Test,   
}

fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    let mut frame = frame::Frame::new(0, 0, 400, 400, "");
    // 128px X 128px 
    let fbm = Fbm::new();

    let my_test = PlaneMapBuilder::new(&Billow::new())
        .build();

    let val = my_test.get_value(1,1);
    println!("my col {:?}", val );
    let s = my_test.size();
    println!("size is: {:?}", s);
    let (myx, myy ) = my_test.size();
    for i in 0..myx{
        for j in 0..myy{
            println!("coord is {}, {} val is: {}", i.to_string(), j.to_string(), my_test.get_value(i,j));
        }
    }
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

