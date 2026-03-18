//! Quizzer binary - main entry point

use quizzer::{Question, get_question};

use clap::Parser;
use std::process;
use std::io;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct WorkingMod {
    state: String,
}


fn main() {
    let work_mod = match WorkingMod::try_parse() {
        Ok(work_mod) => {
            if work_mod.state == "question-entering" || work_mod.state == "quiz"{
                work_mod
            } else {
                eprintln!("Арсен, напиши норм помилку пліззз, що передано не те що треба");
                process::exit(1);
            }
        },
        Err(e) => {
            if e.to_string().contains("unexpected argument") {
                eprintln!("Арсен, напиши норм помилку пліззз");
            } else {
                eprintln!("{}", e);
            }
            process::exit(1);
        }
    };

    if work_mod.state == "question-entering" {
        loop {
            println!("Enter enter new question ot \"exit\" to exit");
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Помилка читання");

            let question_text = input.trim().to_string();
            input.clear();
            if question_text == "exit"{
                process::exit(0);
            }

            println!("Enter correct відповідь:");
            io::stdin()
                .read_line(&mut input)
                .expect("Помилка читання");

            let mut answers = vec![String::new(); 4];
            answers[0] = input.trim().to_string(); // add correct answer
            input.clear();

            for i in 1..4 {
                println!("Enter incorrect відповідь:");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Помилка читання");

                answers[i] = input.trim().to_string();
                input.clear();
            }

            let question = Question {
                question: question_text,
                options: answers.try_into().expect("Потрібно рівно 4 елементи"),
            };

            question.save();

            println!("Додано новий запис:");
            // println!("{:?}", question);
        }
    } else {
        let question_set =  get_question();
        let mut true_answers = 0;
        for q in &question_set{
            println!("{}\nOptions:", q.question);

            let mut indices: Vec<usize> = (0..q.options.len()).collect();
            indices.shuffle(&mut thread_rng());

            for i in indices {
                println!("{}", q.options[i]);
            }

            let mut user_answer = String::new();

            io::stdin()
                .read_line(&mut user_answer)
                .expect("Помилка читання");

            if user_answer.trim() == q.options[0]{
                true_answers+=1;
            }
        }
        println!("your score: {}/{}", true_answers, question_set.len());
    }
}
