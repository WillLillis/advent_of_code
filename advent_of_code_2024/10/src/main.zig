const std = @import("std");

const Position = struct {
    row: usize,
    col: usize,
};

const Direction = enum {
    up,
    down,
    left,
    right,
};

const Grid = std.ArrayList(std.ArrayList(u8));

fn isGradualUphill(pos1: Position, pos2: Position, grid: *const Grid) bool {
    const elev1 = grid.items[pos1.row].items[pos1.col];
    const elev2 = grid.items[pos2.row].items[pos2.col];

    return elev1 < elev2 and elev2 - elev1 == 1;
}

fn nextGoodPos(curr_pos: Position, dir: Direction, grid: *const Grid) ?Position {
    const n_rows = grid.items.len;
    const n_cols = grid.items[0].items.len;
    const next_pos = switch (dir) {
        .up => pos: {
            if (curr_pos.row == 0) return null;
            break :pos .{ .row = curr_pos.row - 1, .col = curr_pos.col };
        },
        .down => pos: {
            if (curr_pos.row >= n_rows - 1) return null;
            break :pos .{ .row = curr_pos.row + 1, .col = curr_pos.col };
        },
        .left => pos: {
            if (curr_pos.col == 0) return null;
            break :pos .{ .row = curr_pos.row, .col = curr_pos.col - 1 };
        },
        .right => pos: {
            if (curr_pos.col >= n_cols - 1) return null;
            break :pos .{ .row = curr_pos.row, .col = curr_pos.col + 1 };
        },
    };

    if (isGradualUphill(curr_pos, next_pos, grid)) {
        return next_pos;
    } else {
        return null;
    }
}

fn countDistinctPaths(curr_pos: Position, grid: *const Grid) u64 {
    if (grid.items[curr_pos.row].items[curr_pos.col] == 9) {
        return 1;
    }
    var n_paths: u64 = 0;
    // just recursively try each direction
    inline for (@typeInfo(Direction).@"enum".fields) |dir| {
        if (nextGoodPos(curr_pos, @enumFromInt(dir.value), grid)) |next_pos| {
            n_paths += countDistinctPaths(next_pos, grid);
        }
    }

    return n_paths;
}

fn countGoodPaths(curr_pos: Position, grid: *const Grid, found_ends: *std.AutoHashMap(Position, void)) !u64 {
    if (grid.items[curr_pos.row].items[curr_pos.col] == 9) {
        if (found_ends.contains(curr_pos)) {
            return 0;
        } else {
            try found_ends.put(curr_pos, {});
            return 1;
        }
    }
    var good_paths: u64 = 0;
    // just recursively try each direction
    inline for (@typeInfo(Direction).@"enum".fields) |dir| {
        if (nextGoodPos(curr_pos, @enumFromInt(dir.value), grid)) |next_pos| {
            good_paths += try countGoodPaths(next_pos, grid, found_ends);
        }
    }

    return good_paths;
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

    var lines = std.mem.splitScalar(u8, input, '\n');
    const n_rows = std.mem.count(u8, input, "\n");
    const n_cols = std.mem.indexOfScalar(u8, input, '\n').?;

    var grid = try Grid.initCapacity(allocator, n_rows);
    defer {
        for (grid.items) |r| {
            r.deinit();
        }
        grid.deinit();
    }
    var trailheads = std.ArrayList(Position).init(allocator);
    defer trailheads.deinit();

    var row: usize = 0;
    while (lines.next()) |line| : (row += 1) {
        if (line.len == 0) break;
        var curr_row = try std.ArrayList(u8).initCapacity(allocator, n_cols);
        for (line, 0..) |c, col| {
            const height = c - '0';
            try curr_row.append(height);
            if (height == 0) {
                try trailheads.append(.{ .row = row, .col = col });
            }
        }
        try grid.append(curr_row);
    }

    // PART 1:
    var good_paths: u64 = 0;
    var found_ends = std.AutoHashMap(Position, void).init(allocator);
    defer found_ends.deinit();
    for (trailheads.items) |trailhead| {
        good_paths += try countGoodPaths(trailhead, &grid, &found_ends);
        found_ends.clearRetainingCapacity();
    }
    std.debug.print("{} good paths\n", .{good_paths});

    // PART 2:
    var num_paths: u64 = 0;
    for (trailheads.items) |trailhead| {
        num_paths += countDistinctPaths(trailhead, &grid);
    }

    std.debug.print("{} distinct paths\n", .{num_paths});
}
