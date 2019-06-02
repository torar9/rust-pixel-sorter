extern crate image;

use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;
use std::io::{BufRead, BufReader};

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

fn save_raw_buffer_into_file(pixels:& Vec<u8>, path:String)
{
    let path = Path::new(&path);
    match OpenOptions::new().create(true).write(true).truncate(true).append(false).open(&path)
    {
        Ok(mut file) =>
        {
            for i in (0..(pixels.len() - 3)).step_by(3)
            {
                write!(file, "{} {} {}\n", pixels.get(i).unwrap(), pixels.get(i + 1).unwrap(), pixels.get(i + 2).unwrap());
            }
            file.sync_all().unwrap();
        },
        Err(err) => {println!("Error: {}", err);}
    };
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

            sorter::sort_by_pixel_value(& mut pixels);
            //sorter::sort_by_sum_of_rgb(& mut pixels);
            //save_raw_buffer_into_file(&pixels, "data.txt".to_string());
            //sorter::sort_by_red(& mut pixels);
            //sorter::sort_by_green(& mut pixels);
            //sorter::sort_by_blue(& mut pixels);
            save_raw_buffer_into_file(& mut pixels, "after.txt".to_string());
            buffer_into_file(& mut pixels, width, height);
        },
        Err(err) => {println!("Error: {}", err);}
    }
}
