#[allow(dead_code)]
pub fn sort_by_sum_of_rgb(pixels:& mut Vec<u8>)
{
    println!("Sorting...");
    for i in (0..(pixels.len() - 5)).step_by(3)
    {
        for j in (0..(pixels.len() - i - 5)).step_by(3)
        {
            let left:u16 = (*pixels.get(j).unwrap() as u16) + (*pixels.get(j + 1).unwrap() as u16) + (*pixels.get(j + 2).unwrap() as u16);
            let right:u16 = (*pixels.get(j + 3).unwrap() as u16) + (*pixels.get(j + 4).unwrap() as u16) + (*pixels.get(j + 5).unwrap() as u16);
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

#[allow(dead_code)]
pub fn sort_by_red(pixels:& mut Vec<u8>)
{
    println!("Sorting...");
    for i in (0..(pixels.len() - 5)).step_by(3)
    {
        for j in (0..(pixels.len() - i - 5)).step_by(3)
        {
            if (*pixels.get(j).unwrap() as u16) < (*pixels.get(j + 3).unwrap() as u16)
            {
                pixels.swap(j, j + 3);
                pixels.swap(j + 1, j + 4);
                pixels.swap(j + 2, j + 5);
            }
        }
    }
    println!("Sorting ended...");
}

#[allow(dead_code)]
pub fn sort_by_green(pixels:& mut Vec<u8>)
{
    println!("Sorting...");
    for i in (0..(pixels.len() - 5)).step_by(3)
    {
        for j in (0..(pixels.len() - i - 5)).step_by(3)
        {
            if (*pixels.get(j + 1).unwrap() as u16) < (*pixels.get(j + 4).unwrap() as u16)
            {
                pixels.swap(j, j + 3);
                pixels.swap(j + 1, j + 4);
                pixels.swap(j + 2, j + 5);
            }
        }
    }
    println!("Sorting ended...");
}

#[allow(dead_code)]
pub fn sort_by_blue(pixels:& mut Vec<u8>)
{
    println!("Sorting...");
    for i in (0..(pixels.len() - 5)).step_by(3)
    {
        for j in (0..(pixels.len() - i - 5)).step_by(3)
        {
            if (*pixels.get(j + 2).unwrap() as u16) < (*pixels.get(j + 5).unwrap() as u16)
            {
                pixels.swap(j, j + 3);
                pixels.swap(j + 1, j + 4);
                pixels.swap(j + 2, j + 5);
            }
        }
    }
    println!("Sorting ended...");
}

#[allow(dead_code)]
pub fn sort_by_pixel_value(pixels:& mut Vec<u8>)
{
    println!("Sorting...");
    for i in (0..(pixels.len() - 5)).step_by(3)
    {
        for j in (0..(pixels.len() - i - 5)).step_by(3)
        {
            let mut left:u32 = *pixels.get(j).unwrap() as u32;  //rgb(*255*,254, 253),  left: (0000 0000, 0000 0000, 1111 1111)
            left <<= 8;                                   //left << 0000 0000,    left: (0000 0000, 1111 1111, 0000 0000)
            left |= *pixels.get(j + 1).unwrap() as u32;   //rgb( 255,*254*,253),  left: (0000 0000, 1111 1111, 1111 1110)
            left <<= 8;                                   //left << 0000 0000,    left: (1111 1111, 1111 1110, 0000 0000)
            left |= *pixels.get(j + 2).unwrap() as u32;   //rgb( 255, 254,*253*), left: (1111 1111, 1111 1110, 1111 1101)

            let mut right:u32 = *pixels.get(j + 3).unwrap() as u32;
            right <<= 8;
            right |= *pixels.get(j + 4).unwrap() as u32;
            right <<= 8;
            right |= *pixels.get(j + 5).unwrap() as u32;
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
