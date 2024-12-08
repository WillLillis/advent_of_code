const std = @import("std");

const Position = struct {
    row: usize,
    col: usize,

    fn next(self: Position, dir: Direction, n_rows: usize, n_cols: usize) ?Position {
        switch (dir) {
            .up => {
                if (self.row == 0) {
                    return null;
                } else {
                    return Position{ .row = self.row - 1, .col = self.col };
                }
            },
            .right => {
                if (self.col >= n_cols - 1) {
                    return null;
                } else {
                    return Position{ .row = self.row, .col = self.col + 1 };
                }
            },
            .down => {
                if (self.row >= n_rows - 1) {
                    return null;
                } else {
                    return Position{ .row = self.row + 1, .col = self.col };
                }
            },
            .left => {
                if (self.col == 0) {
                    return null;
                } else {
                    return Position{ .row = self.row, .col = self.col - 1 };
                }
            },
        }
    }
};

const Direction = enum {
    up,
    right,
    down,
    left,

    fn next(self: Direction) Direction {
        return switch (self) {
            .up => .right,
            .right => .down,
            .down => .left,
            .left => .up,
        };
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
    var blocked_positions = std.AutoHashMap(Position, void).init(allocator);
    defer blocked_positions.deinit();

    const n_rows = std.mem.count(u8, input, "\n");
    const n_cols = std.mem.indexOf(u8, input, "\n").?;

    var guard_pos: Position = undefined;
    var guard_dir = Direction.up;
    var row: usize = 0;
    while (lines.next()) |line| : (row += 1) {
        for (line, 0..) |c, col| {
            if (c == '#') {
                try blocked_positions.put(Position{ .row = row, .col = col }, {});
            } else if (c == '^') {
                guard_pos = Position{ .row = row, .col = col };
            }
        }
    }
    const guard_start_pos = guard_pos;

    var occupied_positions = std.AutoArrayHashMap(Position, [4]bool).init(allocator);
    defer occupied_positions.deinit();

    var paradox_positions = std.AutoArrayHashMap(Position, void).init(allocator);
    defer paradox_positions.deinit();

    // Need to simulate once, then check!
    try occupied_positions.put(guard_pos, [4]bool{ true, false, false, false });
    guard_sim: while (true) {
        var next_pos = guard_pos.next(guard_dir, n_rows, n_cols) orelse break;
        while (blocked_positions.contains(next_pos)) {
            guard_dir = guard_dir.next();
            next_pos = guard_pos.next(guard_dir, n_rows, n_cols) orelse break :guard_sim;
        }
        guard_pos = next_pos;

        if (occupied_positions.getPtr(guard_pos)) |markers| {
            markers[@intFromEnum(guard_dir)] = true;
        } else {
            var marker = [4]bool{ false, false, false, false };
            marker[@intFromEnum(guard_dir)] = true;
            try occupied_positions.put(guard_pos, marker);
        }
    }

    std.debug.print("{} positions\n", .{occupied_positions.count()});

    var check_positions = std.AutoArrayHashMap(Position, [4]bool).init(allocator);
    defer check_positions.deinit();
    var postitions_iter = occupied_positions.iterator();
    _ = postitions_iter.next(); // skip guard's initial position

    // iterate over all guard positions
    while (postitions_iter.next()) |pair| {
        try blocked_positions.put(pair.key_ptr.*, {});
        // reset post simulation
        defer {
            check_positions.clearRetainingCapacity();
            guard_pos = guard_start_pos;
            guard_dir = Direction.up;
            _ = blocked_positions.remove(pair.key_ptr.*);
        }
        // simulate the guard moving with the next obstacle
        {
            try occupied_positions.put(guard_pos, [4]bool{ true, false, false, false });
            guard_sim: while (true) {
                var next_pos = guard_pos.next(guard_dir, n_rows, n_cols) orelse break;
                while (blocked_positions.contains(next_pos)) {
                    guard_dir = guard_dir.next();
                    next_pos = guard_pos.next(guard_dir, n_rows, n_cols) orelse break :guard_sim;
                }
                guard_pos = next_pos;

                if (check_positions.getPtr(guard_pos)) |markers| {
                    if (markers[@intFromEnum(guard_dir)]) {
                        // we've hit a cycle!
                        try paradox_positions.put(pair.key_ptr.*, {});
                        break :guard_sim;
                    }
                    markers[@intFromEnum(guard_dir)] = true;
                } else {
                    var marker = [4]bool{ false, false, false, false };
                    marker[@intFromEnum(guard_dir)] = true;
                    try check_positions.put(guard_pos, marker);
                }
            }
        }
    }

    std.debug.print("{} paradox positions\n", .{paradox_positions.count()});
}
