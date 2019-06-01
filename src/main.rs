extern crate image;

use std::path::Path;

fn bubble_sort(pixels:& mut Vec<u8>)
{
    println!("Sorting...");
    println!("size: {}", pixels.len());
    for i in (0..(pixels.len() - 5)).step_by(3)
    {
        for j in (0..(pixels.len() - i - 5)).step_by(3)
        {
            let left:u32 = (*pixels.get(j).unwrap() as u32) + (*pixels.get(j + 1).unwrap() as u32) + (*pixels.get(j + 2).unwrap() as u32);
            let right:u32 = (*pixels.get(j + 3).unwrap() as u32) + (*pixels.get(j + 4).unwrap() as u32) + (*pixels.get(j + 5).unwrap() as u32);
            if left < right
            {
                pixels.swap(j, j + 3);
                pixels.swap(j + 1, j + 4);
                pixels.swap(j + 2, j + 5);
            }
        }
    }
    println!("Sorting ended...");
}

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

            bubble_sort(& mut pixels);
            buffer_into_file(& mut pixels, width, height);
        },
        Err(err) => {println!("Error: {}", err);}
    }
}
