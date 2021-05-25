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


fn counter_reducer(state: &AppState, action: &Action) -> AppState {
    match action {
        Action::DeleteCard(id) => {
            dbg!("got add action");
            let mut cc = state.cards.clone();
            cc.retain(|x| x.id != *id);
            return AppState{
                cards: cc
            }
        }
        Action::AddCard => AppState {
            cards: state.cards.clone()
        }
    }
}


//main
```lang-rust
fn main() {
    let app = App::default();
    let mut store = Store::new(counter_reducer, AppState::default());
    let (s, r) = channel::<Action>();
    let mut win = Window::new(200, 200, 1000, 1000, "");
    
    let mut controls = Pack::new(100,80,100,20,"");
    let mut add_button = Button::new(0,0,100,20,"New Random Image");
    add_button.emit(s.clone(), Action::AddCard);
    controls.end();

    let mut gallery = Pack::new((WINDOW_WIDTH-GALLERY_WIDTH)/2,100,800,300, "");
        let card_1 = Card::new("imgs/chrome.jpg", s.clone()).unwrap();
        let card_2 = Card::new("imgs/forest.jpg", s.clone()).unwrap();
        let card_3 = Card::new("imgs/smiley.png", s.clone()).unwrap();
    gallery.end();
    gallery.set_type(PackType::Horizontal);
    // gallery.set_frame(FrameType::BorderFrame);
    // gallery.set_color(Color::Red);
    win.end();
    win.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Action::*;
            match msg{
                DeleteCard(id) => {
                    dbg!("should delete card");
                    store.dispatch(Action::DeleteCard(id));
                }
                AddCard=>{
                    dbg!("should add card");
                    store.dispatch(Action::AddCard);
                }
            }
        }
    }
}
```

```lang-rust
//card.rs
impl Card {
    pub fn new(img_path: &str, s: fltk::app::Sender<Action>) -> Result<Self, Box<dyn Error>>  {
        let mut card_container = Pack::new(0,0,100,100,"");
        let mut img_frame = Frame::new(0,0,100,100,"");
        let mut delete_button = Button::new(0,0,100,BUTTON_HEIGHT,"X");
        img_frame.set_frame(FrameType::FlatBox);
        img_frame.set_color(Color::Blue);
        card_container.end();
        let id = Uuid::new_v4();
        delete_button.emit(s, Action::DeleteCard(id));
        let img = ImageReader::open(img_path)?
            .with_guessed_format()?
            .decode()?;
        let resized = img.resize(400,IMAGE_HEIGHT, image::imageops::FilterType::Triangle);
        let (w,h) = resized.dimensions();
        card_container.resize(img_frame.x(), img_frame.y(), w as i32, h as i32 + BUTTON_HEIGHT);
        img_frame.resize(img_frame.x(), img_frame.y(), w as i32, IMAGE_HEIGHT as i32);
        img_frame.redraw();
        img_frame.draw2(move |b|{
            draw::draw_image(&resized.to_rgb8(), b.x(), b.y(), w as i32, h as i32, ColorDepth::Rgb8).unwrap();
        });

        let mut sb = Card {
            pack: card_container,
            frame: img_frame,
            id: id,
        };
        sb.frame.redraw();
            
        Ok(sb)
    }
    pub fn delete(&mut self){
        // self.pack.delete();
        fltk::WidgetBase::delete(self.pack.clone());
    }
}
```

I'm using the RefCell pattern for interior mutability of my apps state. I'm running in a verbosity problem:
______________________________________________________________________
```
//demonstrates a fuzzy search bar 

#[derive(Debug, Clone)]
pub struct Gallery{
    pack: t,
    cards: Vec<Card>,
}

impl Gallery{
    fn new(cards: Vec<Card>, x: i32, y: i32, w: i32, h:i32)->Gallery{
        let mut gallery_container = Pack::new(x,y,w,h, "");
        gallery_container.end();
        gallery_container.set_type(PackType::Horizontal);
        
        for x in 0..cards.len(){
            gallery_container.add(&cards[x].pack);
        }
        let g = Gallery{
            pack: gallery_container,
            cards: cards.clone(), 
        };
        g
    }
}
#[derive(Debug, Clone)]
pub struct AppState{
    gallery: Gallery,
    sender: fltk::app::Sender<Action>,
}
type AppRef = Rc<RefCell<AppState>>;
#[derive(Clone)]
struct AppContainer(AppRef);

impl AppContainer{
    fn new(inner: AppState) -> AppContainer{
        let app = AppState{
            gallery: inner.gallery.clone(),
            // cards: inner.cards.clone(),
            sender: inner.sender.clone(),
            
        };
        return AppContainer(Rc::new(RefCell::new(app)));
    }
    fn delete_card(&self, id: Uuid) {
        (self.0.borrow_mut()).gallery.cards.retain(|x| x.id != id)
    }
    fn add_card(&self, path: String){
        let s = (self.0.borrow_mut()).sender.clone();
        let new_card = Card::new(&path, s.clone()).unwrap();
        (self.0.borrow_mut()).gallery.cards.push(new_card);
    }
}

#[derive(Debug, Clone)]
pub enum Action{
    DeleteCard(Uuid),
    AddCard(String),
}


fn main() {
    let app = App::default();
    let (s, r) = channel::<Action>();

    // GLOBAL.set(initial_app_state);
    let mut win = Window::new(200, 200, WINDOW_WIDTH, WINDOW_HEIGHT, "");
    
    let mut controls = Pack::new(100,80,150,20,"");
    let mut add_button = Button::new(0,0,100,20,"New Random Image");
    add_button.set_color(Color::Green);
    add_button.emit(s.clone(), Action::AddCard("imgs/chrome.jpg".to_string()));
    controls.end();

    let def_cards = vec![
        Card::new("imgs/chrome.jpg", s.clone()).unwrap(),
        Card::new("imgs/forest.jpg", s.clone()).unwrap(),
        Card::new("imgs/smiley.png", s.clone()).unwrap(),
    ];


    let gallery = Gallery::new(def_cards, (WINDOW_WIDTH-GALLERY_WIDTH)/2,100,800,300);
    win.end();
    win.show();

    let initial_app_state = AppState{
        gallery: gallery,
        sender: s.clone(),
    };
    let container = AppContainer::new(initial_app_state.clone());

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Action::*;
            match msg{
                DeleteCard(id) => {
                    container.delete_card(id);
                    dbg!("should delete card");
                }
                AddCard(path)=>{
                    container.add_card("imgs/chrome.jpg".to_string());
                    dbg!("should add card");
                }
            }
        }
    }
}




I'm creating an app with rust and require undo/redo functionality. The [redo crate][1] seems like 
it offers some nice functionality to do this, but I can figure out how to apply the simple example 
to a more complex setup, where ```AppState``` is stored inside of a RefCell ```AppContainer```. When AppContainer
mutates AppState, I want to be able to undo/redo that mutation. 

```lang-rust
use undo::{Action, History};
use std::cell::RefCell;
use std::rc::Rc;

//this is my app states, containing all the state data
#[derive(Clone, Debug)]
struct AppState{
    names: Vec<String>
    //...
}

//App container provides interior mutability
struct AppContainer(Rc<RefCell<AppState>>);

//various functions for mutating AppState, each of which I would like to be able to undo/redo 
impl AppContainer{
    fn new (inner: AppState)->AppContainer{
        let cl = AppState{
            names: inner.names.clone(),
        };
        return AppContainer(Rc::new(RefCell::new(cl)));
    }
    fn add_name(&self, name: String){
        (self.0.borrow_mut()).names.push(name);
    }
    fn remove_name(&self, to_remove: String)->{
        (self.0.borrw_mut()).retain(|x| x != to_remove)
    }
}

struct AddName(name);
impl Action for AddName{
    type Target = std::Ref<AppState>;
    type Output = ();
    type Error = &'static str;

    fn apply(&mut self, s: std::Ref<AppState>) -> undo::Result<Add> {
        s.names.push()
        s.push(self.0);
        Ok(())
    }

    fn undo(&mut self, s: std::Ref<AppState>) -> undo::Result<Add> {
        self.0 = s.pop().ok_or("s is empty")?;
        Ok(())
    }
    
}

B

fn main()->undo::Result<AppContainer>{
    let initial_state = AppState{
        names: vec!["name 1".to_string(), "name 2".to_string()],
    };
    let mut history = History::new();
    let container = AppContainer::new(initial_state.clone());


    history.apply(&mut container, add_name("My Name")).unwrap();
    history.apply(&mut container, remove_name("name ")).unwrap();
    history.undo(&mut container, ).unwrap()
    Ok(())

}
```


  [1]: https://crates.io/crates/redo



  
I'm creating an app with rust and require undo/redo functionality. The [redo crate][1] seems like 
it offers some nice functionality to do this, but I can figure out how to apply the simple example 
to a more complex setup, where ```AppState``` is stored inside of a RefCell ```AppContainer```. When AppContainer
mutates AppState, I want to be able to undo/redo that mutation.

Here I tried to impliment this, but I dont think this is the correct route. For one ```apply()``` expects a mutable reference, but 
I dont know how to both mutate the container and send the data that needed for the mutation. 

So what would be a correct way of mutating the state while enabling undo/redo?

```lang-rust
use undo::{Action, History};
use std::cell::RefCell;
use std::rc::Rc;

//this is my app states, containing all the state data
#[derive(Clone, Debug)]
struct AppState{
    names: Vec<String>
    //...
}

//App container provides interior mutability
struct AppContainer(Rc<RefCell<AppState>>);

//various functions for mutating AppState, each of which I would like to be able to undo/redo 
impl AppContainer{
    fn new (inner: AppState)->AppContainer{
        let cl = AppState{
            names: inner.names.clone(),
        };
        return AppContainer(Rc::new(RefCell::new(cl)));
    }
    fn add_name(&self, name: String){
        (self.0.borrow_mut()).names.push(name);
    }
    fn remove_name(&self, to_remove: String){
        (self.0.borrow_mut()).names.retain(|x| *x != to_remove)
    }
    fn names(&self)->Vec<String>{
        (self.0.borrow()).names
    }
}

//Action for adding a name
struct AddName(String);
impl Action for AddName{
    type Target = (String, AppContainer);
    type Output = ();
    type Error = &'static str;

    fn apply(&mut self, s: (String, &mut AppContainer)) -> undo::Result<AddName> {
        s.1.add_name(s.0);
        Ok(())
    }

    fn undo(&mut self, s: (String, &mut AppContainer)) -> undo::Result<AddName> {
        s.1.remove_name(s.0);
        Ok(())
    }
    
}
//Action for removing a name
struct RemoveName(String);
impl Action for RemoveName{
    type Target = (String, AppContainer);
    type Output = ();
    type Error = &'static str;

    fn apply(&mut self, s: (String, &mut AppContainer)) -> undo::Result<RemoveName> {
        s.1.remove_name(s.0);
        Ok(())
    }

    fn undo(&mut self, s: (String, &mut AppContainer)) -> undo::Result<RemoveName> {
        s.1.add_name(s.0);
        Ok(())
    }
    
}


fn main(){
    type Target = (String, AppContainer);
    type Output = ();
    type Error = &'static str;

    let initial_state = AppState{
        names: vec!["name 1".to_string(), "name 2".to_string()],
    };
    let mut history = History::new();
    let container = AppContainer::new(initial_state.clone());
    history.apply(&mut container, AddName("My Name".to_string())).unwrap();
    dbg!(container.names());
    history.apply(&mut container, RemoveName("Name 1".to_string())).unwrap();
    dbg!(container.names());
    //Errors becuase apply expects a mutable reference, not a tuple
}


```

Example from the undo repo:
```lang-rust
use undo::{Action, History};

struct Add(char);

impl Action for Add {
    type Target = String;
    type Output = ();
    type Error = &'static str;

    fn apply(&mut self, s: &mut String) -> undo::Result<Add> {
        s.push(self.0);
        Ok(())
    }

    fn undo(&mut self, s: &mut String) -> undo::Result<Add> {
        self.0 = s.pop().ok_or("s is empty")?;
        Ok(())
    }
}

fn main() -> undo::Result<Add> {
    let mut target = String::new();
    let mut history = History::new();
    history.apply(&mut target, Add('a'))?;
    history.apply(&mut target, Add('b'))?;
    history.apply(&mut target, Add('c'))?;
    assert_eq!(target, "abc");
    history.undo(&mut target)?;
    history.undo(&mut target)?;
    history.undo(&mut target)?;
    assert_eq!(target, "");
    history.redo(&mut target)?;
    history.redo(&mut target)?;
    history.redo(&mut target)?;
    assert_eq!(target, "abc");
    Ok(())
}

```

  [1]: https://crates.io/crates/redo






  

impl AppContainer{
    fn new(inner: AppState) -> AppContainer{
        let app = AppState{
            gallery: inner.gallery.clone(),
            // cards: inner.cards.clone(),
            sender: inner.sender.clone(),
            
        };
        return AppContainer(Rc::new(RefCell::new(app)));
    }
    fn delete_card(&self, id: Uuid)->Result<History> {
        (self.0.borrow_mut()).gallery.cards.retain(|x| x.id != id)
    }
    fn add_card(&self, path: String)->Result<History>{
        let s = (self.0.borrow_mut()).sender.clone();
        let new_card = Card::new(&path, s.clone()).unwrap();
        (self.0.borrow_mut()).gallery.cards.push(new_card);
    }
}




I'm trying to impliment an undo/redo system for my app via the undo crate. I want a globally accessbile ```HISTORY```
variable to which I can push ```Box<dyn Action>``` trait objects.
```
use undo::{Action, History};

lazy_static::lazy_static! {
    static ref HISTORY: History<Box<dyn Action<Output = (), Error = String, Target = AppState>>> = History::new();
}

```
This gives error:
```
error[E0038]: the trait `Action` cannot be made into an object
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   --> C:\Users\Primary User\.cargo\registry\src\github.com-1ecc6299db9ec823\undo-0.46.0\src\lib.rs:135:8
    |
135 |     fn merge(&mut self, _: &mut Self) -> Merged {
    |        ^^^^^ the trait cannot be made into an object because method `merge` references the `Self` type in this parameter
```
Is there any way to work around this? 


I'm trying to implement a globally accessible undo/redo functionality in my Rust app via the undo crate. I start by creating a lazy static which contains my app's data model, ```APP_STATE:AppState```, which has field ```history``` of
type ```undo::history::History```. Now my problem is that lazy static requires ```Send+Sync```, and while I'm able to define the proper trait 
constraints in the ```AppState``` definition (```History<Box<dyn AppAction>, Box<dyn FnMut(Signal) + Send + Sync>>```), I don't see any option for adding these traits when I call ```History::new()``` 
inside of ```AppState::new()```:
```lang-rust
use undo::{Action, History, Merged, Signal};
use enum_dispatch::enum_dispatch;
lazy_static::lazy_static!{
    static ref APP_STATE: AppState = AppState::new();
}

pub struct AppState{
    names: Vec<String>,
    history: History<Box<dyn AppAction>, Box<dyn FnMut(Signal) + Send + Sync>>
    //correct trait constraints
}

impl AppState{
    pub fn new()->Self{
        AppState{
            names: vec![],
            history: History::new(),
            //not sure how to add these trait constraints here
        }
    }
    
}
//..
```
Gives error:
```
history: History::new(),
         ^^^^^^^^^^^^^^ expected trait `FnMut(Signal) + Send + Sync`, found trait `FnMut(Signal)`

```
If ```History::new()``` does not provide any way for me to add ```Send+Sync```, does that mean I manually instantiate a ```History``` struct? What are some work around for this problem?
```lang-rust
impl AppState{
    pub fn new()->Self{
        AppState{
            names: vec![],
            history: History{
                //field 1
                //field 2
                //..
            },
        }
    }
}
```


If you wanted to change the color of a single pixel in the center of an image via the image crate, how might you do that?
```
use image::*;
let img =  image.open()


I'm trying to implement a globally accessible undo/redo functionality in my Rust app via the undo crate. Additionaly I am using enum_dispatch to handle the dynamic dispatch of my ```dyn AppActions```,
like ```AddName```. I start by creating a lazy static which contains my app's data model, ```APP_STATE:AppState```, which has field ```history``` of
type ```undo::history::History```. In order to give History the correct trait constraints, I need to use ```undo::record::Builder``` to create a ```Record<AppActionEnum,  Box<dyn FnMut(Signal) + Send + Sync>>```:

```lang-rust
use undo::{Action, History, Merged, Signal, record::Builder};
use enum_dispatch::enum_dispatch;
lazy_static::lazy_static!{
    static ref APP_STATE: AppState = AppState::new();
}

pub struct AppState{
    names: Vec<String>,
    history: History<AppActionEnum, Box<dyn FnMut(Signal) + Send + Sync>>
}

impl AppState{
    pub fn new()->Self{
        let test = AddName("test".to_string());

         //build should be of type <AppActionEnum, Box<dyn FnMut(Signal) + Send + Sync>>
         let build = Builder::new()
             .limit(100)
             .capacity(100)
            //  .connect(|s| { dbg!(s); })
             .connect(Box::from(|s| { dbg!(s); })) gives error:
             //"expected trait object `dyn FnMut`, found closure"
             .build::<AppActionEnum>();
    
        AppState{
            names: vec![],
            history: History::from(build),
        }
    }
    //..
}

pub struct AddName(String);
impl AddName{
    pub fn new(name: String) -> Self{
        AddName(name.clone())
    }
}

#[enum_dispatch]
pub enum AppActionEnum {
    DoAddName(AddName),
}

#[enum_dispatch(AppActionEnum)]
pub trait AppAction: Send + Sync{
    //..
}

impl AppAction for AddName{
    //..
}

impl Action for AppActionEnum {
    //..
} 
```
However this yields error:
```
error[E0308]: mismatched types
  --> src\main.rs:31:22
24 |              .connect(|s| { dbg!(s); })
   |                       ---------------- the found closure
...
31 |             history: History::from(build),
   |                      ^^^^^^^^^^^^^^^^^^^^ expected struct `Box`, found closure

```
How do I pass a Box<(dyn FnMut(Signal) + Send + Sync + 'static)>> instead of a closure? I don't understand how what I'm doing here is different than whats shown in the example:
```lang-rust
 //Builder for a Record.
 use undo::{Action, record::Builder, Record};
 struct Add(char);
 impl Action for Add {
     type Target = String;
     type Output = ();
     type Error = &'static str;
     fn apply(&mut self, s: &mut String) -> undo::Result<Add> {
         s.push(self.0);
         Ok(())
     }
     fn undo(&mut self, s: &mut String) -> undo::Result<Add> {
         self.0 = s.pop().ok_or("s is empty")?;
         Ok(())
     }
 }
 let _ = Builder::new()
     .limit(100)
     .capacity(100)
     .connect(|s| { dbg!(s); })
     .build::<Add>();
```

I frequently need to find then remove something from a struct field. Does the following represent an idiomatic apporach to doing this?
```lang-rust
struct Roster{
    names: Vec<String>
    //..
}

impl Roster{
    pub fn remove_name(&mut self, name: &str){
        //are both of these lines necessary or could they be reduce to a single linge?
        let to_keep =  self.current.names.clone().into_iter().filter(|n|*n != name).collect();
        self.current.names = keep
    }
}

fn main(){
    let mut r = Roster{
        vec!["P1".to_string(), "P2".to_string()],
    }
    r.remove_name("P1")
}
```

```

fn main(){
    
}

```

I'm writing a function which searches for an element, then performs some kind of operation on that found element:

```
fn search_and_call<T>(obj_list: &mut Vec<T>,
    mut condition: impl FnMut(&mut T) -> bool,
    mut func: impl FnMut(&mut T) -> ()) {
for x in obj_list {
if condition(x) {
func(x);
}
}
}
```
What if the types I'm using in my condition do not impliment ```Eq```? Is there an idiomatic way to write such a function without
implimenting ```Eq``` for my enum?

```

pub enum Button{
    Round,
    Square
}

pub struct Button{
    ind: i32,
    kind: Button,
}

pub struct ButtContainer{
    buttons: Vec<Button>
}

fn main(){
    let buttons = vec![
        Button{
            ind: 0,
            kind: Round
        },
        Button{
            ind: 0,
            kind: Square
        },
    ]
    let cont = ButtContainer{
        buttons: str
    }
}

```
