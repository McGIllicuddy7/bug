pub const tokens_f = @import("tokens.zig");
const std = @import("std");
pub const Token = tokens_f.Token;
pub const TokenType = tokens_f.TokenType;
pub const Alloc = std.mem.Allocator;
const Arena = std.heap.ArenaAllocator;
const ArrayList = std.ArrayList;
const parser = @import("parser.zig");
const Expr = parser.Expr;
const utils = @import("utils.zig");
const Stream = utils.stream;
pub const Function = struct {
    args: std.ArrayList(Field),
    instructions: std.ArrayList(Statement),
    name: []const u8,
    rv: *Type,
    pub fn print(this: *const @This()) void {
        std.debug.print("{s}\nrv:", .{this.name});
        this.rv.print();
        std.debug.print("args:\n", .{});
        for (this.args.items) |i| {
            std.debug.print("{s}:", .{i.name});
            i.vtype.print();
        }
        std.debug.print("statements:\n", .{});
        for (this.instructions.items) |i| {
            i.print();
        }
        std.debug.print("end\n", .{});
    }
};
pub const Scope = struct { vars: std.ArrayList(Field), parent: ?*Scope };
pub const Parser = struct {
    functions: ArrayList(Function),
    types: ArrayList(Type),
    scope: ?*Scope,
    alloc: std.mem.Allocator,
    pub fn push_scope(this: *@This()) !void {
        var scope = try this.alloc.create(Scope);
        scope.parent = this.scope;
        scope.vars = try ArrayList(Field).initCapacity(this.alloc, 16);
        this.scope = scope;
    }
    pub fn pop_scope(this: *@This()) !void {
        if (this.scope == null) {
            return error.popped_scope_when_there_is_none;
        }
        const v = this.scope.?;
        this.scope = v.parent;
        this.alloc.destroy(v);
    }
};

pub const Field = struct {
    vtype: *Type,
    name: []const u8,
    offset: usize,
    size: usize,
};
pub const Type = union(enum) {
    t_int,
    t_flt,
    t_str,
    t_struct: struct {
        name: []const u8,
        fields: []Field,
    },
    pub fn print(this: *const @This()) void {
        std.debug.print("{s}\n", .{@tagName(this.*)});
    }
};
pub const Statement = union(enum) {
    basicExpr: Expr,
    if_statement: struct { cond: Expr, instructions: []Statement, elsef: []Statement },
    while_loop: struct { cond: Expr, instructions: []Statement },
    declare: struct {
        varname: []const u8,
        vtype: *Type,
        expr: Expr,
    },
    pub fn print(this: *const @This()) void {
        switch (this.*) {
            .basicExpr => |*expr| {
                std.debug.print("statement:", .{});
                expr.print();
            },
            .if_statement => |*f| {
                std.debug.print("if:", .{});
                f.cond.print();
                std.debug.print("do:", .{});
                for (f.instructions) |i| {
                    i.print();
                }
                std.debug.print("else:\n", .{});
                for (f.elsef) |i| {
                    i.print();
                }
                std.debug.print("end\n", .{});
            },
            .while_loop => |*p| {
                std.debug.print("while:\n", .{});
                p.cond.print();
                std.debug.print("do:\n", .{});
                for (p.instructions) |i| {
                    i.print();
                }
                std.debug.print("end\n", .{});
            },
            .declare => |*x| {
                std.debug.print("let {s}:", .{x.varname});
                x.vtype.print();
                std.debug.print("then:\n", .{});
                x.expr.print();
                std.debug.print("end\n", .{});
            },
        }
    }
};
pub const Program = struct {
    functions: ArrayList(Function),
    types: ArrayList(Type),
    pub fn print(this: *const @This()) void {
        for (this.types.items) |i| {
            i.print();
        }
        for (this.functions.items) |i| {
            i.print();
        }
    }
};
pub fn token_equals(l: Token, r: Token) bool {
    return l.tt == r.tt;
}
pub fn done_paren(str: *const Stream(Token)) bool {
    if (str.count_matches(TokenType.TokenOpenParen, TokenType.TokenCloseParen, token_equals)) {
        return true;
    }
    return false;
}
pub fn done_curly(str: *const Stream(Token)) bool {
    if (str.count_matches(TokenType.TokenOpenCUrly, TokenType.TokenCloseCurl, token_equals)) {
        return true;
    }
    return false;
}
pub fn done_semi(str: *const Stream(Token)) bool {
    str.contains(TokenType.TokenSemi, token_equals);
}
pub fn parse_statement(parse: Parser, s: Stream(Token)) !Statement {
    _ = s;
    _ = parse;
    return error.Unimplemented;
}
pub fn parse_scope(parse: Parser, s: Stream(Token)) !ArrayList(Statement) {
    _ = s;
    _ = parse;
    return error.Unimplemented;
}
pub fn parse_type(parse: Parser, s: Token) !*Type {
    _ = s;
    _ = parse;
    return Type.t_int;
}
pub fn parse_token_stream(alloc: Alloc, tokens: []Token) anyerror!Program {
    var stream = Stream(Token).new(tokens);
    var parse = Parser{ .alloc = alloc, .functions = try ArrayList(Function).initCapacity(alloc, 64), .scope = null, .types = try ArrayList(Type).initCapacity(alloc, 64) };
    while (true) {
        const sp = stream.try_next();
        if (sp == null) {
            break;
        }
        const s = sp.?;
        if (s.equals("defun")) {
            const name = try stream.next();
            const vt = try stream.next();
            if ((try stream.peek()).tt != TokenType.TokenOpenParen) {
                return error.expected_paren;
            }
            var expr = stream.collect(done_paren);
            var args = try ArrayList(Field).initCapacity(alloc, 16);
            _ = expr.try_next();
            while (true) {
                const arg_name = (try expr.next()).str;
                const arg_type = try parse_type(parse, try expr.next());
                if ((try expr.peek()).tt == TokenType.TokenCloseParen) {
                    break;
                }
                args.append(alloc, Field{ .name = arg_name, .vtype = arg_type, .offset = 0, .size = 8 });
            }
            const scope = stream.collect(done_curly);
            const scope_v = try parse_scope(scope);
            var f = Function{};
            f.args = args;
            f.instructions = scope_v;
            f.name = name;
            f.rv = parse_type(parse, vt);
            try parse.functions.append(alloc, f);
        } else if (s.equals("deftype")) {
            const name = try stream.next();
            _ = name;
            var scope = stream.collect(done_curly);
            _ = scope.next();
        }
    }
    return error.bruh;
}
