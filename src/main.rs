use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn read_word(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut words: Vec<String> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        if let Ok(word) = line {
            if !words.contains(&word) {
                words.push(word);
            }
        }
    }
    Ok(words)
}

fn hangman_graphic(guesses: i32) {
    match guesses {
        0 => {
            println!("________      ");
            println!("|      |      ");
            println!("|             ");
            println!("|             ");
            println!("|             ");
            println!("|             ");
        },
        1 => {
            println!("________      ");
            println!("|      |      ");
            println!("|      0      ");
            println!("|             ");
            println!("|             ");
            println!("|             ");
        },
        2 => {
            println!("________      ");
            println!("|      |      ");
            println!("|      0      ");
            println!("|     /       ");
            println!("|             ");
            println!("|             ");
        },
        3 => {
            println!("________      ");
            println!("|      |      ");
            println!("|      0      ");
            println!("|     /|      ");
            println!("|             ");
            println!("|             ");
        },
        4 => {
            println!("________      ");
            println!("|      |      ");
            println!("|      0      ");
            println!("|     /|\\     ");
            println!("|             ");
            println!("|             ");
        },
        5 => {
            println!("________      ");
            println!("|      |      ");
            println!("|      0      ");
            println!("|     /|\\     ");
            println!("|     /       ");
            println!("|             ");
        },
        _ => {
            println!("________      ");
            println!("|      |      ");
            println!("|      0      ");
            println!("|     /|\\     ");
            println!("|     / \\     ");
            println!("|             ");
        }
    }
}

fn main() {
    match read_word("src\\wordlist.txt") {
        Ok(words) => {
            let mut next = true;
            println!("开始我们的Hangman游戏!\n");
            while next {
                let choice = words.choose(&mut rand::thread_rng());
                let word = choice.unwrap().to_string();
                let mut underscore_line = String::new();
                for c in word.chars() {
                    if c == ' ' {
                        underscore_line.push(' ');
                    } else {
                        underscore_line.push('_');
                        underscore_line.push(' ');
                    }
                }
                let mut remain_choice = 7;
                println!("您需要猜测单词: {}", underscore_line);
                while remain_choice > 0 {
                    // println!("剩余次数: {}", remain_choice);
                    let mut guess = String::new();
                    print!("请输入一个字母或单词: ");
                    io::stdout().flush().expect("Failed to flush stdout");
                    io::stdin()
                        .read_line(&mut guess)
                        .expect("Failed to read line");
                    if guess.trim().len() != 1 {
                        if guess.trim() == word {
                            println!("恭喜你，你猜对了!");
                            break;
                        } else {
                            println!("很遗憾，你猜错了!");
                        }
                    } else {
                        let char = guess.chars().next().unwrap();
                        if word.contains(char) {
                            let guessed_char = guess.chars().next().unwrap(); // 获取输入的字符
                            if word.contains(guessed_char) {
                                println!("你猜对了一个字母!");
                                remain_choice += 1;
                                for (index, c) in word.chars().enumerate() {
                                    if c == guessed_char {
                                        underscore_line.replace_range(
                                            index * 2..=index * 2,
                                            &guessed_char.to_string(),
                                        );
                                    }
                                }
                                println!("当前猜测状态: {}", underscore_line);
                            } else {
                                println!("很遗憾，你猜错了这个字母!");
                            }
                        } else {
                            // println!("很遗憾，你猜错了!");
                            hangman_graphic(7 -remain_choice);
                        }
                    }
                    if !underscore_line.contains('_') {
                        println!("恭喜你，你猜对了!");
                        break;
                    }
                    remain_choice -= 1;
                }
                if remain_choice == 0 {
                    println!("很遗憾，你被吊死了!");
                    println!("正确答案是: {}", word);
                }
                println!("想再玩一次吗？(y/n):");
                let mut play_again = String::new();
                io::stdin()
                    .read_line(&mut play_again)
                    .expect("Failed to read line");
                if play_again.trim() == "y" {
                    next = true;
                } else {
                    next = false;
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
    println!("希望你玩的愉快!")
}
