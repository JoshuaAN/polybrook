use std::ops::Range;

use logos::Logos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Add,
    Mul,
    Max,
    Div,
    Mod,
    Xor,
    Shl,
    Cmplt,
    Where,
    Cast,
    Stack,
    Index,
    Call,
    Param,
    Assert,
    Const,
}

impl Opcode {
    fn from_bytes(s: &[u8]) -> Option<Self> {
        Some(match s {
            b"add" => Self::Add,
            b"mul" => Self::Mul,
            b"max" => Self::Max,
            b"div" => Self::Div,
            b"mod" => Self::Mod,
            b"xor" => Self::Xor,
            b"shl" => Self::Shl,
            b"cmplt" => Self::Cmplt,
            b"where" => Self::Where,
            b"cast" => Self::Cast,
            b"stack" => Self::Stack,
            b"index" => Self::Index,
            b"call" => Self::Call,
            b"param" => Self::Param,
            b"assert" => Self::Assert,
            b"const" => Self::Const,
            _ => return None,
        })
    }
}

fn name(s: &[u8]) -> &[u8] {
    let end = s[1..]
        .iter()
        .position(|&b| !(b.is_ascii_alphanumeric() || b == b'_'))
        .unwrap()
        + 1;
    &s[1..end]
}

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
#[logos(utf8 = false)]
#[logos(skip r"[ \t\n\r\x0C]+")]
#[logos(skip(r"(?-u:#[^\n]*)", allow_greedy = true))]
pub enum Token<'a> {
    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[regex(r"%[A-Za-z0-9_]+[ \t\n\r\x0C]*=", |lex| name(lex.slice()))]
    Def(&'a [u8]),

    #[regex(r"%[A-Za-z0-9_]+", |lex| &lex.slice()[1..])]
    Ref(&'a [u8]),

    #[regex(r"[a-z]+", |lex| Opcode::from_bytes(lex.slice()))]
    Op(Opcode),

    #[regex(r"-?[0-9]+", |lex| std::str::from_utf8(lex.slice()).unwrap().parse::<i32>().ok())]
    #[regex(r"0x[0-9a-fA-F]+", |lex| {
        u32::from_str_radix(std::str::from_utf8(&lex.slice()[2..]).unwrap(), 16).ok().map(|v| v as i32)
    })]
    Int(i32),
}

pub fn tokenize(src: &[u8]) -> Result<Vec<(Token<'_>, Range<usize>)>, usize> {
    let mut lex = Token::lexer(src);
    let mut tokens = Vec::new();
    while let Some(result) = lex.next() {
        match result {
            Ok(tok) => tokens.push((tok, lex.span())),
            Err(()) => return Err(lex.span().start),
        }
    }
    Ok(tokens)
}
