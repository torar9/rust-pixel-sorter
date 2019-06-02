extern crate image;

use std::path::Path;

mod sorter;

fn buffer_into_file(pixels:& mut Vec<u8>, width:u32, height:u32)
{
    println!("Saving image...");
    image::save_buffer("result.png", pixels, width, height, image::RGB(8)).unwrap();
    println!("Image saved...");
}

fn print_buffer(pixels:& mut Vec<u8>)
{
    for i in 0..pixels.len()
    {
        print!("{} ", pixels.get(i).unwrap());
    }
    println!();
}


fn main()
{
    let mut input = String::new();
    println!("Enter image path:");
    match std::io::stdin().read_line(&mut input)
    {
        Ok(_) =>
        {
            let path = Path::new((&input).trim());
            let mut img = image::open(&path).unwrap().to_rgb();
            let (width, height) = img.dimensions();
            let mut pixels = img.to_vec();
            println!("size: {}", pixels.len());

            sorter::sort_by_red(& mut pixels);
            sorter::sort_by_green(& mut pixels);
            sorter::sort_by_blue(& mut pixels);
            buffer_into_file(& mut pixels, width, height);
        },
        Err(err) => {println!("Error: {}", err);}
    }
}
