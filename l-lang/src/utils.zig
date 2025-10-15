const std = @import("std");
pub fn todo() void {
    std.debug.dumpCurrentStackTrace(null);
    std.debug.panic("to do", .{});
}
pub fn contains(comptime T: type, list: []const T, v: T) bool {
    for (0..list.len) |i| {
        if (list[i] == v) {
            return true;
        }
    }
    return false;
}
pub fn stream(comptime T: type) type {
    return struct {
        values: []const T,
        pub fn new(v: []const T) @This() {
            return .{ .values = v };
        }
        pub fn try_next(this: *@This()) ?T {
            if (this.values.len > 0) {
                const out = this.values[0];
                this.values = this.values[1..];
                return out;
            }
            return null;
        }
        pub fn next(this: *@This()) !T {
            const t = this.try_next();
            if (t == null) {
                return error.expected_token;
            } else {
                return t.?;
            }
        }
        pub fn try_peek(this: *const @This()) ?T {
            if (this.values.len > 0) {
                const out = this.values[0];
                return out;
            }
            return null;
        }
        pub fn peek(this: *const @This()) !T {
            const t = this.try_peek();
            if (t == null) {
                return error.expected_token;
            } else {
                return t.?;
            }
        }
        pub fn collect(this: *@This(), done: fn (*const @This()) bool) @This() {
            var out = @This().new(this.values[0..0]);
            while (done(&out) and this.values.len > 0) {
                out.values.len += 1;
                _ = this.try_next();
            }
            return out;
        }
        pub fn count(v: *const @This(), to_count: T, equals: fn (T, T) bool) usize {
            var tcount: usize = 0;
            for (v.values) |i| {
                if (equals(i, to_count)) {
                    tcount += 1;
                }
            }
            return tcount;
        }
        pub fn count_matches(v: *const @This(), l: T, r: T, equals: fn (T, T) bool) bool {
            return v.count(l, equals) == r.count(l, equals);
        }
    };
}
