use fltk::{prelude::*, *, image::*};

// const IMG: &str = include_str!("./search-24px.svg");

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(402, 275);
    let mut table = table::Table::default().size_of(&wind);

    table.set_rows(8);
    table.set_cols(5);
    table.set_row_height_all(90);
    table.set_col_width_all(80);

    table.end();

    let mut img = SvgImage::load("search-24px.svg").unwrap();

    table.draw_cell(move |t, ctx, row, col, x, y, w, h| {
        if let table::TableContext::Cell = ctx {
            img.scale(w, h - 20, true, false);
            let mut button = button::Button::new(x, y, w, h, None);
            button.set_label(&format!("Image {}", row + col));
            button.set_align(enums::Align::Bottom | enums::Align::Inside);
            button.set_frame(enums::FrameType::FlatBox);
            button.set_image(Some(img.clone()));
            button.set_callback(|b| println!("Selected: {}", b.label()));
            t.add(&button);
        }
    });

    wind.make_resizable(true);
    wind.end();
    wind.show();

    while table.children() == 0 {
        app::wait();
    }

    app::redraw();

    app.run().unwrap();
}

// //demonstrates a fuzzy search bar 
// use fltk::{ group::*, app::*, window::*, prelude::*, enums::*};
// use rand::{distributions::Alphanumeric, Rng};

// mod search_bar;
// use search_bar::SearchBar;
// use search_bar::Result;

// mod search_table;
// use search_table::SearchTable;
// use std::rc::Rc;
// use std::cell::RefCell;
// use sublime_fuzzy::best_match;

// static INITIAL_COUNT: i32 = 100;

// pub struct CustomEvent{}
// impl CustomEvent {
//     const SEARCH_INPUT: i32 = 42;
// }

// struct FuzzySearch{
// }

// impl FuzzySearch{
//     pub fn new(w: i32, h: i32)->Self{
//         let initial_strings = random_string_arr(INITIAL_COUNT, 5, 30); 
//         let active_strings = Rc::from(RefCell::from(initial_strings.clone())); 
//         let mut container= Pack::new(0,0,400,200,None).center_of_parent();
//         let sb = SearchBar::new();
//         let mut table = SearchTable::new(0,0+25, w, h-25, initial_strings.clone()) ;
//         container.end();
//         container.resize(container.x(), container.y()-100, container.width(), container.height());
//         container.redraw();
//         let active_strins_cl = active_strings.clone();
//         // table.
//         container.handle(move |w, ev| 
            
//             if ev == CustomEvent::SEARCH_INPUT.into(){
//                 // let mut damage = w.damage_type();
//                 // damage &= Damage::Scroll;
//                 // if damage == Damage::Scroll {
//                 //     println!("Window received a Scroll damage and was redrawn");
//                 // }
//                 w.redraw();
//             if sb.value().len() > 0{
//                 dbg!(sb.value().len());
//                 let values = fuzzy_search(sb.value(), active_strins_cl.try_borrow_mut().expect("hello").to_vec());
//                 table.set_values(values);
       
//             } else {
//                 table.set_values(initial_strings.clone());
//             }
//             true
//           } else {
//               false
//           });
//         FuzzySearch{
//         }
//     }
// }
// fn main() {
//     let app = App::default();
//     let mut win = Window::new(200, 200, 700, 600, "Fuzzy Search");
//     let _fuzzy_search = FuzzySearch::new(600,400);
//     win.end();
//     win.show();
//     app.run().unwrap();

// }
// fn fuzzy_search(search_value: String, items: Vec<String>)-> Vec<String> {
//     let mut results = vec![];
//     for x in 0..items.len(){
//         let m = best_match(&search_value, &items[x]);
//         //best_match returns an Option<Match> so we need to check for Some and None values
//         match m {
//             //if we get a match back push it to our result array
//             Some(val) => {
//                 let res = Result{
//                     score: val.clone().score(),
//                     val: items[x].clone(),
//                 };
//                 // dbg!(res.score);
//                 if res.score > 0.3 as isize{
//                     results.push(res);
//                 }
//             }
//             //do nothing if result is none
//             None => ()
//         }
//     }
//     //sort the resulsts by their score (heigh->low)
//     results.sort_by(|a, b| b.score.cmp(&a.score));
//     //map the sorted list to the string values
//     let result_strings: Vec<String> = results.into_iter().map(|x|x.val).collect();
//     result_strings
// }

// fn random_string_arr(n_strings: i32, min_str_length: i32, max_str_length: i32) -> Vec<String>{
//     let mut string_arr = vec![];
    
//     for _x in 0..n_strings{
//         let _count =  rand::thread_rng().gen_range(min_str_length..max_str_length);
//         let s: String = rand::thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(7)
//         .map(char::from)
//         .collect();

//         string_arr.push(s)
//     }
//     string_arr
//  }