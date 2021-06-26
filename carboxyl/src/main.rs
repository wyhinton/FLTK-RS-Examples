use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, table::*, button::*, input::*, group::*, prelude::*};
// #[macro_use(lift)]
use carboxyl::{Signal, Sink, lift};

#[derive(Debug, Clone)]
pub enum Message {
    Test,   
}

#[derive(Copy, Debug, Clone)]
enum Event {
    Add(i32),
    Drag(usize, i32),
    Drop,
}

fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    
    let sink = Sink::new();
    let rects = Signal::<Vec<i32>>::cyclic(|rects| {
        let events = sink.stream();
        let spawned = rects
            .snapshot(&events, |mut rects, ev|
                // dbg!(rects);
                match ev {
                Event::Add(r) => {  
                    dbg!(r);
                    rects.push(r);
                    rects
                }
                _ => rects,
            })
            .hold(vec![300]);

        events
            .filter_map({
                let spawned = spawned.clone();
                move |ev| match ev {
                    Event::Drag(idx, pos) => Some(lift!(
                        move |mut rects| {
                            rects[idx] += pos;
                            rects
                        },
                        &spawned
                    )),
                    Event::Drop => Some(spawned.clone()),
                    _ => None,
                }
            })
            .hold(spawned.clone())
            .switch()
    });

    let mut button= Button::new(200,200,200,200,"press me");
    button.set_callback(move|widg|{
        sink.send(Event::Add(61));
    });

    // The current value of the signal

    win.end();
    win.show();

    while app.wait() {
    }
}

