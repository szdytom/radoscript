use {
    radoscript::lex,
    std::io::{stdin, stdout, Write},
};

fn main() {
    let mut lexer = lex::Unit::new();
    loop {
        print!("\n>> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("EOF");
        lexer.add(&input).unwrap();

        loop {
            let t = lexer.next_token();
            if t == None {
                break;
            }

            print!("{:?} ", t.unwrap());
        }
    }
}
