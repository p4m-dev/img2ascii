/* 
1. Считать данные из файла

2. Уменшаем картинки

3. Делаем картинку черно-белой

4. Выбираем ASCII символ для каждого пикселя ["@", "#", "S", "%", "?", "*", "+", ";", ":", ",", "."]

5. Выводим то что получилось
*/

use image;
use image::imageops::FilterType;
use image::GenericImageView;
use std::iter::FromIterator;
use std::fs;
use clap::{Arg, App};



fn process_image(path: &str, width: u32, height: u32) -> String {
    let ascii_symbols = "@#S%?*+;:,.";

    let mut img = image::open(path).unwrap();

    img = img.resize(width, height, FilterType::Gaussian);
    img = img.grayscale();
    img.save("mem_out.png").unwrap();

    let mut char_vec: Vec<char> = Vec::new();

    for (x, _, color) in img.pixels() {
        let symbol = ascii_symbols.chars().nth(usize::from(color[0] / 25)).unwrap();
        char_vec.push(symbol);
        char_vec.push(symbol);
        char_vec.push(symbol);

        if x == img.width() - 1 {
            char_vec.push('\n');
        }
    }

    String::from_iter(char_vec)
}


fn main() {
    let matches = App::new("image2ascii")
        .version("1.0.0")
        .author("Anatoliy Platonov <p4m.dev@gmail.com>")
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .value_name("WIDTH")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .value_name("HEIGHT")
            .takes_value(true))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("INPUT")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("OUTPUT")
            .takes_value(true))
        .get_matches();
    
    let width = match matches.value_of("width") {
        Some(s) => s.parse().unwrap(),
        None => 300
    };
    let height = match matches.value_of("height") {
        Some(s) => s.parse().unwrap(),
        None => 300
    };

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let result_text = process_image(input, width, height);
    fs::write(output, result_text).unwrap();
}
