// Simple app demonstrating demonstrating handling channeled events (which have the advantage of carrying data) as well as custom events, all the while keeping fn main() clean.
use crossbeam_channel::unbounded;
use fltk::{app, app::*, button::*, enums::*, frame::*, group::*, prelude::*, window::*};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

//define our applet, which contains some custom widgets
#[derive(Clone)]
struct CounterApplet {
    product: Rc<RefCell<i32>>,
}

pub struct CounterEvent {}

//define constants for our custom event
impl CounterEvent {
    const GET_PRODUCT: i32 = 42;
}

#[derive(Debug)]
enum CounterAppletEvent {
    Increment,
}

//counter applet wraps our two counter widgets
impl CounterApplet {
    pub fn new() -> Self {
        let (s, r) = unbounded::<CounterAppletEvent>();
        let _ss = s.clone();
        let val_1 = Rc::from(RefCell::from(0));
        let val_2 = Rc::from(RefCell::from(0));
        let mut container = Pack::new(150, 150, 200, 100, "CounterApplet ");
        let c_1 = Counter::new(val_1.clone(), s.clone());
        let c_2 = Counter::new(val_2.clone(), s);
        container.end();
        container.set_frame(FrameType::BorderFrame);
        container.set_color(Color::Red);
        container.set_type(PackType::Horizontal);

        let product = Rc::from(RefCell::from(*val_1.borrow() * *val_2.borrow()));
        let sum_pack = Pack::new(150, 350, 200, 20, "Counter Sums");
        let mut sum_frame = Frame::new(0, 0, 200, 20, "0");
        sum_pack.end();

        let c_1_cl = c_1.clone();
        let c_2_cl = c_2.clone();
        let prod_cl = product.clone();

        app::add_idle(move || {
            //using recv_timeout() is essential for saving CPU
            if let Ok(msg) = r.recv_timeout(Duration::from_millis(20)) {
                match msg {
                    CounterAppletEvent::Increment => {
                        let sum = [c_1_cl.value(), c_2_cl.value()].iter().sum::<i32>();
                        sum_frame.set_label(&sum.to_string());
                        *prod_cl.borrow_mut() = c_1_cl.value() * c_2_cl.value();
                    }
                }
            }
        });

        CounterApplet {
            product,
        }
    }
    pub fn product(&self) -> i32 {
        *self.product.borrow()
    }
}

//stateful counter widget
#[derive(Clone)]
struct Counter {
    val: Rc<RefCell<i32>>,
}

#[derive(Debug)]
enum CounterMessage {
    Increment(i32),
}

impl Counter {
    pub fn new(val: Rc<RefCell<i32>>, s: crossbeam_channel::Sender<CounterAppletEvent>) -> Self {
        let container = Pack::new(0, 0, 100, 100, None);
        let mut frame = Frame::new(0, 0, 50, 50, "0");
        let mut button = Button::new(0, 0, 50, 50, "Counter");
        container.end();

        //custom channel just for Counter Widget
        let (local_sender, r) = unbounded::<CounterMessage>();
        let ss = s.clone();
        button.set_callback(move |_| {
            ss.send(CounterAppletEvent::Increment).unwrap();
            local_sender.send(CounterMessage::Increment(1)).unwrap()
        });
        
        let val_cl = val.clone();
        app::add_idle(move || {
            if let Ok(msg) = r.recv_timeout(Duration::from_millis(20)) {
                match msg {
                    CounterMessage::Increment(step) => {
                        *val_cl.borrow_mut() += step;
                        frame.set_label(&*val_cl.borrow().to_string());
                    }
                }
            }
        });
        Counter {
            val,
        }
    }
    pub fn value(&self) -> i32 {
        return *self.val.borrow();
    }
}

fn main() {
    let app = App::default();
    let mut win = Window::default()
        .with_size(500, 500)
        .center_screen()
        .with_label("Crossbeam Channels");
    let counter_applet = CounterApplet::new();
    let mut get_product = Button::new(150, 100, 200, 20, "Get Product");
    let mut product_frame = Frame::new(150, 40, 200, 20, "0");

    get_product.set_callback(move |_| {
        let _ = app::handle_main(CounterEvent::GET_PRODUCT);
    });
    let counter_applet_cl = counter_applet;

    //access a function from our applet at a higher level, in this case our main window
    //this could just be nested inside of another group widget, or widget wrapper
    win.handle(move |_, ev| {
        if ev == CounterEvent::GET_PRODUCT.into() {
            dbg!("got get product event");
            dbg!(counter_applet_cl.product().to_string());
            product_frame.set_label(&counter_applet_cl.product().to_string());
            true
        } else {
            false
        }
    });

    win.end();
    win.show();
    app.run().unwrap();
}
