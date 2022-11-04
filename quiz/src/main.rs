use std::{
    fs::File, 
    io::{self, Read},
    path::Path,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Quiz {
    questions: Vec<Question>
}

#[derive(Serialize, Deserialize, Debug)]
struct Question {
    prompt: String,
    answers: Vec<String>
}

fn main() -> Result<()> {
    println!("Quiz!");

    let path = Path::new("Quiz.toml");

    println!("Reading questions from: {:?}", path.as_os_str());
    
    let mut contents = String::new();
    File::open(path)?
        .read_to_string(&mut contents)?;

    let quiz: Quiz = toml::from_str(&contents)?;
    let mut score: u8 = 0;

    for question in &quiz.questions {
        println!("{}", question.prompt);

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)?;

        let answer: String = answer
            .trim()
            .to_lowercase()
            .into();

        let answers: Vec<String> = question.answers.iter()
            .map(|x| x.to_lowercase())
            .collect();

        if answers.contains(&answer) {
            score += 1;
            println!("Correct! Score: {score}\n");
        } else {
            println!("Incorrect. Acceptable answers are {:?}\n", question.answers);
        }
    }

    let count_questions = quiz.questions.len();

    println!("Quiz complete. You got {score}/{count_questions}. {}",
             if score as usize >= count_questions / 2 
                { "Well done!" } 
             else 
                { "Better luck next time. " }
    );

    Ok(())
}
