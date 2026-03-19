//! Quizzer library
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub options: [String; 4],
}

impl Question {
    pub fn append_to_json(self) {
        let mut questions: Vec<Question> = match fs::read_to_string("quiz.json") {
            Ok(content) => serde_json::from_str(&content).unwrap_or(Vec::new()),
            Err(_) => Vec::new(),
        };
        questions.push(self);
        let json = serde_json::to_string(&questions).unwrap();
        fs::write("quiz.json", json).unwrap();
    }
}
pub fn import_from_json() -> Vec<Question> {
    match fs::read_to_string("quiz.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or(Vec::new()),
        Err(_) => Vec::new(),
    }
}

pub fn get_question() -> Vec<Question> {
    let mut questions = Vec::new();
    questions.push(Question {
        question: "2 + 2 = ?".to_string(),
        options: [
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
        ],
    });

    questions.push(Question {
        question: "Столиця Франції?".to_string(),
        options: [
            "Лондон".to_string(),
            "Париж".to_string(),
            "Берлін".to_string(),
            "Мадрид".to_string(),
        ],
    });
    questions.push(Question {
        question: "Otakza od zaka fresko?".to_string(),
        options: ["Tak".to_string(), "niiii".to_string(), "may be".to_string(), "xto ja".to_string()]
    });
    questions
}
