use fltk::{app::*, frame::*, window::*, image::*, table::*, button::*};
use super::Result;
use super::Message;

mod table_button;
use table_button::TableButton;

use std::ops::{Deref, DerefMut};




pub struct SearchTable{
    table: Table,
    values: Vec<Result>,
    items: Vec<String>
}
fn closest_multiple(mut n: i32, mut x: i32) -> i32 { 
    let result = ((n + x/2) / x) * x;

    // println!("x is {}, n is {}", x, n );
    // if (x > n){
    //     println!("{}", "it was less than");
    //     return x; 
    // } 
    // let mut tn = n; 
    // tn = n + (x / 2); 
    // // n = n - (n % x); 
    // tn = tn - (tn.rem_euclid(x)); 
    // println!("tn is {}", tn );
    return result; 
} 

fn format_table(num_cols: u32, p_width: i32, t: &Table, item_count: usize){
    let mut tc = t.clone();
    // println!("closet multiple is {}", closest_multiple(item_count as i32, num_cols as i32));
    println!("closet multiple is {}", closest_multiple(num_cols as i32, item_count as i32));
    tc.set_rows(20);
    tc.set_cols(num_cols);
    tc.set_row_height_all(90);
    // tc.set_col_width_all((p_width/num_cols as i32)-1);
    tc.set_col_width_all((p_width/num_cols as i32));
}

impl SearchTable{
    pub fn new(x: i32, y: i32, w: i32, h: i32, all_items: Vec<String>, s_items: Vec<Result>, s: fltk::app::Sender<Message>) -> SearchTable {
        let mut st = SearchTable{
            table: Table::new(x, y, w+20, h, ""),
            values: s_items.clone(), 
            items: all_items.clone(),
        };
        format_table(8, w, &st.table, all_items.len());        

        let ic = st.items.clone(); 
        st.table.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
            TableContext::Cell => {
                let t_index = row*t.cols() as i32+ col;
                // let mut button = TableButton::new(x,y,w,h, &all_items[t_index as usize]);
                let mut button = TableButton::new(x,y,w,h, &ic[t_index as usize]);
                t.add(&button.wid);
            },
            _ => ()
        });
        //we need to send a message to redraw the app after the table has been drawn
        s.send(Message::RedrawTable);
        st.table.end();

        st
    }
    //draw only the buttons with a match score greater than 0
    pub fn update(&mut self, search_results: Vec<Result>, s: fltk::app::Sender<Message>){
        fltk::GroupExt::clear(&mut self.table);
        //fix
        format_table(8, self.table.width(), &self.table, search_results.len());       

        self.table.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
            TableContext::Cell => {
                let t_index = row*t.cols() as i32+ col;
                if (t_index < search_results.len() as i32){
                    let cur_res = &search_results.clone()[t_index as usize];
                    if cur_res.score > 0 {
                        let mut button = TableButton::new(x,y,w,h, &cur_res.val.to_string());
                        &button.redraw();
                        t.add(&button.wid);
                        
                    }
                }
            },
            _ => ()
        });
        s.send(Message::RedrawTable);
        
    
    }
    //reset table to show all values
    pub fn reset(&mut self, s: fltk::app::Sender<Message>, all_items: Vec<String>){
        println!("{}", "should reset table" );
        format_table(8, self.table.width(), &self.table, all_items.len());       
        self.table.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
            TableContext::Cell => {
                let t_index = row*t.cols() as i32 + col;
                println!("all items len at reset is {}", all_items.len().to_string());
                if t_index < all_items.len() as i32 {
                    let mut button = TableButton::new(x,y,w,h, &all_items[t_index as usize]);
                    button.set_callback2(|b| println!("Selected: {}", b.label()));
                    t.add(&button.wid);  
                }
            },
            _ => ()
        });
        s.send(Message::RedrawTable);
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