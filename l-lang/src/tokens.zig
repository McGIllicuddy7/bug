const std = @import("std");
const utils = @import("utils.zig");
const TokenNames: [][]u8 = [_][]u8{ "TokenNone", "TokenStr", "TokenChar", "TokenInt", "TokenFloat", "TokenOpenCurl", "TokenCloseCurl", "TokenOpenParen", "TokenCloseParen", "TokenSemi", "TokenColon", "TokenOperator", "TokenIdent", "TokenComma", "TokenDot", "TokenOpenBracket", "TokenCloseBracket" };
pub const Token = struct {
    str: []const u8,
    file: []const u8,
    line: usize,
    tt: TokenType,
    pub fn print(v: *const @This()) void {
        std.debug.print("{{tt:{s}, text:{s}, file:{s}, line:{x}}}\n", .{ @tagName(v.tt), v.str, v.file, v.line });
    }
    pub fn equals(v: *const @This(), str: []const u8) bool {
        return str_equals(v.str, str);
    }
};
pub const String = std.ArrayList(u8);
pub const TTState = enum {
    TTEmpty,
    TTChar,
    TTNum,
    TTString,
    TTIdent,
};
pub const TokenType = enum {
    TokenNone,
    TokenStr,
    TokenChar,
    TokenInt,
    TokenFloat,
    TokenOpenCurl,
    TokenCloseCurl,
    TokenOpenParen,
    TokenCloseParen,
    TokenSemi,
    TokenColon,
    TokenOperator,
    TokenIdent,
    TokenComma,
    TokenDot,
    TokenOpenBracket,
    TokenCloseBracket,
    TokenTypeCount,
};
pub fn is_whitespace(c: u8) bool {
    if (c == ' ') {
        return true;
    } else if (c == '\n') {
        return true;
    } else if (c == '\t') {
        return true;
    } else if (c == '\r') {
        return true;
    } else {
        return false;
    }
}
pub fn is_number(c: u8) bool {
    if (c == '0') {
        return true;
    } else if (c == '1') {
        return true;
    } else if (c == '2') {
        return true;
    } else if (c == '3') {
        return true;
    } else if (c == '4') {
        return true;
    } else if (c == '5') {
        return true;
    } else if (c == '6') {
        return true;
    } else if (c == '7') {
        return true;
    } else if (c == '8') {
        return true;
    } else if (c == '9') {
        return true;
    } else if (c == '.') {
        return true;
    } else {
        return false;
    }
}
pub fn is_entire_token(c: u8, n: u8) bool {
    if (n == '=') {
        if (c == '+') {
            return true;
        } else if (c == '-') {
            return true;
        } else if (c == '*') {
            return true;
        } else if (c == '/') {
            return true;
        } else if (c == '=') {
            return true;
        } else if (c == '>') {
            return true;
        } else if (c == '<') {
            return true;
        }
    } else if (n == '>') {
        if (c == '-') {
            return true;
        }
    }
    return false;
}
pub fn is_reserved(c: u8) bool {
    if (c == '(') {
        return true;
    } else if (c == ')') {
        return true;
    } else if (c == '+') {
        return true;
    } else if (c == '-') {
        return true;
    } else if (c == '*') {
        return true;
    } else if (c == '/') {
        return true;
    } else if (c == '{') {
        return true;
    } else if (c == '}') {
        return true;
    } else if (c == ';') {
        return true;
    } else if (c == '.') {
        return true;
    } else if (c == '[') {
        return true;
    } else if (c == ']') {
        return true;
    } else if (c == '<') {
        return true;
    } else if (c == '=') {
        return true;
    } else if (c == '>') {
        return true;
    } else if (c == ':') {
        return true;
    } else {
        return false;
    }
}
pub fn calc_tt(c: u8) TokenType {
    if (c == '(') {
        return TokenType.TokenOpenParen;
    } else if (c == ')') {
        return TokenType.TokenCloseParen;
    } else if (c == '{') {
        return TokenType.TokenOpenCurl;
    } else if (c == '}') {
        return TokenType.TokenCloseCurl;
    } else if (c == '[') {
        return TokenType.TokenOpenBracket;
    } else if (c == ']') {
        return TokenType.TokenCloseBracket;
    } else if (c == ';') {
        return TokenType.TokenSemi;
    } else if (c == ':') {
        return TokenType.TokenColon;
    } else if (c == '.') {
        return TokenType.TokenDot;
    } else if (c == ',') {
        return TokenType.TokenComma;
    } else {
        return TokenType.TokenOperator;
    }
}
pub const TokenStream = struct {
    al: std.mem.Allocator,
    idx: usize,
    line: usize,
    file: []const u8,
    chars: []const u8,
    pub fn new(al: std.mem.Allocator, chars: []const u8, file: []const u8) TokenStream {
        var out: TokenStream = undefined;
        out.al = al;
        out.chars = chars;
        out.idx = 0;
        out.file = file;
        out.line = 1;
        return out;
    }
    pub fn next(strm: *@This()) !?Token {
        var s = try String.initCapacity(strm.al, 100);
        var state = TTState.TTEmpty;
        var out: Token = undefined;
        var was_slash = false;
        if (strm.idx > strm.chars.len) {
            return null;
        }
        var done = false;
        while (strm.idx < strm.chars.len) {
            if (done) {
                break;
            }
            const c = strm.chars[strm.idx];
            strm.idx += 1;
            if (c == '\n') {
                strm.line += 1;
            }
            if (state == TTState.TTEmpty) {
                if (is_whitespace(c)) {
                    continue;
                } else {
                    if (c == '"') {
                        state = TTState.TTString;
                        try s.append(strm.al, c);
                        continue;
                    } else if (c == '\'') {
                        state = TTState.TTChar;
                        try s.append(strm.al, c);
                        continue;
                    } else if (is_number(c)) {
                        state = TTState.TTNum;
                        try s.append(strm.al, c);
                        continue;
                    } else if (c == ',') {
                        try s.append(strm.al, c);
                        out.tt = TokenType.TokenComma;
                        done = true;
                        break;
                    } else if (is_reserved(c)) {
                        const has_next = strm.idx < strm.chars.len;
                        try s.append(strm.al, c);
                        if (has_next) {
                            const n = strm.chars[strm.idx];
                            if (is_entire_token(c, n)) {
                                strm.idx += 1;
                                try s.append(strm.al, c);
                            }
                        }
                        out.tt = calc_tt(c);
                        done = true;
                        continue;
                    } else {
                        state = TTState.TTIdent;
                        try s.append(strm.al, c);
                        continue;
                    }
                }
            } else {
                if (state == TTState.TTChar) {
                    if (was_slash) {
                        if (c == 't') {
                            try s.append(strm.al, '\t');
                        } else if (c == '"') {
                            try s.append(strm.al, '"');
                        } else if (c == 'n') {
                            try s.append(strm.al, '\"');
                        } else if (c == '0') {
                            try s.append(strm.al, 0);
                        }
                        was_slash = false;
                    } else {
                        if (c == '\\') {
                            was_slash = true;
                        } else {
                            try s.append(strm.al, c);
                            if (c == '\'') {
                                break;
                            }
                            was_slash = false;
                        }
                    }
                } else if (state == TTState.TTString) {
                    if (was_slash) {
                        if (c == 't') {
                            try s.append(strm.al, '\t');
                        } else if (c == '"') {
                            try s.append(strm.al, '"');
                        } else if (c == 'n') {
                            try s.append(strm.al, '\"');
                        } else if (c == '0') {
                            try s.append(strm.al, 0);
                        }
                        was_slash = false;
                    } else {
                        if (c == '\\') {
                            was_slash = true;
                        } else {
                            try s.append(strm.al, c);
                            if (c == '"') {
                                break;
                            }
                            was_slash = false;
                        }
                    }
                } else if (state == TTState.TTIdent) {
                    if (is_whitespace(c) or is_reserved(c)) {
                        strm.idx -= 1;
                        if (c == '\n') {
                            strm.line -= 1;
                        }
                        break;
                    } else {
                        try s.append(strm.al, c);
                    }
                } else if (state == TTState.TTNum) {
                    if (is_number(c)) {
                        try s.append(strm.al, c);
                    } else {
                        strm.idx -= 1;
                        break;
                    }
                }
            }
        }
        if (strm.idx == strm.chars.len) {
            strm.idx = strm.chars.len + 1;
        }
        out.file = strm.file;
        out.line = strm.line;
        out.str = s.items;
        if (state == TTState.TTNum) {
            if (utils.contains(u8, out.str, '.')) {
                out.tt = TokenType.TokenFloat;
            } else {
                out.tt = TokenType.TokenInt;
            }
        } else if (state == TTState.TTChar) {
            out.tt = TokenType.TokenChar;
        } else if (state == TTState.TTString) {
            out.tt = TokenType.TokenStr;
        } else if (state == TTState.TTIdent) {
            out.tt = TokenType.TokenIdent;
        }
        return out;
    }
    pub fn peek(strm: *@This()) !?Token {
        const prev = strm.*;
        const out = try strm.next();
        strm.* = prev;
        return out;
    }
};
pub fn str_equals(a: []const u8, b: []const u8) bool {
    if (a.len != b.len) {
        return false;
    }
    for (0..a.len) |i| {
        if (a[i] != b[i]) {
            return false;
        }
    }
    return true;
}
