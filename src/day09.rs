use itertools::Itertools;
use std::collections::HashMap;

/// Compresses coordinates and returns unique axes plus compressed indices.
fn coordinate_compress(
    points: &[(i128, i128)],
) -> (
    Vec<i128>,
    Vec<i128>,
    Vec<(usize, usize)>,
    HashMap<i128, usize>,
    HashMap<i128, usize>,
) {
    let mut xs: Vec<i128> = points
        .iter()
        .flat_map(|p| [p.0, p.0 + 1])
        .collect();
    xs.sort();
    xs.dedup();

    let mut ys: Vec<i128> = points
        .iter()
        .flat_map(|p| [p.1, p.1 + 1])
        .collect();
    ys.sort();
    ys.dedup();

    let x_index: HashMap<i128, usize> =
        xs.iter().enumerate().map(|(idx, &val)| (val, idx)).collect();
    let y_index: HashMap<i128, usize> =
        ys.iter().enumerate().map(|(idx, &val)| (val, idx)).collect();

    let compressed = points
        .iter()
        .map(|&(x, y)| (*x_index.get(&x).unwrap(), *y_index.get(&y).unwrap()))
        .collect();

    (xs, ys, compressed, x_index, y_index)
}

/// Builds a boolean grid (by compressed cells) of tiles that are red or green.
fn build_allowed_mask(
    points: &[(i128, i128)],
    xs: &[i128],
    ys: &[i128],
) -> Vec<Vec<bool>> {
    if xs.len() < 2 || ys.len() < 2 {
        return vec![];
    }

    let width = xs.len() - 1;
    let height = ys.len() - 1;
    let mut mask = vec![vec![false; width]; height];

    let mut vertical_edges: Vec<(f64, f64, f64)> = Vec::new();
    for idx in 0..points.len() {
        let (x1, y1) = points[idx];
        let (x2, y2) = points[(idx + 1) % points.len()];
        if x1 == x2 && y1 != y2 {
            let y_min = y1.min(y2) as f64;
            let y_max = y1.max(y2) as f64;
            vertical_edges.push((x1 as f64, y_min, y_max));
        }
    }

    for row in 0..height {
        let sample_y = (ys[row] as f64 + ys[row + 1] as f64) / 2.0;
        let mut crossings: Vec<f64> = vertical_edges
            .iter()
            .filter_map(|(x, y_min, y_max)| {
                if sample_y > *y_min && sample_y < *y_max {
                    Some(*x)
                } else {
                    None
                }
            })
            .collect();
        crossings.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut idx = 0;
        while idx + 1 < crossings.len() {
            let start = crossings[idx];
            let end = crossings[idx + 1];
            for col in 0..width {
                let sample_x = (xs[col] as f64 + xs[col + 1] as f64) / 2.0;
                if sample_x >= start && sample_x <= end {
                    mask[row][col] = true;
                }
            }
            idx += 2;
        }
    }

    mask
}

fn mark_boundary_tiles(
    mask: &mut [Vec<bool>],
    points: &[(i128, i128)],
    x_index: &HashMap<i128, usize>,
    y_index: &HashMap<i128, usize>,
) {
    if mask.is_empty() || mask[0].is_empty() {
        return;
    }

    let len = points.len();
    for idx in 0..len {
        let (x1, y1) = points[idx];
        let (x2, y2) = points[(idx + 1) % len];
        if x1 == x2 {
            let col = *x_index.get(&x1).unwrap();
            let y_start = y1.min(y2);
            let y_end = y1.max(y2);
            let row_start = *y_index.get(&y_start).unwrap();
            let row_end = *y_index.get(&(y_end + 1)).unwrap();
            for row in row_start..row_end {
                mask[row][col] = true;
            }
        } else if y1 == y2 {
            let row = *y_index.get(&y1).unwrap();
            let x_start = x1.min(x2);
            let x_end = x1.max(x2);
            let col_start = *x_index.get(&x_start).unwrap();
            let col_end = *x_index.get(&(x_end + 1)).unwrap();
            for col in col_start..col_end {
                mask[row][col] = true;
            }
        }
    }
}

fn build_allowed_prefix(mask: &[Vec<bool>], xs: &[i128], ys: &[i128]) -> Vec<Vec<i128>> {
    if xs.len() < 2 || ys.len() < 2 {
        return vec![vec![0]];
    }

    let height = ys.len() - 1;
    let width = xs.len() - 1;
    let mut prefix = vec![vec![0i128; width + 1]; height + 1];

    for row in 0..height {
        for col in 0..width {
            let cell_area = if mask[row][col] {
                (ys[row + 1] - ys[row]) * (xs[col + 1] - xs[col])
            } else {
                0
            };

            prefix[row + 1][col + 1] = cell_area
                + prefix[row][col + 1]
                + prefix[row + 1][col]
                - prefix[row][col];
        }
    }

    prefix
}

fn area_in_region(
    prefix: &[Vec<i128>],
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
) -> i128 {
    prefix[row_end][col_end]
        - prefix[row_start][col_end]
        - prefix[row_end][col_start]
        + prefix[row_start][col_start]
}

pub fn solve(input: String) {
    let mut result: i128 = 0;
    let mut result_part2: i128 = 0;

    let mut tiles: Vec<(i128, i128)> = vec![];
    input.lines().for_each(|line| {
        let (x, y) = line
            .split(',')
            .map(|x| x.parse::<i128>().unwrap())
            .collect_tuple()
            .unwrap();
        tiles.push((x, y));
    });

    for tile in &tiles {
        println!("Tile: {:?}", tile);
    }

    let (
        compressed_xs,
        compressed_ys,
        compressed_points,
        x_index,
        y_index,
    ) = coordinate_compress(&tiles);
    println!("Compressed xs: {:?}", compressed_xs);
    println!("Compressed ys: {:?}", compressed_ys);
    println!("Compressed points: {:?}", compressed_points);

    let mut allowed_mask = build_allowed_mask(&tiles, &compressed_xs, &compressed_ys);
    mark_boundary_tiles(&mut allowed_mask, &tiles, &x_index, &y_index);
    println!("Allowed mask (rows={}):", allowed_mask.len());
    // for row in &allowed_mask {
    //     println!("{:?}", row);
    // }

    let allowed_prefix = build_allowed_prefix(&allowed_mask, &compressed_xs, &compressed_ys);

    let mut max_area = 0;

    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let area =
                ((tiles[i].0 - tiles[j].0 + 1).abs() * (tiles[i].1 - tiles[j].1 + 1).abs()) as i128;
            println!("Area between {:?} and {:?} is {}", tiles[i], tiles[j], area);

            if area > max_area {
                max_area = area;
            }
        }
    }
    result = max_area;

    let mut max_area_part2 = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            let x_min = x1.min(x2);
            let x_max = x1.max(x2);
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);

            let col_start = *x_index.get(&x_min).unwrap();
            let col_end = *x_index.get(&(x_max + 1)).unwrap();
            let row_start = *y_index.get(&y_min).unwrap();
            let row_end = *y_index.get(&(y_max + 1)).unwrap();

            let allowed_area = area_in_region(&allowed_prefix, row_start, row_end, col_start, col_end);
            let total_area = (compressed_xs[col_end] - compressed_xs[col_start])
                * (compressed_ys[row_end] - compressed_ys[row_start]);

            if allowed_area == total_area {
                let width = (x1 - x2).abs() + 1;
                let height = (y1 - y2).abs() + 1;
                let area = width * height;
                if area > max_area_part2 {
                    max_area_part2 = area;
                }
            }
        }
    }

    result_part2 = max_area_part2;

    println!("*******************");
    println!("Solved Part 1: {}", result);
    println!("Solved Part 2: {}", result_part2);
    println!("*******************");
}
