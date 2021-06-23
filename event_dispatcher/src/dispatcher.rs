/// An event dispatcher with a razor thin use case.
/// It needs to be decorated with error handling and possibly subscriber removal
/// to be of any real use.
pub struct Dispatcher<T>
where
    T: PartialEq + Eq + Clone,
{
    subscribers: Vec<Box<dyn FnMut(T)>>,
}

impl<T> Dispatcher<T>
where
    T: PartialEq + Eq + Clone + Sized,
{
    pub fn new() -> Self {
        Self { subscribers: Vec::new() }
    }
    /// Add a subscriber
    pub fn add_subscriber(&mut self, listener: Box<dyn FnMut(T)>) {
        self.subscribers.push(listener);
    }
    /// sends the event to all registered subscribers
    pub fn dispatch_event(&mut self, event: T) {
        for f in self.subscribers.iter_mut() {
            f(event.clone());
        }
    }
}

struct Adder {
    pack: Pack,
}

impl Adder {
    pub fn new(dispatcher: DispatcherType) -> Self {
        let mut container = Pack::new(80, 50, 200, 20, "Adder Widget");
        let mut button1 = Button::new(0, 0, 50, 20, "add1");
        let button1_id = 1;
        let dispatch_cl = Rc::clone(&dispatcher);
        button1.set_callback(move |_| {
            dispatch_cl
                .borrow_mut()
                .dispatch_event(CustomEvent::AddOne(button1_id));
        });
        let mut button2 = Button::new(0, 0, 50, 20, "add2");
        let button2_id = 2;
        let dispatch_cl = Rc::clone(&dispatcher);
        button2.set_callback(move |_| {
            dispatch_cl
                .borrow_mut()
                .dispatch_event(CustomEvent::AddTwo(button2_id));
        });
        let mut button3 = Button::new(0, 0, 50, 20, "add3");
        let button3_id = 3;
        let dispatch_cl = Rc::clone(&dispatcher);
        button3.set_callback(move |_| {
            dispatch_cl
                .borrow_mut()
                .dispatch_event(CustomEvent::AddThree(button3_id));
        });
        container.end();
        container.set_type(PackType::Horizontal);
        Adder { pack: container }
    }
}

struct MyWindow {}

/// the number inside can identify the origin widget
#[derive(Clone, Eq, PartialEq, Debug)]
enum CustomEvent {
    AddOne(i32),
    AddTwo(i32),
    AddThree(i32),
}
type DispatcherType = Rc<RefCell<Dispatcher<CustomEvent>>>;

impl MyWindow {
    pub fn new() -> Self {
        let dispatcher = Rc::from(RefCell::from(Dispatcher::<CustomEvent>::new()));
        let counter = Rc::from(RefCell::from(0_i32));

        //increment the counter value
        //see how any widget can intercept the event
        let counter_cl = Rc::clone(&counter);
        let f = move |event: CustomEvent| match event {
            CustomEvent::AddOne(wid) => {
                *counter_cl.borrow_mut() += 1;
                println!("event handled here. widget id:{}", wid);
            }
            CustomEvent::AddTwo(wid) => {
                *counter_cl.borrow_mut() += 2;
                println!("event handled here. widget id:{}", wid);
            }
            CustomEvent::AddThree(wid) => {
                *counter_cl.borrow_mut() += 3;
                println!("event handled here. widget id:{}", wid);
            }
        };
        dispatcher.borrow_mut().add_subscriber(Box::new(f));

        let mut win = Window::new(200, 200, 300, 200, "Ordinal Events");
        let mut disp_frame = Frame::new(200, 0, 200, 200, "0").center_of_parent();
        let _adder = Adder::new(Rc::clone(&dispatcher));

        let counter_cl = Rc::clone(&counter);
        //update our frames label with the counter value
        let f = move |event: CustomEvent| match event {
            CustomEvent::AddOne(wid) => {
                disp_frame.set_label(&*counter_cl.borrow().to_string());
                println!("also handled event here. widget id:{}", wid);
            }
            CustomEvent::AddTwo(wid) => {
                disp_frame.set_label(&*counter_cl.borrow().to_string());
                println!("also handled event here. widget id:{}", wid);
            }
            CustomEvent::AddThree(wid) => {
                disp_frame.set_label(&*counter_cl.borrow().to_string());
                println!("also handled event here. widget id:{}", wid);
            }
        };
        dispatcher.borrow_mut().add_subscriber(Box::new(f));

        win.end();
        win.show();


        MyWindow {}
    }
}




impl ConcurrentCounter {
        pub fn new(val: usize) -> Self {
            ConcurrentCounter(Arc::new(Mutex::new(val)))
        }
        pub fn increment(&self, by: usize) {
            let mut counter = self.0.lock().unwrap();
            *counter = *counter + by;
        }
        pub fn get(&self) -> usize {
            let counter = self.0.lock().unwrap();
            *counter
        }
    }