#[derive(Debug,PartialEq)]
pub enum Token {
    // Basics
    TokNum = 1,
    TokDice = 2,
    // advanced
    TokBrac = 3,
}

pub fn lexer(input: String) -> Vec<Token> {
    let result: Vec<Token> = Vec::new();
    return result;
}

pub fn find_next_token(input: String) -> Token {
    
}

#[cfg(test)]
mod tests {
    use crate::lexer::{lexer, Token};
    #[test]
    fn simple_dice_test() {
        let input = String::from("(1d1)");
        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::TokNum);
        expected.push(Token::TokDice);
        expected.push(Token::TokNum);
        let result = lexer(input);
        assert_eq!(expected,result);
    }
}