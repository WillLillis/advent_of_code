const std = @import("std");

const Operation = enum {
    add,
    mul,
    concat,
};

const numOps: usize = @typeInfo(Operation).@"enum".fields.len;

const Equation = struct {
    result: u128,
    operands: std.ArrayListUnmanaged(u128),

    fn isValid(self: *const Equation) !bool {
        std.debug.assert(self.operands.items.len > 0);
        const n_combinations = std.math.pow(usize, numOps, self.operands.items.len - 1);
        for (0..n_combinations) |n_comb| {
            var curr_amount = self.operands.items[0];
            var comb_num = n_comb;

            for (1..self.operands.items.len) |i_num| {
                const curr_op: Operation = @enumFromInt(comb_num % numOps);
                const curr_num = self.operands.items[i_num];
                switch (curr_op) {
                    .add => curr_amount += curr_num,
                    .mul => curr_amount *= curr_num,
                    .concat => {
                        // GOOD: 
                        var buf: [256]u8 = undefined;
                        const concatted = try std.fmt.bufPrint(buf[0..], "{}{}", .{ curr_amount, curr_num });
                        const new_num = try std.fmt.parseUnsigned(u64, concatted, 10);
                        curr_amount = new_num;
                        // BAD: 
                        // var curr_digits = @as(u128, @intFromFloat(@log10(@as(f128, @floatFromInt(curr_num))))) + 1;
                        // curr_digits += @as(u128, @intFromFloat(@log10(@as(f128, @floatFromInt(curr_amount))))) -| 1;
                        // const multiplier = std.math.pow(u128, 10, curr_digits);
                        // const add_amount = curr_amount * multiplier;
                        // curr_amount = curr_num + add_amount;
                    },
                }

                comb_num /= numOps;
            }

            if (curr_amount == self.result) {
                return true;
            }
        }

        return false;
    }
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
    var lines = std.mem.splitScalar(u8, input, '\n');

    var equations = std.ArrayList(Equation).init(allocator);
    defer {
        for (equations.items) |*eq| {
            eq.operands.deinit(allocator);
        }
        equations.deinit();
    }

    while (lines.next()) |line| {
        if (line.len == 0) break;

        const colon_idx = std.mem.indexOfScalar(u8, line, ':').?;
        const result = try std.fmt.parseUnsigned(u128, line[0..colon_idx], 10);
        var op_strs = std.mem.splitScalar(u8, line[colon_idx + 1 ..], ' ');
        const n_items = std.mem.count(u8, line[colon_idx + 1 ..], " ");
        var operands = try std.ArrayListUnmanaged(u128).initCapacity(allocator, n_items + 1);
        errdefer operands.deinit(allocator);
        while (op_strs.next()) |op| {
            if (op.len == 0) continue;
            const num = try std.fmt.parseUnsigned(u128, op, 10);
            operands.appendAssumeCapacity(num);
        }
        try equations.append(Equation{ .result = result, .operands = operands });
    }

    var total: u128 = 0;
    for (equations.items) |eq| {
        if (try eq.isValid()) {
            total += eq.result;
        }
    }

    std.debug.print("Total: {}\n", .{total});
}
