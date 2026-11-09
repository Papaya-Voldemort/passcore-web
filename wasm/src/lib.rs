use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ScoringResult {
    score: u16,
    feedback: String,
    grade: String,
}

#[wasm_bindgen]
impl ScoringResult {
    #[wasm_bindgen(getter)]
    pub fn score(&self) -> u16 {
        self.score
    }

    #[wasm_bindgen(getter)]
    pub fn feedback(&self) -> String {
        self.feedback.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn grade(&self) -> String {
        self.grade.clone()
    }
}

#[wasm_bindgen]
pub fn score_password(password: &str) -> ScoringResult {
    ScoringResult {
        score: passcore::score(password),
        feedback: passcore::review_password(password).to_string(),
        grade: passcore::grade_password(password).to_string(),
    }
}
