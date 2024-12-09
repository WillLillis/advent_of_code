const std = @import("std");

const MoveInfo = struct {
    idx: usize,
    len: usize,
};

fn next_free_idx(fs: *const std.ArrayList(?usize), start_pos: usize) MoveInfo {
    const empty_idx: usize = std.mem.indexOfScalarPos(?usize, fs.items, start_pos, null).?;
    const empty_len = blk: {
        var i = empty_idx;
        while (i < fs.items.len and fs.items[i] == null) : (i += 1) {}
        break :blk i - empty_idx;
    };

    return .{ .idx = empty_idx, .len = empty_len };
}

fn next_moved_idx(fs: *const std.ArrayList(?usize), file_id: usize) MoveInfo {
    const move_idx = std.mem.indexOfScalar(?usize, fs.items, file_id).?;
    var i = move_idx;
    while (i < fs.items.len) : (i += 1) {
        if (fs.items[i] != fs.items[move_idx]) {
            break;
        }
    }
    const move_len = i - move_idx;

    // std.debug.print("id: {}, idx: {}, len: {}\n", .{ file_id, move_idx, move_len });
    return .{ .idx = move_idx, .len = move_len };
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

    var expanded_fs = std.ArrayList(?usize).init(allocator);
    defer expanded_fs.deinit();
    var file_id: usize = 0;
    var is_file = true;
    for (input) |c| {
        if (c == '\n') break;
        std.debug.assert(c >= '0' and c <= '9');
        const n_blocks = c - '0';
        for (0..n_blocks) |_| {
            if (is_file) {
                try expanded_fs.append(file_id);
            } else {
                try expanded_fs.append(null);
            }
        }

        if (is_file) file_id += 1;
        is_file = !is_file;
    }

    // std.debug.print("{any}\n", .{expanded_fs.items});

    // PART 1:
    // var empty_idx: usize = std.mem.indexOfScalar(?usize, expanded_fs.items, null).?;
    // var to_move_idx: usize = expanded_fs.items.len - 1;
    // while (to_move_idx >= 0 and to_move_idx > empty_idx) : (to_move_idx -= 1) {
    //     const moved_val = expanded_fs.items[to_move_idx];
    //     expanded_fs.items[to_move_idx] = null;
    //     expanded_fs.items[empty_idx] = moved_val;
    //
    //     empty_idx = std.mem.indexOfScalar(?usize, expanded_fs.items, null).?;
    // }

    // PART 2:
    var move_id = expanded_fs.items[expanded_fs.items.len - 1].?;
    var move_info = next_moved_idx(&expanded_fs, move_id);
    while (move_id >= 0) {
        var empty_info = next_free_idx(&expanded_fs, 0);
        while (move_info.idx > empty_info.idx) {
            if (move_info.len <= empty_info.len) {
                const move_val = expanded_fs.items[move_info.idx];
                for (0..move_info.len) |i| {
                    expanded_fs.items[move_info.idx + i] = null;
                    expanded_fs.items[empty_info.idx + i] = move_val;
                }
            }
            empty_info = next_free_idx(&expanded_fs, empty_info.idx + 1);
        }
        if (move_id == 0) break;
        move_id -= 1;
        move_info = next_moved_idx(&expanded_fs, move_id);
    }

    // std.debug.print("{any}\n", .{expanded_fs.items});
    var sum: u128 = 0;
    for (expanded_fs.items, 0..) |entry, i| {
        if (entry) |id| {
            sum += i * id;
        }
    }
    std.debug.print("Sum: {}\n", .{sum});
}
