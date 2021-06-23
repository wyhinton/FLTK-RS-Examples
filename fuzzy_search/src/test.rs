//demonstrates a fuzzy search bar 
use fltk::{ group::*, app::*, window::*, prelude::*, button::*, table::*, draw::*, enums::*, input::*, frame::*, image::*, app};
use rand::{distributions::Alphanumeric, Rng};

use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::cell::RefCell;
use sublime_fuzzy::best_match;

static INITIAL_COUNT: i32 = 20;

pub struct CustomEvent{}
impl CustomEvent {
    const SEARCH_INPUT: i32 = 42;
}

struct FuzzySearch{
}

impl FuzzySearch{
    pub fn new(w: i32, h: i32)->Self{
        let initial_strings = random_string_arr(INITIAL_COUNT, 5, 30); 
        let active_strings = Rc::from(RefCell::from(initial_strings.clone())); 
        let mut container= Pack::new(0,0,400,200,None).center_of_parent();
        let sb = SearchBar::new();
        let mut table = SearchTable::new(0,0+25, w, h-25, active_strings.clone()) ;
        container.end();
        let active_strins_cl = active_strings.clone();
        container.handle(move |_, ev| 
            if ev == CustomEvent::SEARCH_INPUT.into(){
            if sb.value().len() > 0{
                dbg!(sb.value().len());
                let values = fuzzy_search(sb.value(), active_strins_cl.try_borrow_mut().expect("hello").to_vec());
                table.set_values(values);
       
            } else {
                table.set_values(initial_strings.clone());
            }
            true
          } else {
              false
          });
        FuzzySearch{
        }
    }
}
fn main() {
    let app = App::default();
    let mut win = Window::new(200, 200, 700, 500, "Fuzzy Search");
    let _fuzzy_search = FuzzySearch::new(600,400);
    win.end();
    win.show();
    app.run().unwrap();
}

pub struct SearchTable{
    table: Table,
    items: Rc<RefCell<Vec<String>>>,
    buttons: Vec<Button>
}

impl SearchTable{
    pub fn new(x: i32, y: i32, w: i32, h: i32, items: Rc<RefCell<Vec<String>>>) -> SearchTable {
        //table setup
        let mut table = Table::new(x,y,w+20,h, None);
        table.end();
        table.set_rows(10);
        table.set_cols(5);
        table.set_row_height_all(90);
        
        

        //provide access to our strings inside of our table draw call
        let items_cl_1 = items.clone();
        table.draw(move|widg|{
            //provide access to our strings inside of draw_cell
            // let ic_cl_2 = items_cl_1.clone();
            draw_rect_fill(
                widg.x(),
                widg.y(),
                widg.width(),
                widg.height(),
                Color::Red,
            );
            //draw the children after drawing the background fill color
            widg.draw_children();
        });
        //draw our widgets again if we input into our search bar
        table.handle(move |widg, ev| 
          if ev == CustomEvent::SEARCH_INPUT.into(){
            // fltk::prelude::GroupExt::clear(widg); 
              widg.redraw();
            true
        } else {
            false
        });
        let mut buttons = vec![];
        for x in 0..100{
            buttons.push(Button::new(0,0,50,50, None));
        }
        let st = SearchTable{
            table: table,
            items: items.clone(),
            buttons: buttons,
        };

        st
    }
    //update our available list of strings
    pub fn set_values(&mut self, new_items: Vec<String>){
        *self.items.borrow_mut() = new_items;
        let ic_cl_2 = self.items.clone();
        // fltk::prelude::GroupExt::clear(self.table);//<---causes some kind of issue where the following code does not execute
        self.table.draw_cell(move |t, ctx, row, col, x, y, w, h| {
            if let TableContext::Cell = ctx {
                let t_index = row*t.cols() as i32+ col;
                //create a new button intance 
                //maybe our buttons could be cached so we don't have to call the constructor over and over
                if t_index < *&ic_cl_2.borrow().len() as i32{
                    let mut button = Button::new(x, y, w, h,None);
                    button.set_label(&ic_cl_2.borrow()[t_index as usize] );
                    t.add(&button);
                }
                dbg!(t.children());
            }
        });
    }

}

impl Deref for SearchTable {
    type Target = Table;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl DerefMut for SearchTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}


#[derive(Debug, Eq, Clone, Ord, PartialEq, PartialOrd)]
pub struct Result {
    pub score: isize,
    pub val: String,
}

pub struct SearchBar {
    pub pack: Pack, 
    pub input: Input,
    pub value: Rc<RefCell<String>>,
}

impl SearchBar {
    pub fn new() -> Self {
        //initialize widgets
        
        let search_val = Rc::from(RefCell::from("".to_string()));
        let mut container = Pack::new(0,0,0,25, None);
        let icon = Icon::new();
        //make our text input fill up the right amount of space
        let input_w = icon.parent().unwrap().parent().unwrap().width() - icon.width();
        let mut text_input = Input::new(0,0,input_w,25, None);

        //if our text input is not empty, draw some place-holder text
        text_input.draw(move |b|{
            set_draw_color(Color::Black);
            if b.value().is_empty() {
                draw_text2("Search...", b.x() + 10 , b.y(), b.width(), b.height(), Align::Inside | Align::Left );
            }

        });
        container.end(); 
        container.set_type(PackType::Horizontal);
        container.set_frame(FrameType::BorderFrame);
        container.set_color(Color::Black);

        let val_cl = Rc::clone(&search_val);
        text_input.handle(move |t, ev| {
            match ev {
                Event::KeyDown =>{
                    println!("{}", "got a key down" );
                    // dbg!(t.value());
                    *val_cl.borrow_mut() = t.value();
                    let _ = app::handle_main(CustomEvent::SEARCH_INPUT).unwrap();
                    true
                }
                _=>false 
            }
        });

        let sb = SearchBar {
            pack: container,
            input: text_input,
            value:  search_val,
        };
        sb
    }
    pub fn value(&self)->String{
        (*self.value.borrow().to_owned()).to_string()
    }
    
}
//search bar hour glass icon
struct Icon{
    frame: Frame
}

impl Icon{
    pub fn new()->Self{
        let icon_w = 24; 
        let h = 24; 
        let mut icon = Frame::new(0,0,icon_w,h, None);
        let mut search_icon = SvgImage::load("search-24px.svg").unwrap();
        icon.draw(move |f|{
            search_icon.draw(f.x(), f.y(), 24, 24);    
        });
        icon.set_frame(FrameType::NoBox); 
        Icon{
            frame: icon
        }
    }
}

impl Deref for Icon {
    type Target = Frame;

    fn deref(&self) -> &Self::Target {
        &self.frame
    }
}

impl DerefMut for Icon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frame
    }
}

fn fuzzy_search(search_value: String, items: Vec<String>)-> Vec<String> {
    let mut results = vec![];
    for x in 0..items.len(){
        let m = best_match(&search_value, &items[x]);
        //best_match returns an Option<Match> so we need to check for Some and None values
        match m {
            //if we get a match back push it to our result array
            Some(val) => {
                let res = Result{
                    score: val.clone().score(),
                    val: items[x].clone(),
                };
                dbg!(res.score);
                if res.score > 0.3 as isize{
                    results.push(res);
                }
            }
            //do nothing if result is none
            None => ()
        }
    }
    //sort the resulsts by their score (heigh->low)
    results.sort_by(|a, b| b.score.cmp(&a.score));
    //map the sorted list to the string values
    let result_strings: Vec<String> = results.into_iter().map(|x|x.val).collect();
    result_strings
}

fn random_string_arr(n_strings: i32, min_str_length: i32, max_str_length: i32) -> Vec<String>{
    let mut string_arr = vec![];
    
    for _x in 0..n_strings{
        let _count =  rand::thread_rng().gen_range(min_str_length..max_str_length);
        let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

        string_arr.push(s)
    }
    string_arr
 }



