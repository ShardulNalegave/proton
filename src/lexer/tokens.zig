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

    Let,
    Const,
    Fn,
    If,
    Else,
    While,
    For,
    Enum,
    Struct,

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
    LessThanEqualTo, // <=
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
