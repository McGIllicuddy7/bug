const std = @import("std");
const c = @cImport(@cInclude("stdio.h"));
const utils = @import("utils.zig");
const l_lang = @import("l_lang");
const tokens = @import("tokens.zig");
const parse = @import("parser.zig");
const arena = std.heap.ArenaAllocator;
pub fn getline(buf: []u8) ![]u8 {
    var stdin = std.fs.File.stdin();
    var int = stdin.reader(buf).interface;
    return try int.takeSentinel('\n');
}
pub fn main() !void {
    var ar = arena.init(std.heap.c_allocator);
    var finished = false;
    while (true) {
        if (finished) {
            break;
        }
        var buf: [128]u8 = undefined;
        var buff: []u8 = &buf;
        buff = try getline(buff);
        var t = tokens.TokenStream.new(ar.allocator(), buff, "stdin");
        var tlist = try std.ArrayList(tokens.Token).initCapacity(ar.allocator(), 64);
        while (true) {
            const tv = try t.next();
            if (tv == null) {
                break;
            }
            const k = tv.?;
            if (tokens.str_equals(k.str, "exit")) {
                finished = true;
                break;
            }
            try tlist.append(ar.allocator(), k);
            // k.print();
        }
        const expr = try parse.parse_expression(ar.allocator(), tlist.items);
        expr.print();
    }
}
