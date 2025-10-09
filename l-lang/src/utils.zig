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
