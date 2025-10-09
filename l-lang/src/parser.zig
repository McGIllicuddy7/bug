pub const tokens_f = @import("tokens.zig");
const std = @import("std");
pub const Token = tokens_f.Token;
pub const TokenType = tokens_f.TokenType;
pub const Alloc = std.mem.Allocator;
const Arena = std.heap.ArenaAllocator;
const ArrayList = std.ArrayList;
const Eval = struct { oprs: ArrayList(Opr), ops: ArrayList(Op), vars: ArrayList(Var), sp: i32, al: std.mem.Allocator };
pub const Var = struct { t: Token, sv: i32 };
pub const Expr = struct {
    ops: ArrayList(Opr),
    pub fn print(s: *const @This()) void {
        const op_names: []const []const u8 = &[_][]const u8{ "+", "-", "*", "/", "=", "unknown what this is", "num", "float", "str", "ident", "field", "call" };
        for (0..s.ops.items.len) |i| {
            const o = s.ops.items[i];
            if (@intFromEnum(o.t) < @intFromEnum(OpType.o_num)) {
                std.debug.print("exp:{s}\n", .{op_names[@intFromEnum(o.t)]});
            } else if (o.t == OpType.o_num) {
                std.debug.print("exp:{any}\n", .{o.v.v});
            } else if (o.t == OpType.o_flt) {
                std.debug.print("exp:{any}\n", .{o.v.f});
            } else if (o.t == OpType.o_str or o.t == OpType.o_idnt) {
                std.debug.print("exp ident:{s}\n", .{o.v.s});
            } else if (o.t == OpType.o_call) {
                std.debug.print("call:{x}\n", .{o.v.v});
            } else if (o.t == OpType.o_fld) {
                std.debug.print("field_access\n", .{});
            } else {}
        }
    }
};
pub const utils = @import("utils.zig");
const OpType = enum {
    o_ad, //add
    o_sb, //subtract
    o_ml, //multiply
    o_dv, //divide
    o_as, //assign
    o_gt, //goto
    o_cgt, //conditional_goto
    o_dec, //declare
    o_type, //type operator
    o_num, //number operator
    o_flt, //float operator
    o_str, //string operator
    o_idnt, //indentifer operator
    o_fld, //field access
    o_call, //call
    o_function,
    o_auto_dec, //auto_declare
};
const Opr = struct {
    t: OpType,
    v: union {
        s: []const u8,
        v: i64,
        f: f64,
    },
};
const Op = enum {
    Er,
    OpenParen,
    CloseParen,
    Add,
    Sub,
    Mul,
    Div,
    Call,
    Dot,
    Colon,
    ColonEquals,
    Assign,
};
fn op_prior(o: Op) i32 {
    if (o == Op.Mul) {
        return 2;
    } else if (o == Op.Div) {
        return 2;
    } else if (o == Op.Sub) {
        return 1;
    } else if (o == Op.Add) {
        return 1;
    } else if (o == Op.Assign) {
        return 0;
    } else if (o == Op.Dot) {
        return 3;
    } else if (o == Op.Colon) {
        return 4;
    } else if (o == Op.ColonEquals) {
        return 4;
    } else if (o == Op.OpenParen) {
        return -1;
    } else {
        utils.todo();
    }
    return -1;
}
pub fn eval_op(o: Op, ev: *Eval) !void {
    if (ev.vars.items.len > 0) {
        const v0 = ev.vars.items[ev.vars.items.len - 1];
        ev.vars.items.len -= 1;
        try write_var(ev, v0);
    }
    if (ev.vars.items.len > 0) {
        const v1 = ev.vars.items[ev.vars.items.len - 1];
        ev.vars.items.len -= 1;
        try write_var(ev, v1);
    }
    var op: Opr = undefined;
    if (o == Op.Add) {
        op.t = OpType.o_ad;
    } else if (o == Op.Sub) {
        op.t = OpType.o_sb;
    } else if (o == Op.Div) {
        op.t = OpType.o_dv;
    } else if (o == Op.Mul) {
        op.t = OpType.o_ml;
    } else if (o == Op.Assign) {
        op.t = OpType.o_as;
    } else if (o == Op.Dot) {
        op.t = OpType.o_fld;
    } else if (o == Op.Colon) {
        op.t = OpType.o_dec;
    } else if (o == Op.ColonEquals) {
        op.t = OpType.o_auto_dec;
    } else {
        utils.todo();
    }
    try ev.oprs.append(ev.al, op);
}
pub fn write_var(ev: *Eval, v: Var) !void {
    var o: Opr = undefined;
    if (v.t.tt == TokenType.TokenInt) {
        o = Opr{ .t = OpType.o_num, .v = @TypeOf(o.v){
            .v = 0,
        } };
        o.t = OpType.o_num;
        o.v.v = try std.fmt.parseInt(i64, v.t.str, 10);
    } else if (v.t.tt == TokenType.TokenFloat) {
        o = Opr{ .t = OpType.o_num, .v = @TypeOf(o.v){
            .f = 0.0,
        } };
        o.t = OpType.o_flt;
        o.v.f = try std.fmt.parseFloat(f64, v.t.str);
    } else if (v.t.tt == TokenType.TokenStr) {
        o = Opr{ .t = OpType.o_num, .v = @TypeOf(o.v){
            .s = "",
        } };
        o.t = OpType.o_str;
        o.v.s = v.t.str;
    } else if (v.t.tt == TokenType.TokenIdent) {
        o = Opr{ .t = OpType.o_num, .v = @TypeOf(o.v){
            .s = "",
        } };
        o.t = OpType.o_idnt;
        o.v.s = v.t.str;
    } else {
        utils.todo();
        return;
    }
    try ev.oprs.append(ev.al, o);
}
pub fn get_next_outside_of_expr(tokens: []Token, start: usize, t: TokenType) i64 {
    var paren_count: i64 = 0;
    var curly_count: i64 = 0;
    for (start..tokens.len) |i| {
        if (paren_count == 0 and curly_count == 0) {
            if (tokens[i].tt == t) {
                return @intCast(i);
            }
        }
        if (tokens[i].tt == TokenType.TokenOpenParen) {
            paren_count += 1;
        } else if (tokens[i].tt == TokenType.TokenCloseParen) {
            paren_count -= 1;
        }
        if (tokens[i].tt == TokenType.TokenOpenCurl) {
            curly_count += 1;
        } else if (tokens[i].tt == TokenType.TokenCloseCurl) {
            curly_count -= 1;
        }
    }
    return -1;
}
pub fn parse_expression(alloc: Alloc, tokens: []Token) !Expr {
    var local = Arena.init(std.heap.c_allocator);
    const al = local.allocator();
    var ev: Eval = undefined;
    ev.ops = try ArrayList(Op).initCapacity(al, 64);
    ev.oprs = try ArrayList(Opr).initCapacity(alloc, 64);
    ev.vars = try ArrayList(Var).initCapacity(al, 64);
    ev.sp = 0;
    ev.al = alloc;
    defer local.deinit();
    var i: usize = 0;
    var last_was_v = false;
    while (i < tokens.len) {
        if (tokens[i].tt == TokenType.TokenInt or tokens[i].tt == TokenType.TokenFloat or tokens[i].tt == TokenType.TokenStr or tokens[i].tt == TokenType.TokenIdent) {
            var v: Var = undefined;
            v.t = tokens[i];
            v.sv = -1;
            try ev.vars.append(al, v);
            last_was_v = true;
        } else if (tokens[i].tt == TokenType.TokenOpenParen) {
            if (last_was_v) {
                i += 1;
                const v = ev.vars.items[ev.vars.items.len - 1];
                ev.vars.items.len -= 1;
                const end = get_next_outside_of_expr(tokens, i, TokenType.TokenCloseParen);
                if (end == -1) {
                    return error.ExprDoesntend;
                }
                var arg_count: i64 = 0;
                while (i < end) {
                    var e = get_next_outside_of_expr(tokens, i, TokenType.TokenComma);
                    if (e == -1 or e > end) {
                        e = end;
                    }
                    const ep = try parse_expression(alloc, tokens[i..@as(usize, @intCast(e))]);
                    arg_count += 1;
                    for (0..ep.ops.items.len) |j| {
                        try ev.oprs.append(alloc, ep.ops.items[j]);
                    }
                    i = @as(usize, @intCast(e)) + 1;
                }
                const _t: Opr = undefined;
                last_was_v = true;
                var op = Opr{ .t = OpType.o_idnt, .v = @TypeOf(_t.v){ .s = v.t.str } };
                try ev.oprs.append(alloc, op);
                last_was_v = true;
                op = Opr{ .t = OpType.o_call, .v = @TypeOf(_t.v){ .v = arg_count } };
                try ev.oprs.append(alloc, op);
            } else {
                last_was_v = false;
                try ev.ops.append(al, Op.OpenParen);
            }
        } else if (tokens[i].tt == TokenType.TokenCloseParen) {
            while (ev.ops.items[ev.ops.items.len - 1] != Op.OpenParen) {
                const o = ev.ops.items[ev.ops.items.len - 1];
                ev.ops.items.len -= 1;
                try eval_op(o, &ev);
            }
            ev.ops.items.len -= 1;
            last_was_v = true;
        } else if (tokens[i].tt == TokenType.TokenDot or tokens[i].tt == TokenType.TokenOperator or tokens[i].tt == TokenType.TokenColon) {
            last_was_v = false;
            var o: Op = undefined;
            if (tokens[i].tt == TokenType.TokenDot) {
                o = Op.Dot;
            } else if (tokens[i].equals("+")) {
                o = Op.Add;
            } else if (tokens[i].equals("-")) {
                o = Op.Sub;
            } else if (tokens[i].equals("*")) {
                o = Op.Mul;
            } else if (tokens[i].equals("/")) {
                o = Op.Div;
            } else if (tokens[i].equals(":")) {
                o = Op.Colon;
            } else if (tokens[i].equals(":=")) {
                o = Op.ColonEquals;
            } else {
                return error.invalid_expression;
            }
            while (ev.ops.items.len > 0) {
                if (op_prior(ev.ops.items[ev.ops.items.len - 1]) < op_prior(o)) {
                    break;
                }
                const t = ev.ops.items[ev.ops.items.len - 1];
                ev.ops.items.len -= 1;
                if (t == Op.OpenParen) {
                    break;
                }
                try eval_op(t, &ev);
            }
            try ev.ops.append(al, o);
            last_was_v = false;
        } else {
            tokens[i].print();
            return error.unsupported_token;
        }
        i += 1;
    }
    if (ev.ops.items.len == 0) {
        if (ev.vars.items.len == 1) {
            try write_var(&ev, ev.vars.items[0]);
            ev.vars.items.len -= 1;
        }
    }
    while (ev.ops.items.len > 0) {
        //std.debug.print("{any}", .{ev});
        const o = ev.ops.items[ev.ops.items.len - 1];
        if (o == Op.OpenParen or o == Op.CloseParen) {
            ev.ops.items.len -= 1;
            continue;
        }
        ev.ops.items.len -= 1;
        try eval_op(o, &ev);
    }
    i += 1;
    var out: Expr = undefined;
    out.ops = ev.oprs;
    return out;
}
pub const Function = struct { instructions: std.ArrayList(Statement) };
pub const Scope = struct { vars: std.ArrayList(Field), parent: ?*Scope };
pub const Parser = struct {
    functions: std.AutoHashMap(
        []u8,
        Function,
    ),
    scope: ?*Scope,
};

pub const Field = union(enum) {
    vtype: *Type,
    name: []const u8,
    offset: usize,
    size: usize,
};
pub const Type = union(enum) { t_int, t_flt, t_str, t_struct: struct {
    name: []const u8,
    fields: []Field,
} };
pub const Statement = union(enum) {
    basicExpr: Expr,
    if_statement: struct { cond: Expr, instructions: []Statement, elsef: []Statement },
    while_loop: struct { cond: Expr, instructions: []Statement },
    declare: struct {
        varname: []const u8,
        vtype: Type,
        expr: Expr,
    },
};
pub fn get_scope_end(tokens: []Token, start: i64) i64 {
    var count = 0;
    for (start..tokens.len) |i| {
        if (tokens[i].tt == TokenType.TokenOpenCurl) {
            count += 1;
        } else if (tokens[i].tt == TokenType.TokenCloseCurl) {
            count -= 1;
        }
        if (count == 0) {
            return @as(i64, @intCast(i));
        }
    }
    return -1;
}
pub fn get_statement_end(tokens: []Token) i64 {
    if (tokens[0].equals("if") or tokens[0].equals("while")) {
        const start = get_next_outside_of_expr(tokens, 1, TokenType.TokenOpenCurl);
        const end = get_scope_end(tokens, start);
        if (tokens.len() > end + 2) {
            if (tokens[0].equals("if") and tokens[end + 1].equals("else")) {
                return get_scope_end(tokens, end + 2);
            }
        }
        return end;
    } else {
        return get_next_outside_of_expr(tokens, 0, TokenType.TokenSemi);
    }
}
pub fn parse_statement(parser: *Parser, tokens: []Token) !Statement {
    var out: Statement = undefined;
    if (tokens[0].equals("if") or tokens[0].equals("while")) {
        const e1 = get_next_outside_of_expr(tokens, 1, TokenType.TokenOpenCurl);
        const expr = try parse_expression(alloc, tokens[1 .. e1 - 2]);
        const e2 = get_scope_end(tokens, e1);
        const scope = try parse_scope(alloc, parser, tokens[e1 + 1 .. e2]);

        if (tokens[0].equals("if")) {
            out = .{ .if_statement = .{ expr, scope.items, []Statement{} } };
            if (tokens[e2 + 1].equals("else")) {
                const e3 = get_scope_end(tokens, e2 + 2);
                const scope2 = try parse_scope(alloc, parser, tokens[e2 + 3 .. e3]);
                out.if_statement.elsef = scope2.items;
            }
        } else {
            out = .{ .while_loop = .{ expr, scope.items } };
        }
        return out;
    }
    const end = get_statement_end(tokens);
    if (tokens[0].equals("let")) {
        const name = tokens[1];
        const t = tokens[2];
        const expr = try parse_expression(alloc, tokens[2..end]);
        out = .{ .declare = .{ .varname = name.str, .vtype = try parse_type(parser, t), .epxpr = expr } };
    }
}
pub fn parse_scope(parser: *Parser, tokens: []Token) !ArrayList(Statement) {
    var i: i64 = 0;
    var out = try ArrayList(Statement).initCapacity(parser.alloc, 64);
    while (i < tokens.len) {
        const e = get_statement_end(tokens[i..]);
        const s = try parse_statement(parser, tokens[i..e]);
        try out.append(parser.alloc, s);
        i = e + 1;
    }
}
