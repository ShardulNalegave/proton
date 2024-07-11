const tokens = @import("tokens.zig");

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

    pub fn peek(self: *Lexer) !u8 {
        if (self.pos >= self.source.len) return error.EOF;
        return self.source[self.pos];
    }

    pub fn advance(self: *Lexer) !u8 {
        if (self.pos >= self.source.len) return error.EOF;
        const c = self.source[self.pos];
        self.pos += 1;
        return c;
    }

    fn make_token(self: *Lexer, kind: tokens.TokenKind, literal: []const u8) tokens.Token {
        return tokens.Token{
            .kind = kind,
            .literal = literal,
            .line = self.line,
            .filename = self.filename,
        };
    }

    pub fn next_token(self: *Lexer) tokens.Token {
        const c: u8 = self.advance() catch return self.make_token(tokens.TokenKind.EOF, "");
        switch (c) {
            ' ', '\t' => return self.next_token(),
            '\n' => {
                self.line += 1;
                return self.next_token();
            },

            '(' => return self.make_token(tokens.TokenKind.LeftParen, "("),
            ')' => return self.make_token(tokens.TokenKind.RightParen, ")"),
            '{' => return self.make_token(tokens.TokenKind.LeftBrace, "{"),
            '}' => return self.make_token(tokens.TokenKind.RightBrace, "}"),
            '[' => return self.make_token(tokens.TokenKind.LeftBracket, "["),
            ']' => return self.make_token(tokens.TokenKind.RightBracket, "]"),

            ';' => return self.make_token(tokens.TokenKind.Semicolon, ";"),
            '.' => return self.make_token(tokens.TokenKind.Dot, "."),
            ',' => return self.make_token(tokens.TokenKind.Comma, ","),
            '+' => return self.make_token(tokens.TokenKind.Plus, "+"),
            '-' => return self.make_token(tokens.TokenKind.Minus, "-"),
            '*' => return self.make_token(tokens.TokenKind.Asterisk, "*"),
            '/' => if (self.peek() catch {
                return self.make_token(tokens.TokenKind.FrontSlash, "/");
            } == '/') {
                self.pos += 1;
                return self.make_token(tokens.TokenKind.FloorDivide, "//");
            } else {
                return self.make_token(tokens.TokenKind.FrontSlash, "/");
            },
            '=' => if (self.peek() catch {
                return self.make_token(tokens.TokenKind.Assign, "=");
            } == '=') {
                self.pos += 1;
                return self.make_token(tokens.TokenKind.Equals, "==");
            } else {
                return self.make_token(tokens.TokenKind.Assign, "=");
            },

            else => unreachable,
        }

        unreachable;
    }
};
