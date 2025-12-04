// fn print_grid(grid: &Vec<Vec<char>>) {
//     for row in grid {
//         let line: String = row.iter().collect();
//         println!("{}", line);
//     }
// }

fn get_adjacent_rolls(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut rolls = 0;
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in directions {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x >= 0
            && new_x < grid.len() as i32
            && new_y >= 0
            && new_y < grid[0].len() as i32
            && grid[new_x as usize][new_y as usize] == '@'
        {
            rolls += 1;
        }
    }

    rolls
}

fn remove_rolls(grid: &Vec<Vec<char>>) -> (Vec<Vec<char>>, i32) {
    let mut new_grid = grid.clone();
    let mut removed_count = 0;
    grid.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, &cell)| {
            if cell == '@' {
                let rolls = get_adjacent_rolls(&grid, x, y);
                if rolls < 4 {
                    new_grid[x][y] = '.';
                    removed_count += 1;
                }
            }
        });
    });

    (new_grid, removed_count)
}
pub fn solve(input: String) {
    let result;
    let mut result_part2;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (mut new_grid, mut removed_count) = remove_rolls(&grid);
    result = removed_count;
    result_part2 = removed_count;
    while removed_count != 0 {
        (new_grid, removed_count) = remove_rolls(&new_grid);
        result_part2 += removed_count;
    }

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}
