const std = @import("std");

pub fn main() !void {
    // Just take the ooga booga approach and scan through the list in O(n^2)
    const file_name = "input.txt";
    var file = std.fs.cwd();
    const input_file = try file.openFile(file_name, .{});
    defer input_file.close();
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    // Lol
    // defer {
    //     const deinit_status = gpa.deinit();
    //     if (deinit_status == .leak) @panic("MEM LEAK");
    // }
    const input = try input_file.readToEndAlloc(allocator, 1000000);
    defer allocator.free(input);
    var lines = std.mem.splitScalar(u8, input, '\n');

    var rules = std.AutoHashMap(u32, std.ArrayList(u32)).init(allocator);

    // Read in rules, store as array lists in a hashmap
    while (lines.next()) |line| {
        if (line.len == 0) break;
        var nums = std.mem.splitScalar(u8, line, '|');

        const num1 = try std.fmt.parseUnsigned(u32, nums.next().?, 10);
        const num2 = try std.fmt.parseUnsigned(u32, nums.next().?, 10);
        std.debug.assert(nums.next() == null);
        if (rules.getPtr(num1)) |vals| {
            try vals.append(num2);
        } else {
            var rule = try std.ArrayList(u32).initCapacity(allocator, 1);
            rule.appendAssumeCapacity(num2);
            try rules.put(num1, rule);
        }
    }

    // var good_count: u32 = 0;
    var nums = std.ArrayList(u32).init(allocator);

    // Part 1 is kinda nice
    // // Iterate over the page lists
    // outer: while (lines.next()) |line| {
    //     if (line.len == 0) break;
    //     defer nums.clearRetainingCapacity();
    //
    //     var line_nums = std.mem.splitScalar(u8, line, ',');
    //     while (line_nums.next()) |num| {
    //         try nums.append(try std.fmt.parseUnsigned(u8, num, 10));
    //     }
    //
    //     // for each index...
    //     for (1..nums.items.len) |preceding_idx| {
    //         if (rules.getPtr(nums.items[preceding_idx])) |rule| {
    //             // ...check all the numbers that came before it
    //             for (0..preceding_idx) |check_idx| {
    //                 if (std.mem.indexOfScalar(u32, rule.items, nums.items[check_idx])) |_| {
    //                     // broke the rule
    //                     continue :outer;
    //                 }
    //             }
    //         }
    //     }
    //     // all rules followed
    //     good_count += nums.items[nums.items.len / 2]; // Add the middle page number
    // }
    // Part 2 is ugly:
    // Iterate over the page lists
    var bad_lists = std.ArrayList(std.ArrayList(u32)).init(allocator);
    outer: while (lines.next()) |line| {
        if (line.len == 0) break;
        defer nums.clearRetainingCapacity();

        var line_nums = std.mem.splitScalar(u8, line, ',');
        while (line_nums.next()) |num| {
            try nums.append(try std.fmt.parseUnsigned(u8, num, 10));
        }

        // for each index...
        for (1..nums.items.len) |preceding_idx| {
            if (rules.getPtr(nums.items[preceding_idx])) |rule| {
                // ...check all the numbers that came before it
                for (0..preceding_idx) |check_idx| {
                    if (std.mem.indexOfScalar(u32, rule.items, nums.items[check_idx])) |_| {
                        // broke the rule
                        try bad_lists.append(try nums.clone());
                        continue :outer;
                    }
                }
            }
        }
        // all rules followed
    }

    // std.debug.print("Bad count: {}\n", .{bad_lists.items.len});

    var middle_count: u32 = 0;
    // ooga booga approoach, just fix things one at a time until it passes
    for (bad_lists.items, 0..) |*bad_nums, i| {
        _ = i;
        // std.debug.print("{}: {any}\n", .{i, bad_nums.items});
        var verified = false;
        check: while (!verified) {
            // for each index...
            for (1..bad_nums.items.len) |preceding_idx| {
                if (rules.getPtr(bad_nums.items[preceding_idx])) |rule| {
                    // ...check all the numbers that came before it
                    for (0..preceding_idx) |check_idx| {
                        if (std.mem.indexOfScalar(u32, rule.items, bad_nums.items[check_idx])) |_| {
                            // broke the rule
                            const popped = bad_nums.orderedRemove(check_idx);
                            // std.debug.print("popped: {}\n", .{popped});
                            try bad_nums.insert(preceding_idx, popped);
                            continue :check;
                        }
                    }
                }
            }
            verified = true;
            middle_count += bad_nums.items[bad_nums.items.len / 2]; // Add the middle page number
        }
    }

    // for (bad_lists.items) |bad_nums| {
    //     std.debug.print("{any}\n", .{bad_nums.items});
    // }
    // std.debug.print("Bad middle count: {}\n", .{middle_count});

    // std.debug.print("Good count: {}\n", .{good_count});
}
