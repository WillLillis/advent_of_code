const std = @import("std");

fn lessThan(ctx: void, lhs: usize, rhs: usize) bool {
    _ = ctx;
    return lhs < rhs;
}

pub fn main() !void {
    const file_name = "input1.txt";
    var file = std.fs.cwd();
    const input_file = try file.openFile(file_name, .{});
    defer input_file.close();
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        const deinit_status = gpa.deinit();
        if (deinit_status == .leak) @panic("MEM LEAK");
    }
    const raw_input = try input_file.readToEndAlloc(allocator, 1000000);
    defer allocator.free(raw_input);
    var lines = std.mem.splitScalar(u8, raw_input, '\n');

    var list_1 = std.ArrayList(u64).init(allocator);
    defer list_1.deinit();
    var list_2 = std.ArrayList(u64).init(allocator);
    defer list_2.deinit();
    while (lines.next()) |line| {
        if (line.len == 0) break;
        {
            var num_1_end: ?usize = null;
            var idx_1: usize = 0;
            while (idx_1 < line.len) : (idx_1 += 1) {
                if (line[idx_1] == ' ') {
                    num_1_end = idx_1;
                    break;
                }
            }
            try list_1.append(try std.fmt.parseUnsigned(u64, line[0..num_1_end.?], 10));
        }
        {
            var num_2_start: ?usize = null;
            var idx_2 = line.len - 1;
            while (idx_2 >= 0) : (idx_2 -= 1) {
                if (line[idx_2] == ' ') {
                    num_2_start = idx_2 + 1;
                    break;
                }
            }
            try list_2.append(try std.fmt.parseUnsigned(u64, line[num_2_start.?..], 10));
        }
    }

    // Part 1
    {
        std.mem.sort(u64, list_1.items, {}, lessThan);
        std.mem.sort(u64, list_2.items, {}, lessThan);

        var distance: u64 = 0;
        var i: usize = 0;
        while (i < list_1.items.len) : (i += 1) {
            distance += @abs(@as(i64, @intCast(list_1.items[i])) - @as(i64, @intCast(list_2.items[i])));
        }

        // std.debug.print("Distance: {}\n", .{distance});
    }

    // Part 2
    // {
    //     std.mem.sort(u64, list_2.items, {}, lessThan);
    //
    //     var similarity: u64 = 0;
    //     for (list_1.items) |entry| {
    //         var count: u64 = 0;
    //         if (std.mem.indexOfScalar(u64, list_2.items, entry)) |first_idx| {
    //             var i = first_idx;
    //             while (list_2.items[i] == entry) : (i += 1) {}
    //             count += i - first_idx;
    //         }
    //         similarity += entry * count;
    //     }
    //
    //     // std.debug.print("Similarity: {}\n", .{similarity});
    // }
}
