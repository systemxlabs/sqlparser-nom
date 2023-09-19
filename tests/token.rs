use sqlparser_nom::parser::token::{Token, TokenKind, Tokenizer};

#[test]
pub fn tokenize() {
    let sql = "Select a, t1.b, count(c) FROM t1 where a > 1 order by b limit 10, 20";
    let tokenizer = Tokenizer::new(sql);

    let tokens = tokenizer.collect::<Vec<_>>();

    #[rustfmt::skip]
    let expected = vec![
        Token { source: sql, kind: TokenKind::SELECT, span: 0..6 },
        Token { source: sql, kind: TokenKind::Ident, span: 7..8 },
        Token { source: sql, kind: TokenKind::Comma, span: 8..9 },
        Token { source: sql, kind: TokenKind::Ident, span: 10..12 },
        Token { source: sql, kind: TokenKind::Dot, span: 12..13 },
        Token { source: sql, kind: TokenKind::Ident, span: 13..14 },
        Token { source: sql, kind: TokenKind::Comma, span: 14..15 },
        Token { source: sql, kind: TokenKind::Ident, span: 16..21 },
        Token { source: sql, kind: TokenKind::LParen, span: 21..22 },
        Token { source: sql, kind: TokenKind::Ident, span: 22..23 },
        Token { source: sql, kind: TokenKind::RParen, span: 23..24 },
        Token { source: sql, kind: TokenKind::FROM, span: 25..29 },
        Token { source: sql, kind: TokenKind::Ident, span: 30..32 },
        Token { source: sql, kind: TokenKind::WHERE, span: 33..38 },
        Token { source: sql, kind: TokenKind::Ident, span: 39..40 },
        Token { source: sql, kind: TokenKind::Gt, span: 41..42 },
        Token { source: sql, kind: TokenKind::LiteralInteger, span: 43..44 },
        Token { source: sql, kind: TokenKind::ORDER, span: 45..50 },
        Token { source: sql, kind: TokenKind::BY, span: 51..53 },
        Token { source: sql, kind: TokenKind::Ident, span: 54..55 },
        Token { source: sql, kind: TokenKind::LIMIT, span: 56..61 },
        Token { source: sql, kind: TokenKind::LiteralInteger, span: 62..64 },
        Token { source: sql, kind: TokenKind::Comma, span: 64..65 },
        Token { source: sql, kind: TokenKind::LiteralInteger, span: 66..68 },
    ];

    assert_eq!(tokens, expected);
}
