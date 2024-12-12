const std = @import("std");

const BlinkResult = struct {
    num1: u64,
    num2: ?u64,
};

fn n_digits(val: u64) u64 {
    return std.math.log10(val) + 1;
}

fn blink_num(curr_num: u64) !BlinkResult {
    if (curr_num == 0) {
        return .{ .num1 = 1, .num2 = null };
    } else if (n_digits(curr_num) % 2 == 0) {
        const digits = n_digits(curr_num);
        const left = curr_num / try std.math.powi(u64, 10, digits / 2);
        const right = blk: {
            var buf: [64]u8 = undefined;
            const printed = try std.fmt.bufPrint(&buf, "{}", .{curr_num});
            break :blk try std.fmt.parseUnsigned(u64, printed[digits / 2 ..], 10);
        };
        return .{ .num1 = left, .num2 = right };
    } else {
        return .{ .num1 = curr_num * 2024, .num2 = null };
    }
}

pub fn main() !void {
    const file_name = "input.txt";
    var file = std.fs.cwd();
    const input_file = try file.openFile(file_name, .{});
    defer input_file.close();
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        const deinit_status = gpa.deinit();
        if (deinit_status == .leak) @panic("MEM LEAK");
    }
    const input = try input_file.readToEndAlloc(allocator, 1000000);
    defer allocator.free(input);
    var vals = std.mem.splitScalar(u8, input, ' ');

    var store1 = std.AutoHashMap(u64, usize).init(allocator);
    defer store1.deinit();
    var store2 = std.AutoHashMap(u64, usize).init(allocator);
    defer store2.deinit();
    var curr_nums = &store1;
    var next_nums = &store2;

    while (vals.next()) |val| {
        const cleaned = std.mem.trim(u8, val, " \n");
        const parsed = try std.fmt.parseUnsigned(u64, cleaned, 10);
        if (store1.getPtr(parsed)) |stored| {
            stored.* = stored.* + 1;
        } else {
            try store1.put(parsed, 1);
        }
    }

    for (0..75) |_| {
        var val_iter = curr_nums.iterator();
        while (val_iter.next()) |val| {
            const curr_num = val.key_ptr.*;
            const curr_count = val.value_ptr.*;
            const blinked_nums = try blink_num(curr_num);

            if (next_nums.getPtr(blinked_nums.num1)) |count| {
                count.* = count.* + curr_count;
            } else {
                try next_nums.put(blinked_nums.num1, curr_count);
            }
            if (blinked_nums.num2) |other| {
                if (next_nums.getPtr(other)) |count| {
                    count.* = count.* + curr_count;
                } else {
                    try next_nums.put(other, curr_count);
                }
            }
        }
        // swap the pointers
        const tmp = curr_nums;
        curr_nums = next_nums;
        next_nums = tmp;

        next_nums.clearRetainingCapacity();
    }

    var count: usize = 0;
    var val_iter = curr_nums.valueIterator();
    while (val_iter.next()) |val| {
        count += val.*;
    }

    std.debug.print("{} stones\n", .{count});

    // var val_iter = curr_nums.iterator();
    // while (val_iter.next()) |val| {
    //     std.debug.print("{}, {}\n", .{ val.key_ptr.*, val.value_ptr.* });
    // }
}
