use super::ConfigVal;
use super::Operation;
use image::GenericImageView;
use image::imageops::*; 

pub fn add(val: f64, add_input_configs: Vec<ConfigVal>)->f64{
    println!("{}", add_input_configs[0].val); 
    let to_add = add_input_configs[0].val;
    return val + to_add; 
}

pub fn test_op(val: f64, test_input_conifgs: Vec<ConfigVal>)->f64{
    return val 
}

// pub fn contrast(&img: GenericImageView, contrast_amount: f64) -> GenericImageView{
//     let bf = image::imageops::contrast(&img, contrast_amount as f32);
//     return 
// }