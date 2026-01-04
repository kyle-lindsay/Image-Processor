use std::char;

fn main() {
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;
    let mut padding = 0;
    let mut data: String;

    let mut cache_val: Vec<u8> = vec![];
    let mut cache_hex: Vec<String> = vec![];

    //Create Image
    let mut image: Vec<u8> = vec![0; WIDTH * HEIGHT * 3];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let pixel_start = i * 3 * WIDTH + 3 * j;
            if j % 2 != i % 2 {
                image[pixel_start] = 255;
                image[pixel_start + 1] = 255;
                image[pixel_start + 2] = 255;
            } else {
                image[pixel_start] = 0;
                image[pixel_start + 1] = 0;
                image[pixel_start + 2] = 0;
            }

            //print!("\n\n{}, {}, {}", image[i * WIDTH + j], image[i * WIDTH + j +1], image[i * WIDTH + j+2])
        }
    }

    //Calculate Padding
    for _i in 0..4 {
        if ((WIDTH * 3) + padding) % 4 != 0 {
            padding += 1;
        } 
    }

    //Header
    data = format!("{}", generate_header(WIDTH, HEIGHT, padding));
    //print!("{}", data);

    //Image
    for i in 0..HEIGHT {
        data = format!("{}\n", data);
        for j in 0..WIDTH * 3 {
            data = format!("{}{}", data, cache(&mut cache_val, &mut cache_hex, image[i * 3 * WIDTH + j]));
            // print!("\n{}:", i * 3 * WIDTH + j);
            // print!("{}\n", to_hex(image[i * 3 * WIDTH + j] as u32));
        }

        //Add padding
        for _i in 0..padding {
            data = format!("{}00",data)
        }

    }

   print!("{}", data);
}

fn cache(val: &mut Vec<u8>, hex: &mut Vec<String>, num: u8) -> String {
    let mut found = false;
    let mut i = 0;

    while !found && i < val.len() {
        if num == val[i] {
            found = true;
        } else {
            i += 1;
        }
    }

    if !found {
        val.push(num);
        hex.push(to_hex(num as u32));
        return hex[i].clone();
    } else {
        return hex[i].clone();
    }

}

fn generate_header(width: usize, height: usize, padding: usize) -> String {
    let image_data_size: usize = width * height * 3 + padding;
    let file_size: usize = 54 + (image_data_size);

    return format!("42 4D {} 00 00 00 00 36 00 00 00 28 00 00 00 {} {} 01 00 18 00 00 00 00 00 {} 13 0B 00 00 13 0B 00 00 00 00 00 00 00 00 00 00", to_little_endian(to_hex(file_size as u32)), to_little_endian(to_hex(width as u32)), to_little_endian(to_hex(height as u32)), to_little_endian(to_hex(image_data_size as u32)));
}

fn to_little_endian(hex: String) -> String{
    let len =  hex.len();
    let mut reverse = ['0', '0', '0', '0', '0', '0', '0', '0'];
    let mut result: String = String::from("");

    let vec: Vec<char> = hex.chars().collect();

    for i in 0..len {
         reverse[len-i-1] = vec[i];
    }

    let mut i = 0;
    while i < 8 {
        result = format!("{}{}{}", result, reverse[i+1], reverse[i]);
        i += 2;
    }
    return result;
}

fn to_hex(number: u32) -> String {
    let mut len: usize = 2;
    let mut number:u32 = number;

    while number > 16u32.pow(len as u32) - 1 {
        len += 2;
    }

    let mut hex = vec!['0';len];
    let characters = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

    for i in 0..(len as usize) {
        hex[i] = characters[(number % 16) as usize];
        number /= 16;
    }

    let mut result: String = String::from("");

    for i in 0..len {
        result = format!("{}{}", result, hex[(len-i)-1]);
    }

    return result;
}
