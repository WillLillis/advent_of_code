//! By convention, main.zig is where your main function lives in the case that
//! you are building an executable. If you are making a library, the convention
//! is to delete this file and start with root.zig instead.
const std = @import("std");
const Order = std.math.Order;

fn check_plan(plan: []const u8, skip_idx: ?usize) !bool {
    var num_str_iter = std.mem.splitScalar(u8, plan, ' ');
    var prev_val: ?u64 = null;
    var prev_dir: ?Order = null;
    var idx: usize = 0;
    while (num_str_iter.next()) |num| : (idx += 1) {
        if (skip_idx) |skip| {
            if (skip == idx) continue;
        }
        const curr_num = try std.fmt.parseUnsigned(u64, num, 10);
        if (prev_val) |prev_num| {
            const diff = @abs(@as(i64, @intCast(curr_num)) - @as(i64, @intCast(prev_num)));
            if (diff > 3) return false;

            const dir = std.math.order(curr_num, prev_num);
            switch (dir) {
                .eq => return false,
                .gt, .lt => {
                    if (prev_dir) |last_dir| {
                        if (last_dir != dir) return false;
                    }
                },
            }
            prev_dir = dir;
        }
        prev_val = curr_num;
    }

    return true;
}

fn is_valid_plan(plan: []const u8) !bool {
    // first try without removing a number
    if (try check_plan(plan, null)) return true;

    // then remove each number in turn until it works, or we exhausts all indexes
    const plan_len = std.mem.count(u8, plan, " ") + 1;
    for (0..plan_len) |i| {
        if (try check_plan(plan, i)) return true;
    }

    return false;
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
    const raw_input = try input_file.readToEndAlloc(allocator, 1000000);
    defer allocator.free(raw_input);
    var lines = std.mem.splitScalar(u8, raw_input, '\n');
    var safe_reports: u32 = 0;
    while (lines.next()) |line| {
        if (line.len == 0) break;
        // std.debug.print("Line: {s} -- ", .{line});
        if (try is_valid_plan(line)) {
            safe_reports += 1;
        }

        // var num_str_iter = std.mem.splitScalar(u8, line, ' ');
        // var did_dampen = false;
        // var is_safe = true;
        // var prev_val: ?u64 = null;
        // var prev_dir: ?Order = null;
        // while (num_str_iter.next()) |num| {
        //     const curr_num = try std.fmt.parseUnsigned(u64, num, 10);
        //     if (prev_val) |prev_num| {
        //         const diff = @abs(@as(i64, @intCast(curr_num)) - @as(i64, @intCast(prev_num)));
        //         if (diff > 3) {
        //             if (did_dampen) {
        //                 std.debug.print("(DELTA {}, {})   ", .{ prev_num, curr_num });
        //                 is_safe = false;
        //                 break;
        //             } else {
        //                 did_dampen = true;
        //                 continue;
        //             }
        //         }
        //
        //         const dir = std.math.order(curr_num, prev_num);
        //         switch (dir) {
        //             .eq => {
        //                 if (did_dampen) {
        //                     std.debug.print("(EQ)      ", .{});
        //                     is_safe = false;
        //                     break;
        //                 } else {
        //                     did_dampen = true;
        //                     continue;
        //                 }
        //             },
        //             .gt, .lt => {
        //                 if (prev_dir) |last_dir| {
        //                     if (last_dir != dir) {
        //                         if (did_dampen) {
        //                             std.debug.print("(REVERSE) ", .{});
        //                             is_safe = false;
        //                             break;
        //                         } else {
        //                             did_dampen = true;
        //                             continue;
        //                         }
        //                     }
        //                 }
        //             },
        //         }
        //         prev_dir = dir;
        //     }
        //     prev_val = curr_num;
        // }
        // if (is_safe) safe_reports += 1;
        // if (is_safe) {
        //     // std.debug.print("SAFE\n", .{});
        // } else {
        //     std.debug.print("{s} -- UNSAFE\n", .{line});
        // }
    }

    std.debug.print("{} safe reports\n", .{safe_reports});
}
