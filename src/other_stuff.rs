enum Kernel {
    BoxBlur(i32),
}

fn blur(src_img: &DynamicImage, kernel: Kernel) -> DynamicImage {
    let mut coord_vector = Vec::new();
    let mut weight_vector = Vec::new();

    match kernel {
        Kernel::BoxBlur(rad) => {
            let mut x = -rad;
            let mut y = -rad;

            while x < rad {
                while y < rad {
                    coord_vector.push([x, y]);
                    weight_vector.push(1);
                    y = y + 1;
                }
                y = -rad;
                x = x + 1;
            }
        }
    }

    let img_width = src_img.width();
    let img_height = src_img.height();

    let mut dst_img = copy_img(&src_img);

    for pixel in src_img.pixels() {
        let mut adj_colors: Vec<Rgba<u8>> = Vec::new();
        let mut i = 0;
        for point in &coord_vector {
            let x = wrap_u32(pixel.0, point[0], img_width);
            let y = wrap_u32(pixel.1, point[1], img_height);
            let color = src_img.get_pixel(x, y);
            let mut channel = 0;
            let mut weighted_color = [0, 0, 0, 0];
            while channel <= 3 {
                weighted_color[channel] = color[channel] * weight_vector[i];
                channel = channel + 1;
                if channel == 3 {
                    //println!("{}", color[channel]);
                }
            }
            adj_colors.push(color);
            i = i + 1;
        }
        dst_img.put_pixel(pixel.0, pixel.1, mean(adj_colors));
    }

    dst_img
}

fn sharpen_pixel(origin: Rgba<u8>, colors: Vec<Rgba<u8>>) -> Rgba<u8> {
    let mut difference = Vec::new();

    for c in &colors {
        let mut i = 0;
        while i <= 3 {
            let mut color = [0, 0, 0, 0];
            color[i] = (origin[i] as i16 - c[i] as i16) as u8;
            difference.push(Rgba(color));
            i = i + 1;
        }
    }

    return mean(difference);
}

fn wrap_u32(num: u32, add: i32, max: u32) -> u32 {
    if (num as i64 + add as i64) < 0 {
        (-(num as i64 + add as i64) % (max as i64)) as u32
    } else {
        ((num as i64 + add as i64) % (max as i64)) as u32
    }
}fn wrap_u32(num: u32, add: i32, max: u32) -> u32 {
    if (num as i64 + add as i64) < 0 {
        (-(num as i64 + add as i64) % (max as i64)) as u32
    } else {
        ((num as i64 + add as i64) % (max as i64)) as u32
    }
}

fn mean(colors: Vec<Rgba<u8>>) -> Rgba<u8> {
    let mut color: [u32; 4] = [0, 0, 0, 0];

    for c in &colors {
        let mut i = 0;
        while i <= 3 {
            //println!("{i}");
            color[i] = color[i] + c[i] as u32;
            i = i + 1;
        }
    }

    let mut i = 0;
    let mut color_u8: [u8; 4] = [0, 0, 0, 0];

    while i <= 3 {
        color_u8[i] = (color[i] / colors.len() as u32) as u8;
        i = i + 1;
    }

    Rgba(color_u8)
}
