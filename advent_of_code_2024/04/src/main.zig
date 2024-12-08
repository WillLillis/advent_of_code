const std = @import("std");

const WordGrid = std.ArrayList(std.ArrayList(u8));

const Offset = struct {
    row: i32,
    col: i32,

    fn scale(self: Offset, factor: i32) Offset {
        return .{ .row = self.row * factor, .col = self.col * factor };
    }
};

const Direction = enum {
    north,
    north_east,
    east,
    south_east,
    south,
    south_west,
    west,
    north_west,

    fn toOffset(self: Direction) Offset {
        return switch (self) {
            .north => .{ .row = -1, .col = 0 },
            .north_east => .{ .row = -1, .col = 1 },
            .east => .{ .row = 0, .col = 1 },
            .south_east => .{ .row = 1, .col = 1 },
            .south => .{ .row = 1, .col = 0 },
            .south_west => .{ .row = 1, .col = -1 },
            .west => .{ .row = 0, .col = -1 },
            .north_west => .{ .row = -1, .col = -1 },
        };
    }
};

fn get_char(
    grid: *const WordGrid,
    row: usize,
    col: usize,
    dir: Direction,
    n: usize,
) ?u8 {
    const offset = dir.toOffset().scale(@as(i32, @intCast(n)));
    const target_row = @as(i32, @intCast(row)) + offset.row;
    const target_col = @as(i32, @intCast(col)) + offset.col;
    if (target_row < 0) return null;
    if (target_row >= grid.items.len) return null;
    if (target_col < 0) return null;
    if (target_col >= grid.items[row].items.len) return null;

    return grid.items[@as(usize, @intCast(target_row))].items[@as(usize, @intCast(target_col))];
}

fn is_valid_xmas(expected_x: u8, expected_m: ?u8, expected_a: ?u8, expected_s: ?u8) bool {
    if (expected_m != null and expected_a != null and expected_s != null) {
        if (expected_x == 'X' and expected_m.? == 'M' and expected_a.? == 'A' and expected_s.? == 'S') {
            return true;
        }
    }
    return false;
}

fn is_valid_cross(top_left: ?u8, top_right: ?u8, bottom_left: ?u8, bottom_right: ?u8, center: u8) bool {
    if (center != 'A') return false;
    if (top_left != null and top_right != null and bottom_left != null and bottom_right != null) {
        if (!((top_left.? == 'M' and bottom_right.? == 'S') or (top_left.? == 'S' and bottom_right.? == 'M'))) return false;
        if (!((top_right.? == 'M' and bottom_left.? == 'S') or (top_right.? == 'S' and bottom_left.? == 'M'))) return false;
        return true;
    }
    return false;
}

fn count_words(grid: *const WordGrid, row: usize, col: usize) u64 {
    // NOTE: Part 1:
    // var word_count: u64 = 0;
    // inline for (@typeInfo(Direction).@"enum".fields) |dir| {
    //     const expected_x = grid.items[row].items[col];
    //     const expected_m = get_char(grid, row, col, @enumFromInt(dir.value), 1);
    //     const expected_a = get_char(grid, row, col, @enumFromInt(dir.value), 2);
    //     const expected_s = get_char(grid, row, col, @enumFromInt(dir.value), 3);
    //     // std.debug.print("{s}: x: {c}, m: {?c}. a: {?c}, s: {?c}\n", .{
    //     //     dir.name,
    //     //     expected_x,
    //     //     expected_m,
    //     //     expected_a,
    //     //     expected_s,
    //     // });
    //
    //     // std.debug.print("Found one: ({}, {}) -- {s}\n", .{row, col, dir.name});
    //     if (is_valid_xmas(expected_x, expected_m, expected_a, expected_s)) word_count += 1;
    // }
    
    // NOTE: Part 2:
    const center = grid.items[row].items[col];
    const top_left = get_char(grid, row, col, .north_west, 1);
    const top_right = get_char(grid, row, col, .north_east, 1);
    const bottom_left = get_char(grid, row, col, .south_west, 1);
    const bottom_right = get_char(grid, row, col, .south_east, 1);
    if (is_valid_cross(top_left, top_right, bottom_left, bottom_right, center)) {
        return 1;
    } else {
        return 0;
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
    const line_count = std.mem.count(u8, input, "\n");
    std.debug.assert(line_count > 0);
    // std.debug.print("{s}\n", .{input});

    // Create a grid
    var grid = try std.ArrayList(std.ArrayList(u8)).initCapacity(allocator, line_count);
    defer grid.deinit();
    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) break;
        var curr_line = try std.ArrayList(u8).initCapacity(allocator, line.len);
        curr_line.appendSliceAssumeCapacity(line);
        grid.appendAssumeCapacity(curr_line);
    }
    defer {
        for (grid.items) |line| {
            allocator.free(line.items);
        }
    }

    // std.debug.print("   0123456789\n", .{});
    // for (grid.items, 0..) |line, i| {
    //     std.debug.print("{}: {s}\n", .{ i, line.items });
    // }

    var word_count: u64 = 0;
    for (0..grid.items.len) |i| {
        for (0..grid.items[i].items.len) |j| {
            word_count += count_words(&grid, i, j);
        }
    }
    std.debug.print("Count: {}\n", .{word_count});
}
