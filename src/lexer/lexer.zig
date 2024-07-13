const std = @import("std");
const tokens = @import("tokens.zig");

pub const LexerError = error{
    InvalidToken,
};

pub const Lexer = struct {
    source: []const u8,
    pos: usize,
    line: usize,
    filename: []const u8,

    pub fn new(source: []const u8, filename: []const u8) Lexer {
        return Lexer{
            .source = source,
            .pos = 0,
            .line = 0,
            .filename = filename,
        };
    }

    pub fn peek(self: *Lexer) ?u8 {
        if (self.pos >= self.source.len) return null;
        return self.source[self.pos];
    }

    pub fn advance(self: *Lexer) ?u8 {
        if (self.pos >= self.source.len) return null;
        const c = self.source[self.pos];
        self.pos += 1;
        return c;
    }

    fn makeToken(self: *Lexer, kind: tokens.TokenKind, literal: []const u8) tokens.Token {
        return tokens.Token{
            .kind = kind,
            .literal = literal,
            .line = self.line,
            .filename = self.filename,
        };
    }

    pub fn nextToken(self: *Lexer) !tokens.Token {
        const c: u8 = self.advance() orelse return self.makeToken(tokens.TokenKind.EOF, "");
        switch (c) {
            ' ', '\t' => return self.nextToken(),
            '\n' => {
                self.line += 1;
                return self.nextToken();
            },

            '(' => return self.makeToken(tokens.TokenKind.LeftParen, "("),
            ')' => return self.makeToken(tokens.TokenKind.RightParen, ")"),
            '{' => return self.makeToken(tokens.TokenKind.LeftBrace, "{"),
            '}' => return self.makeToken(tokens.TokenKind.RightBrace, "}"),
            '[' => return self.makeToken(tokens.TokenKind.LeftBracket, "["),
            ']' => return self.makeToken(tokens.TokenKind.RightBracket, "]"),

            ';' => return self.makeToken(tokens.TokenKind.Semicolon, ";"),
            '.' => return self.makeToken(tokens.TokenKind.Dot, "."),
            ',' => return self.makeToken(tokens.TokenKind.Comma, ","),
            '+' => return self.makeToken(tokens.TokenKind.Plus, "+"),
            '-' => return self.makeToken(tokens.TokenKind.Minus, "-"),
            '*' => return self.makeToken(tokens.TokenKind.Asterisk, "*"),
            '!' => return self.makeToken(tokens.TokenKind.Not, "!"),
            '~' => return self.makeToken(tokens.TokenKind.BitwiseNot, "~"),
            '/' => if (self.peek() == '/') {
                _ = self.advance();
                return self.makeToken(tokens.TokenKind.FloorDivide, "//");
            } else {
                return self.makeToken(tokens.TokenKind.FrontSlash, "/");
            },
            '=' => if (self.peek() == '=') {
                _ = self.advance();
                return self.makeToken(tokens.TokenKind.Equals, "==");
            } else {
                return self.makeToken(tokens.TokenKind.Assign, "=");
            },
            '<' => if (self.peek() == '=') {
                _ = self.advance();
                return self.makeToken(tokens.TokenKind.LessThanEqualTo, "<=");
            } else if (self.peek() == '<') {
                _ = self.advance();
                return self.makeToken(tokens.TokenKind.BitwiseLeftShift, "<<");
            } else {
                return self.makeToken(tokens.TokenKind.LessThan, "<");
            },
            '>' => if (self.peek() == '=') {
                _ = self.advance();
                return self.makeToken(tokens.TokenKind.GreaterThanEqualTo, ">=");
            } else if (self.peek() == '>') {
                _ = self.advance();
                return self.makeToken(tokens.TokenKind.BitwiseRightShift, ">>");
            } else {
                return self.makeToken(tokens.TokenKind.GreaterThan, ">");
            },
            '&' => if (self.peek() == '&') {
                _ = self.advance();
                return self.makeToken(tokens.TokenKind.And, "&&");
            } else {
                return self.makeToken(tokens.TokenKind.BitwiseAnd, "&");
            },
            '|' => if (self.peek() == '|') {
                _ = self.advance();
                return self.makeToken(tokens.TokenKind.Or, "||");
            } else {
                return self.makeToken(tokens.TokenKind.BitwiseOr, "|");
            },

            else => |val| {
                if (std.ascii.isAlphabetic(val) or val == '_') {
                    return self.makeToken(tokens.TokenKind.Identifier, self.readIdentifier());
                } else if (std.ascii.isDigit(val)) {
                    return self.makeToken(tokens.TokenKind.Number, self.readNumber());
                } else unreachable;
            },
        }

        unreachable;
    }

    fn readIdentifier(self: *Lexer) []const u8 {
        const pos = self.pos - 1;

        var c = self.peek();
        while (std.ascii.isAlphanumeric(c.?) or c.? == '_') {
            _ = self.advance();

            c = self.peek();
            if (c == null) break;
        }

        return self.source[pos..self.pos];
    }

    fn readNumber(self: *Lexer) []const u8 {
        const pos = self.pos - 1;
        var hasDecimalPt = false;

        var c = self.peek();
        while (std.ascii.isDigit(c.?) or c.? == '.') {
            if (c.? == '.') {
                if (hasDecimalPt) {
                    break;
                } else {
                    hasDecimalPt = true;
                }
            }

            _ = self.advance();

            c = self.peek();
            if (c == null) break;
        }

        return self.source[pos..self.pos];
    }
};
