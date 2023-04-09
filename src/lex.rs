#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Variable(String),
    Figure(String),
    Arithmetic(String),
    Comparison(String),
    Type(String),
    VOID,
    WHILE,
    FOR,
    IF,
    ELSE,
    BREAK,
    CONTINUE,
    EQ,
    LP,
    RP,
    LCB,
    RCB,
    LSB,
    RSB,
    COMMA,
    SEMICOL,
    RETURN,
    None,
}

//先頭のスペース・tab・改行を消した文字列スライスを返す
fn ignore_space(text: &str) -> Result<&str, &str> {
    let mut i = 0;
    for c in text.chars() {
        if c == '\r' || c == '\n' || c == '\t' || c == ' ' {
            i += 1;
        } else {
            break;
        }
    }
    return Ok(&text[i..]);
}
//先頭のコメントを消した文字列スライスを返す
fn ignore_comment(text: &str) -> Result<&str, &str> {
    if text.len() < 2 {
        return Err("");
    }
    let mut i = 0;
    if &text[0..1] == "//" {
        for c in text.chars() {
            if c == '\n' {
                i += 1;
                break;
            } else {
                i += 1;
            }
        }
    } else {
        return Err("");
    }
    return Ok(&text[i..]);
}
//文字列トークンを返す
fn get_word(text: &str) -> Result<(&str, Token), &str> {
    let token: Token;
    if text.len() < 1 {
        return Err("");
    }
    if !is_alpha_underbar(&text.chars().nth(0).unwrap()) {
        return Err("");
    }
    let mut i = 0;
    let mut chars: Vec<char> = Vec::new();
    for c in text.chars() {
        if is_alpha_underbar(&c) || is_number(&c) {
            i += 1;
            chars.push(c);
        } else {
            break;
        }
    }
    let word: String = chars.iter().collect();
    match &word[..] {
        "int" => token = Token::Type(String::from("int")),
        "char" => token = Token::Type(String::from("char")),
        "void" => token = Token::VOID,
        "while" => token = Token::WHILE,
        "for" => token = Token::FOR,
        "if" => token = Token::IF,
        "else" => token = Token::ELSE,
        "break" => token = Token::BREAK,
        "continue" => token = Token::CONTINUE,
        "return" => token = Token::RETURN,
        _ => token = Token::Variable(word.to_string()),
    }
    return Ok((&text[i..], token));
}
//数値トークンを返す
fn get_figure(text: &str) -> Result<(&str, Token), &str> {
    if text.len() == 0 {
        return Err("");
    }
    let mut i = 0;
    for ch in text.chars() {
        if is_number(&ch) {
            i += 1;
        } else {
            break;
        }
    }
    if i == 0 {
        return Err("");
    }
    Ok((&text[i..], Token::Figure(String::from(&text[0..i]))))
}
//比較演算子トークンを返す
fn get_comparison(text: &str) -> Result<(&str, Token), &str> {
    if text.len() < 2 {
        return Err("");
    }
    let two_op = &text[0..2];
    match two_op {
        "==" | "!=" | ">=" | "<=" => {
            let token = Token::Comparison(two_op.to_string());
            return Ok((&text[2..], token));
        }
        _ => {}
    }
    let one_op = &text[0..1];
    match one_op {
        ">" | "<" => {
            let token = Token::Comparison(one_op.to_string());
            return Ok((&text[1..], token));
        }
        _ => {
            return Err("");
        }
    }
}
//算術演算子トークンを返す
fn get_arithmetic(text: &str) -> Result<(&str, Token), &str> {
    if text.len() < 1 {
        return Err("");
    }
    let op = &text[0..1];
    match op {
        "+" | "-" | "*" | "/" => {
            let token = Token::Arithmetic(op.to_string());
            return Ok((&text[1..], token));
        }
        _ => {
            return Err("");
        }
    }
}
//その他のトークンを返す
fn get_other(text: &str) -> Result<(&str, Token), &str> {
    let token: Token;
    if text.len() < 1 {
        return Err("");
    }
    let op = &text[0..1];
    match op {
        "=" => token = Token::EQ,
        "(" => token = Token::RP,
        ")" => token = Token::LP,
        "{" => token = Token::LCB,
        "}" => token = Token::RCB,
        "[" => token = Token::LSB,
        "]" => token = Token::RSB,
        ";" => token = Token::SEMICOL,
        "," => token = Token::COMMA,
        _ => token = Token::None,
    }
    return Ok((&text[1..], token));
}

//アルファベット or アンダーバーを判定
fn is_alpha_underbar(ch: &char) -> bool {
    if *ch >= 'a' && *ch <= 'z' {
        return true;
    } else if *ch >= 'A' && *ch <= 'Z' {
        return true;
    } else if *ch == '_' {
        return true;
    } else {
        return false;
    }
}
//数値か判定
fn is_number(ch: &char) -> bool {
    if *ch >= '0' && *ch <= '9' {
        return true;
    } else {
        return false;
    }
}

pub fn lex(mut text: &str) -> Result<Vec<Token>, &str> {
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        if let Ok(s) = ignore_space(text) {
            text = s;
        }
        if let Ok(s) = ignore_comment(text) {
            text = s;
            continue;
        }
        let tmp_token: Token;
        if let Ok((s, token)) = get_word(text) {
            text = s;
            tmp_token = token;
        } else if let Ok((s, token)) = get_figure(text) {
            text = s;
            tmp_token = token;
        } else if let Ok((s, token)) = get_comparison(text) {
            text = s;
            tmp_token = token;
        } else if let Ok((s, token)) = get_arithmetic(text) {
            text = s;
            tmp_token = token;
        } else if let Ok((s, token)) = get_other(text) {
            text = s;
            tmp_token = token;
        } else if text == "" {
            break;
        } else {
            return Err("");
        }
        tokens.push(tmp_token);
    }
    return Ok(tokens);
}
