use std::env;

mod lex;
mod parse;

fn main() {
    eprintln!("-----program started-----");
    //input
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        return;
    }
    let arg = &args[1];
    eprintln!("{}", arg);

    //tokenize
    let tokens = lex::lex(&arg).unwrap();
    eprintln!("tokenized");
    eprintln!("{:?}", tokens);

    //parse
    let node = parse::init(tokens);

    eprintln!("{:?}", node);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  mov rax, {}", arg);
    println!("  ret");

    eprintln!("-----program finished-----");
    return;
}
