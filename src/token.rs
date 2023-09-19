use std::ops::Range;

use logos::{Lexer, Logos};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    // Source sql
    pub source: &'a str,
    pub kind: TokenKind,
    // Left closed, right open
    pub span: Range<usize>,
}
impl<'a> Token<'a> {
    pub fn text(&self) -> &'a str {
        &self.source[self.span.clone()]
    }
}

pub struct Tokenizer<'a> {
    source: &'a str,
    lexer: Lexer<'a, TokenKind>,
}
impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Tokenizer {
            source,
            lexer: TokenKind::lexer(source),
        }
    }
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(kind) => match kind {
                Ok(kind) => Some(Token {
                    source: self.source,
                    kind,
                    span: self.lexer.span(),
                }),
                Err(_) => panic!("Unable to recognize the rest tokens"),
            },
            None => None,
        }
    }
}

#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Skip
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r"--[^\n\f]*", logos::skip)]
    Comment,

    #[regex(r"/\*[^\+]([^\*]|(\*[^/]))*\*/", logos::skip)]
    CommentBlock,

    #[regex(r#"[_a-zA-Z][_$a-zA-Z0-9]*"#)]
    Ident,

    #[regex(r#"`[^`]*`"#)]
    #[regex(r#""([^"\\]|\\.|"")*""#)]
    #[regex(r#"'([^'\\]|\\.|'')*'"#)]
    QuotedString,

    #[regex(r"[0-9]+")]
    LiteralInteger,

    #[regex(r"[0-9]+[eE][+-]?[0-9]+")]
    #[regex(r"([0-9]*\.[0-9]+([eE][+-]?[0-9]+)?)|([0-9]+\.[0-9]*([eE][+-]?[0-9]+)?)")]
    LiteralFloat,

    #[token("=")]
    Eq,

    #[token("<>")]
    #[token("!=")]
    NotEq,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token("<=")]
    LtEq,

    #[token(">=")]
    GtEq,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("//")]
    IntDiv,

    #[token("%")]
    Modulo,

    #[token("||")]
    StringConcat,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    // Keywords
    #[token("AND", ignore(ascii_case))]
    AND,

    #[token("AS", ignore(ascii_case))]
    AS,

    #[token("ASC", ignore(ascii_case))]
    ASC,

    #[token("BY", ignore(ascii_case))]
    BY,

    #[token("DESC", ignore(ascii_case))]
    DESC,

    #[token("DISTINCT", ignore(ascii_case))]
    DISTINCT,

    #[token("EXISTS", ignore(ascii_case))]
    EXISTS,

    #[token("FROM", ignore(ascii_case))]
    FROM,

    #[token("FULL", ignore(ascii_case))]
    FULL,

    #[token("GROUP", ignore(ascii_case))]
    GROUP,

    #[token("HAVING", ignore(ascii_case))]
    HAVING,

    #[token("IN", ignore(ascii_case))]
    IN,

    #[token("INNER", ignore(ascii_case))]
    INNER,

    #[token("INTO", ignore(ascii_case))]
    INTO,

    #[token("IS", ignore(ascii_case))]
    IS,

    #[token("JOIN", ignore(ascii_case))]
    JOIN,

    #[token("LEFT", ignore(ascii_case))]
    LEFT,

    #[token("LIKE", ignore(ascii_case))]
    LIKE,

    #[token("LIMIT", ignore(ascii_case))]
    LIMIT,

    #[token("NOT", ignore(ascii_case))]
    NOT,

    #[token("NULL", ignore(ascii_case))]
    NULL,

    #[token("OFFSET", ignore(ascii_case))]
    OFFSET,

    #[token("ON", ignore(ascii_case))]
    ON,

    #[token("OR", ignore(ascii_case))]
    OR,

    #[token("ORDER", ignore(ascii_case))]
    ORDER,

    #[token("OUTER", ignore(ascii_case))]
    OUTER,

    #[token("OVER", ignore(ascii_case))]
    OVER,

    #[token("PARTITION", ignore(ascii_case))]
    PARTITION,

    #[token("SELECT", ignore(ascii_case))]
    SELECT,

    #[token("WHERE", ignore(ascii_case))]
    WHERE,

    #[token("WINDOW", ignore(ascii_case))]
    WINDOW,

    #[token("WITH", ignore(ascii_case))]
    WITH,

    #[token("FIRST", ignore(ascii_case))]
    FIRST,

    #[token("LAST", ignore(ascii_case))]
    LAST,
}
