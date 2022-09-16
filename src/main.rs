use std::io::stdout;
use std::io::prelude::*;
use std::fs::File;
use rand::thread_rng;
use rand::Rng;

mod app;
mod typer;

fn get_rand_word(word_opt: Option<&str>) -> String{
    match word_opt{
        Some(c) => c.to_string(),
        None => "None".to_string()
    }
}


fn main() -> crossterm::Result<()> {
    
    let mut words = String::from("");

    for i in 0..10 {
        let mut rng = thread_rng();
        let mut y: usize = rng.gen_range(0..102401);

        let mut file = File::open("/usr/share/dict/american-english").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        let words_arr: Vec<&str> = contents.split("\n").collect();
        let word_opt = words_arr.into_iter().nth(y);

        println!("{:?}",get_rand_word(word_opt));

        let text = get_rand_word(word_opt);
        let t:&str = &text;

        words.push_str(" ");
        words.push_str(t);
        println!("{words}")
    }


    words.remove(0);
    let output = &mut stdout();
    app::run(output, words);
    Ok(())
}
