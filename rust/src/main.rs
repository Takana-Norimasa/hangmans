use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use rand::Rng;

struct Words{
	len:usize,
	string:String,
	is_wrong:bool,
	is_correct:bool,
	is_answered:bool,
}

fn readfile(words:&mut Vec<Words>){
    let filepath="../data/toeic1500.dat";
    println!("In file {}", filepath);

    let file=File::open(filepath)
        .expect("file not found");
    let reader=BufReader::new(file);

    for line in reader.lines(){
        let line=line.unwrap();
        words.push(
                Words{
                    len:line.len(),
                    string:line,
                    is_wrong:false,
                    is_correct:false,
                    is_answered:false,
                }
            );
    }
}

fn display_result(words:&mut Vec<Words>){
    let mut wrong:f32=0.0;
    let mut correct:f32=0.0;

    for w in words {
        if w.is_wrong {
            println!("{}",w.string);
            wrong+=1.0;
        }

        if w.is_correct {
            correct+=1.0;
        }
    }
	println!("-----------------------");
	println!("wrong:{}",wrong);
	println!("correct:{}",correct);
	println!("correct answer rate:{}",correct/(wrong+correct)*100.0);
}

fn display_data(word:&mut Words,used:&mut [bool],input:char,remain:&mut u32){
    let mut hit:bool=false;
    let mut ctr:usize=0;

    //println!("\x1bc");

    // hint
    print!("hint:");
    for c in word.string.chars() {
        if used[(c as usize)-('a' as usize)]{
            if c==input {hit=true;}
            print!("{}",c);
            ctr+=1;
        }else{
            print!("-");
        }
    }
    io::stdout().flush()
        .expect("flush error.");
    println!("");
    if ctr==word.len {
        word.is_correct=true;
    }

    // display used
    println!("====================================");
    print!("used:");
    io::stdout().flush()
        .expect("flush error");
    for c in 'a'..'z' {
        if used[(c as usize)-('a' as usize)] {
            print!("\x1b[41m{}\x1b[49m",c);
        }else{
            print!("{}",c);
        }
    }
    println!("");
    println!("====================================");

    // display remain
    if hit==false && input!='0' {
        *remain-=1;
    }
    print!("remain[{}]:",*remain);
    print!("\x1b[42m");
    for i in 0..*remain {
        print!("  ");
    }
    println!("\x1b[49m");
}

fn playgame(words:&mut Vec<Words>){
    let hit:bool=false;
    let mut remain:u32;
    let input:String=String::new();
    let mut game_continue:bool=true;
    let mut used:[bool;27];
    const TRY:u32=7;

    while game_continue {
        let range_max=words.len();
        let mut word:&mut Words=&mut words[rand::thread_rng().gen_range(0,range_max)];
        if word.is_correct {continue;}

        // initialize data
        let mut used:[bool;27]=[false;27];

        remain=TRY;
        display_data(&mut word,&mut used,'0',&mut remain);
        while remain>0 {
            println!("ans:{}",word.string);
            print!("input char>>");
            io::stdout().flush()
                .expect("flush error");

            let mut input=String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line.");
            let input=input.chars().next().unwrap();
            let input_num=input as u32;

            if !(('a' as u32)<=input_num && input_num<=('z' as u32)) || used[(input_num as usize)-('a' as usize)] {
                continue;
            }

            used[(input_num as usize)-('a' as usize)]=true;
            display_data(&mut word,&mut used,input as char,&mut remain);
            println!("inputed:{}",input as u32);

            if word.is_correct {break;}
        }
        
        if word.is_correct {
            println!("correct!");
        }else{
            println!("wrong!");
            word.is_wrong=true;
        }
        word.is_answered=true;

		print!("continue?[y/n]>");
        io::stdout().flush()
                .expect("flush error");

        let mut input=String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");
        let mut input=input.chars().next().unwrap() as char;

        if input=='n' {
            game_continue=false;
        }
    }
}

fn main(){
    let mut words:Vec<Words>=Vec::new();

    readfile(&mut words);
    playgame(&mut words);
    display_result(&mut words);
}
