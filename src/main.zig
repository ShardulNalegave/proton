const std = @import("std");
const lexer = @import("lexer/lexer.zig");
const tokens = @import("lexer/tokens.zig");

pub fn main() !void {
    const source_code = "([]) + - ; {} / // == = <= << < hello.world 3985 5.534 39.454.32";
    var l = lexer.Lexer.new(source_code, "test.proton");
    while (true) {
        const tok = try l.nextToken();
        if (tok.kind == tokens.TokenKind.EOF) {
            break;
        }

        std.debug.print("Kind: {s}, Literal: {s}\n", .{ @tagName(tok.kind), tok.literal });
    }
}
