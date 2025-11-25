#[derive(PartialEq, Debug)]
struct TokenNum {
    number: String
}



#[derive(Debug,PartialEq)]
pub enum Token {
    // Basics
    TokNum(TokenNum),
    TokDice,
    // advanced
    TokBrac,
    // When there isn't any tokens
    TokEmpty,
}

pub fn lexer(input: String) -> Vec<Token> {
    let result: Vec<Token> = Vec::new();
    return result;
}

pub fn find_next_token(input: String, mut n:usize) -> (Token, usize) {
    let mut token_temp: String = String::new();
    // 
    while input.chars().nth(n).expect("Couldn't unwrap due to no character existing").is_digit(10) {
        token_temp.push(input.chars().nth(n).unwrap());
        n +=1;
        if input.len() <= n || !input.chars().nth(n).expect("Couldn't unwrap due to no character existing").is_digit(10) {
            let result = TokenNum{ number: token_temp};
            return (Token::TokNum(result), n);
        }
    }

    return (Token::TokEmpty, n);
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