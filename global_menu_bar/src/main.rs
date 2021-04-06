//demonstrates a fuzzy search bar 
use fltk::*;
use fltk::{app::*, frame::*, window::*, menu::*};


#[derive(Debug, Clone)]
pub enum Message {
    Test,   
}

fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    // let menu = MenuBar::new(0,0,1000, 30, "");
    // menu.add("&File/New...\t", Shortcut::Ctrl | Shortcut::Shift | 'n', MenuFlag::Normal,  );
    // menu.add
    // menu.fn add<F: FnMut() + 'static>(
    //     &mut self,
    //     label: &str,
    //     shortcut: Shortcut,
    //     flag: MenuFlag,
    //     cb: F
    // )
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

