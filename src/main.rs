// MIT License
//
// Copyright (c) 2020 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

mod font;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        println!("RustoFont : Rust implementation of FFont\n\
            Usage: {command} [font] text\
            \n---\n\
            {command} [font] text\n\
            {command} [font] text text1 text2 : Multiple texts\
            \n---\n\
            [font]:\
            \n---\n\
            [--d --default] : FFont's default font",
            command = args.get(0).unwrap());


        std::process::exit(1);
    }

    let option = args.get(1).unwrap();
    let mut text_data  = String::new();

    for text in args.iter().skip(2) {
        text_data.push_str(&text);
        text_data.push(' ');
    }

    match option.as_str() {
        "--d" | "--default" => {
            print!("{}", font::FFont::generate(&font::FFont::init_default(), text_data));
        },
        _ => {

        }
    }
}
