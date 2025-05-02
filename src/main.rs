mod logic;
mod data_structures;


fn main() {
    let _error: Result<(), Box<dyn std::error::Error>> = logic::lexer::lexer_start();
}
