fn main(source: &str) -> String {
  println!("Hello, world!");
}

enum Token {
  OR,
  AND,
  NEAR,
  NOTNEAR,
  WITH,
  NOTWITH,
  NOT,
  EXCLUDE,
  LEFTPAREN,
  RIGHTPAREN,
  TERM,
  WILDCARD,
  METADATA
}

fn parse(source: &str) -> Vec<Token> {

}

