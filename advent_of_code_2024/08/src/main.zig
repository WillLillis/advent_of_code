const std = @import("std");

const Position = struct {
    row: isize,
    col: isize,
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

    const n_rows = std.mem.count(u8, input, "\n");
    const n_cols = std.mem.indexOfScalar(u8, input, '\n').?;
    var sat_locs = std.AutoHashMap(u8, std.ArrayList(Position)).init(allocator);
    var sat_types = std.ArrayList(u8).init(allocator);
    var anti_nodes = std.AutoHashMap(Position, void).init(allocator);
    defer {
        var locs = sat_locs.iterator();
        while (locs.next()) |*loc| {
            loc.value_ptr.deinit();
        }
        sat_locs.deinit();
        sat_types.deinit();
        anti_nodes.deinit();
    }
    var row: usize = 0;
    while (lines.next()) |line| {
        if (line.len == 0) break;
        defer row += 1;
        for (0..line.len) |col| {
            const curr_item = line[col];
            switch (curr_item) {
                '.' => {},
                else => {
                    const curr_loc = Position{ .row = @as(isize, @intCast(row)), .col = @as(isize, @intCast(col)) };
                    try anti_nodes.put(curr_loc, {});
                    if (sat_locs.getPtr(curr_item)) |loc_list| {
                        try loc_list.append(curr_loc);
                        try sat_types.append(curr_item);
                    } else {
                        var loc_list = try std.ArrayList(Position).initCapacity(allocator, 1);
                        try loc_list.append(curr_loc);
                        try sat_locs.put(curr_item, loc_list);
                    }
                },
            }
        }
    }

    // just iterate through all the pairs between sats of the same frequency...
    for (sat_types.items) |sat_type| {
        const locs = sat_locs.getPtr(sat_type).?;
        for (0..locs.items.len) |i| {
            const sat_1 = locs.items[i];
            for (i + 1..locs.items.len) |j| {
                const sat_2 = locs.items[j];
                const row_delta = sat_1.row - sat_2.row;
                const col_delta = sat_1.col - sat_2.col;
                var row_jump = row_delta;
                var col_jump = col_delta;
                while (true) {
                    defer row_jump += row_delta;
                    defer col_jump += col_delta;
                    var still_valid = false;

                    const anti_node_1: Position = .{ .row = sat_1.row + row_jump, .col = sat_1.col + col_jump };
                    const anti_node_2: Position = .{ .row = sat_2.row - row_jump, .col = sat_2.col - col_jump };

                    if (anti_node_1.row >= 0 and anti_node_1.row < n_rows and anti_node_1.col >= 0 and anti_node_1.col < n_cols) {
                        // std.debug.print("Adding {any}\n", .{anti_node_1});
                        still_valid = true;
                        try anti_nodes.put(anti_node_1, {});
                    }
                    if (anti_node_2.row >= 0 and anti_node_2.row < n_rows and anti_node_2.col >= 0 and anti_node_2.col < n_cols) {
                        // std.debug.print("Adding {any}\n", .{anti_node_2});
                        still_valid = true;
                        try anti_nodes.put(anti_node_2, {});
                    }
                    if (!still_valid) break;
                }
            }
        }
    }

    std.debug.print("{} antinodes\n", .{anti_nodes.count()});
    // for (0..n_rows) |i| {
    //     for (0..n_cols) |j| {
    //         if (anti_nodes.contains(.{ .row = @intCast(i), .col = @intCast(j) })) {
    //             std.debug.print("#", .{});
    //         } else {
    //             std.debug.print(".", .{});
    //         }
    //     }
    //     std.debug.print("\n", .{});
    // }
}
