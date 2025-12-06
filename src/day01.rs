fn rotate_dial_part2(current_pos: i32, direction: i32, clicks: i32) -> (i32, bool) {
    let mut new_pos = current_pos;
    let mut did_click_at_0 = false;
    if direction == -1 {
        new_pos -= clicks;
        if new_pos < 0 {
            did_click_at_0 = true;
            new_pos = 100 - (-1 * new_pos);
        }
    } else {
        new_pos += clicks;
        if new_pos > 99 {
            did_click_at_0 = true;
            new_pos = new_pos - 100;
        }
    }

    if new_pos == 0 || current_pos == 0 {
        did_click_at_0 = false;
    }
    (new_pos, did_click_at_0)
}

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;
    let mut current_position = 50;

    input.lines().for_each(|line| {
        let direction = if &line[0..1] == "L" { -1 } else { 1 };
        let mut clicks = line[1..].parse::<i32>().unwrap();

        if clicks > 99 {
            result_part2 += clicks / 100;
        }

        clicks = clicks % 100;
        let did_click_at_0;

        (current_position, did_click_at_0) = rotate_dial_part2(current_position, direction, clicks);
        if current_position == 0 {
            result += 1;
        }

        if did_click_at_0 || current_position == 0 {
            result_part2 += 1;
        }
    });

    println!("*******************");
    println!("Solved Part 1: {}", result);
    println!("Solved Part 2: {}", result_part2);
    println!("*******************");
}
