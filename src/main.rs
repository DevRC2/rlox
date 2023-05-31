#[derive(Debug)]
enum TokensType {
    LET,

    INT,
    STR,

    LP,
    RP,
    COMMA,

    MUL,
    DIV,
    ADD,
    SUB,
}
#[derive(Debug)]
struct Token {
    ttype: TokensType,
    content: String,
    pos: (usize, usize),
}
impl Token {
    fn new(ttype: TokensType, content: String, pos: (usize, usize)) -> Self {
        Self {
            ttype,
            content,
            pos,
        }
    }
}

struct Tokenizer {
    source: String,          // Source code.                         , Required: <String>
    tokens: Vec<Vec<Token>>, // Vec<Token> for each line.            , Default : Empty Vec<Token>
    counter: usize,          // Char position.                       , Default : 0
    pos: (usize, usize),     // Source code line                     , Default : (0, 0)
    is_in_string: bool,      // If char in string,  ["anything"]     , Default : false
    is_in_comment: bool,     // If char in comment, [#anything]      , Default : false
    current_char: Option<char>, // Current char                      , Default : None
}

impl Tokenizer {
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            counter: 0,
            pos: (0, 0),
            is_in_string: false,
            is_in_comment: false,
            current_char: None,
        }
    }

    fn scan(&mut self) {
        self.update_char(); // updating current_char beacuse it's `None` by default.
        let mut temp_tokens: Vec<Token> = vec![];

        let mut read_from = 0;
        let mut read_to = 0;
        let mut keywords = "".to_string();

        while self.source.len() > self.counter {
            match self.current_char.unwrap() {
                '#' => {
                    if !self.is_in_string {
                        self.is_in_comment = true;
                        self.pos.1 = 0;
                    }
                }
                '0'..='9' => {}
                '\n' => {
                    if !self.is_in_comment && !self.is_in_string {
                        self.pos.0 += 1; // line
                    }
                    self.is_in_comment = false;
                    self.pos.1 = 0;
                }
                ' ' => {
                    
                    if !self.is_in_comment && !self.is_in_string {
                        self.update_char();
                       println!("{}", self.current_char.unwrap());
                    }
                }
                _ => {
                    
                }
            }

            self.update_char();
        }
        println!("{:?}", temp_tokens);
    }
    fn update_char(&mut self) {
        self.current_char = self.source.chars().nth(self.counter);
        self.counter += 1;
        self.pos.1 += 1;
    }
}

fn main() {
    let src = std::fs::read_to_string("./src/main.rl").expect("cannot find the file.");

    Tokenizer::new(src).scan();
}
