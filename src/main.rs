use std::io::stdout;
use std::io::prelude::*;
use std::fs::File;
use rand::thread_rng;
use rand::Rng;

mod app;
mod typer;

fn get_rand_word(words: Option<&str>) -> String{
    match words{
        Some(c) => c.to_string(),
        None => "None".to_string()
    }
}


fn main() -> crossterm::Result<()> {
    let mut rng = thread_rng();
    let y: usize = rng.gen_range(0..102401);

    let mut file = File::open("/usr/share/dict/american-english").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let words_arr: Vec<&str> = contents.split("\n").collect();
    let words = words_arr.into_iter().nth(y);

    println!("{:?}",get_rand_word(words));

    let text = get_rand_word(words);


    let output = &mut stdout();
    app::run(output, text);
    Ok(())
}
