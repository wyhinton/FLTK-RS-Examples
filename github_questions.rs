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