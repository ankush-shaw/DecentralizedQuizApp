#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, Map, String};

#[contract]
pub struct QuizContract;

#[contractimpl]
impl QuizContract {

    // Add a question with correct answer
    pub fn add_question(env: Env, id: u32, question: String, answer: String) {
        let mut questions: Map<u32, (String, String)> =
            env.storage().instance().get(&symbol_short!("QUEST")).unwrap_or(Map::new(&env));

        questions.set(id, (question, answer));
        env.storage().instance().set(&symbol_short!("QUEST"), &questions);
    }

    // Get question
    pub fn get_question(env: Env, id: u32) -> String {
        let questions: Map<u32, (String, String)> =
            env.storage().instance().get(&symbol_short!("QUEST")).unwrap();

        let (q, _) = questions.get(id).unwrap();
        q
    }

    // Submit answer
    pub fn submit_answer(env: Env, user: String, id: u32, user_answer: String) -> bool {
        let questions: Map<u32, (String, String)> =
            env.storage().instance().get(&symbol_short!("QUEST")).unwrap();

        let (_, correct_answer) = questions.get(id).unwrap();

        let mut scores: Map<String, u32> =
            env.storage().instance().get(&symbol_short!("SCORE")).unwrap_or(Map::new(&env));

        if user_answer == correct_answer {
            let current_score = scores.get(user.clone()).unwrap_or(0);
            scores.set(user, current_score + 1);
            env.storage().instance().set(&symbol_short!("SCORE"), &scores);
            true
        } else {
            false
        }
    }

    // Get user score
    pub fn get_score(env: Env, user: String) -> u32 {
        let scores: Map<String, u32> =
            env.storage().instance().get(&symbol_short!("SCORE")).unwrap_or(Map::new(&env));

        scores.get(user).unwrap_or(0)
    }
}