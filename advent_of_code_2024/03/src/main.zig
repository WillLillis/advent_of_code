const std = @import("std");

const ParseStateTag = enum {
    start,
    @"mul(",
    num1, // the number AND the comma
    num2, // the number AND the rparen
    finish,
    @"do()",
    @"don't()",
};

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

    var enabled = true;
    var index: usize = 0;
    var total: u128 = 0;
    var number1: ?u128 = null;
    var number2: ?u128 = null;
    std.debug.print("{s}\n", .{input});
    state: switch (ParseStateTag.start) {
        .start => {
            std.debug.print("[.start] Reached\n", .{});
            if (!(index + "mul(0,0)".len >= input.len)) {
                std.debug.print("[.start] Continuing to .do() state\n", .{});
                continue :state .@"do()";
            }
        },
        .@"do()" => {
            std.debug.print("[.do()] Reached\n", .{});
            std.debug.print("[.do()] Checking against {s} \n", .{input[index .. index + "do()".len]});
            if (index + "do()".len <= input.len and std.mem.eql(u8, "do()", input[index .. index + "do()".len])) {
                std.debug.print("[.do()] Matched\n", .{});
                enabled = true;
                index += "do()".len;
                continue :state .start;
            } else {
                std.debug.print("[.do()] Didn't match\n", .{});
                continue :state .@"don't()";
            }
        },
        .@"don't()" => {
            std.debug.print("[.dont't()] Reached\n", .{});
            std.debug.print("[.don't()] Checking against {s} \n", .{input[index .. index + "don't()".len]});
            if (index + "don't()".len <= input.len and std.mem.eql(u8, "don't()", input[index .. index + "don't()".len])) {
                std.debug.print("[.don't()] Matched\n", .{});
                enabled = false;
                index += "don't()".len;
                continue :state .start;
            } else {
                std.debug.print("[.don't()] Didn't match\n", .{});
                continue :state .@"mul(";
            }
        },
        .@"mul(" => {
            std.debug.print("[.mul(] Reached\n", .{});
            for (0..4) |i| {
                std.debug.print("[.mul(] Looking at {c} ({})\n", .{ input[index], i });
                const expected: u8 = switch (i) {
                    0 => 'm',
                    1 => 'u',
                    2 => 'l',
                    3 => '(',
                    else => continue :state .start,
                };
                defer index += 1;
                if (input[index] != expected) {
                    continue :state .start;
                } else if (i == 3) {
                    continue :state .num1;
                }
            }
        },
        .num1 => {
            std.debug.print("[.num1] Reached\n", .{});
            const start_idx = index;
            for (0..3) |i| {
                std.debug.print("[.num1] Looking at {c} ({})\n", .{ input[index], i });
                switch (input[index]) {
                    '0'...'9' => index += 1,
                    ',' => {
                        std.debug.print("[.num1] Parsing {s}\n", .{input[start_idx..index]});
                        number1 = try std.fmt.parseUnsigned(u128, input[start_idx..index], 10);
                        index += 1;
                        continue :state .num2;
                    },
                    else => {
                        continue :state .start;
                    },
                }
            }
            // we can fall out of the loop if there are 3 valid digits
            if (input[index] == ',') {
                std.debug.print("[.num1] Parsing {s}\n", .{input[start_idx..index]});
                number1 = try std.fmt.parseUnsigned(u128, input[start_idx..index], 10);
                index += 1;
                continue :state .num2;
            } else {
                continue :state .start;
            }
        },
        .num2 => {
            std.debug.print("[.num2] Reached\n", .{});
            const start_idx = index;
            for (0..3) |i| {
                std.debug.print("[.num2] Looking at {c} ({})\n", .{ input[index], i });
                switch (input[index]) {
                    '0'...'9' => index += 1,
                    ')' => {
                        std.debug.print("[.num2] Parsing {s}\n", .{input[start_idx..index]});
                        number2 = try std.fmt.parseUnsigned(u128, input[start_idx..index], 10);
                        // `index` is incremented in the `.finish` block
                        continue :state .finish;
                    },
                    else => {
                        number1 = null;
                        continue :state .start;
                    },
                }
            }
            // we can fall out of the loop if there are 3 valid digits
            if (input[index] == ')') {
                std.debug.print("[.num2] Parsing {s}\n", .{input[start_idx..index]});
                number2 = try std.fmt.parseUnsigned(u128, input[start_idx..index], 10);
                // `index` is incremented in the `.finish` block
                continue :state .finish;
            } else {
                continue :state .start;
            }
        },
        .finish => {
            std.debug.print("[.finish] Reached\n", .{});
            std.debug.print("[.finish] Adding {} * {} = {}\n", .{
                number1.?,
                number2.?,
                number1.? * number2.?,
            });
            if (enabled) {
                std.debug.print("[.finish] Adding\n", .{});
                total += @as(u128, @intCast(number1.? * number2.?));
            } else {
                std.debug.print("[.finish] Not adding\n", .{});
            }
            number1 = null;
            number2 = null;
            index += 1;
            continue :state .start;
        },
    }

    std.debug.print("Total: {}\n", .{total});
}
