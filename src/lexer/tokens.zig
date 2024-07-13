pub const Token = struct {
    kind: TokenKind,
    literal: []const u8,
    line: usize,
    filename: []const u8,
};

pub const TokenKind = enum {
    EOF,

    LeftParen, // (
    RightParen, // )
    LeftBrace, // {
    RightBrace, // }
    LeftBracket, // [
    RightBracket, // ]

    Identifier,
    Number,
    String,
    Character,

    Let,
    Const,
    Fn,
    If,
    Else,
    While,
    For,
    Enum,
    Struct,
    True,
    False,
    Return,

    Semicolon, // ;
    Dot, // .
    Comma, // ,
    Plus, // +
    Minus, // -
    Asterisk, // *
    FrontSlash, // /
    FloorDivide, // //
    Assign, // =
    Equals, // ==
    LessThan, // <
    LessThanEqualTo, // <=
    GreaterThan, // >
    GreaterThanEqualTo, // >=

    And, // &&
    Or, // ||
    Not, // !
    BitwiseAnd, // &
    BitwiseOr, // |
    BitwiseNot, // ~
    BitwiseLeftShift, // <<
    BitwiseRightShift, // >>
};
