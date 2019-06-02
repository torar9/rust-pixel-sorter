pub fn sort_by_sum_of_rgb(pixels:& mut Vec<u8>)
{
    println!("Sorting...");
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

pub fn sort_by_red(pixels:& mut Vec<u8>)
{
    println!("Sorting...");
    for i in (0..(pixels.len() - 5)).step_by(3)
    {
        for j in (0..(pixels.len() - i - 5)).step_by(3)
        {
            if (*pixels.get(j).unwrap() as u32) < (*pixels.get(j + 3).unwrap() as u32)
            {
                pixels.swap(j, j + 3);
                pixels.swap(j + 1, j + 4);
                pixels.swap(j + 2, j + 5);
            }
        }
    }
    println!("Sorting ended...");
}

pub fn sort_by_green(pixels:& mut Vec<u8>)
{
    println!("Sorting...");
    for i in (0..(pixels.len() - 5)).step_by(3)
    {
        for j in (0..(pixels.len() - i - 5)).step_by(3)
        {
            if (*pixels.get(j + 1).unwrap() as u32) < (*pixels.get(j + 4).unwrap() as u32)
            {
                pixels.swap(j, j + 3);
                pixels.swap(j + 1, j + 4);
                pixels.swap(j + 2, j + 5);
            }
        }
    }
    println!("Sorting ended...");
}

pub fn sort_by_blue(pixels:& mut Vec<u8>)
{
    println!("Sorting...");
    for i in (0..(pixels.len() - 5)).step_by(3)
    {
        for j in (0..(pixels.len() - i - 5)).step_by(3)
        {
            if (*pixels.get(j + 2).unwrap() as u32) < (*pixels.get(j + 5).unwrap() as u32)
            {
                pixels.swap(j, j + 3);
                pixels.swap(j + 1, j + 4);
                pixels.swap(j + 2, j + 5);
            }
        }
    }
    println!("Sorting ended...");
}
