use std::collections::HashMap;

pub fn parse(input: &str) -> (u64, u64) {
    // parse coordinates as (x, y) in the order they appear
    let mut coords = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<u64>().unwrap();
        let y = parts.next().unwrap().parse::<u64>().unwrap();
        coords.push((x, y));
    }

    let mut max_area_p1 = 0;

    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];

            // discrete "point-count" area: inclusive endpoints
            let dx = x1.abs_diff(x2) + 1;
            let dy = y1.abs_diff(y2) + 1;
            let area = dx * dy;

            max_area_p1 = max_area_p1.max(area);
        }
    }

    let max_area_p2 = largest_rectangle_inside_pairwise(&coords);

    (max_area_p1, max_area_p2)
}

pub fn part1(input: &(u64, u64)) -> u64 {
    input.0
}

pub fn part2(input: &(u64, u64)) -> u64 {
    input.1
}

fn largest_rectangle_inside_pairwise(coords: &[(u64, u64)]) -> u64 {
    if coords.len() < 2 {
        return 0;
    }

    // 1) Coordinate compression: collect all distinct x and y from polygon vertices
    let mut xs: Vec<u64> = coords.iter().map(|&(x, _)| x).collect();
    let mut ys: Vec<u64> = coords.iter().map(|&(_, y)| y).collect();

    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    if xs.len() < 2 || ys.len() < 2 {
        return 0;
    }

    let w = xs.len() - 1; // number of compressed columns
    let h = ys.len() - 1; // number of compressed rows

    // 2) Build "inside" grid: 1 if the cell's center is inside the polygon, 0 otherwise
    // For example, the test compressed grid is 3x3, and has "inside" values of:
    //     [0, 1, 1],
    //     [1, 1, 1],
    //     [0, 0, 1],
    // (this is y=0 at top, x=0 at left), and represents the overall shape of the polygon
    let mut inside = vec![vec![0u32; w]; h];

    for cy in 0..h {
        let py = (ys[cy] as f64 + ys[cy + 1] as f64) * 0.5;
        for cx in 0..w {
            let px = (xs[cx] as f64 + xs[cx + 1] as f64) * 0.5;
            if point_in_polygon(px, py, coords) {
                inside[cy][cx] = 1;
            }
        }
    }

    // 3) Build 2D prefix sums over "inside" to allow O(1) sum queries over any sub-rectangle.
    //
    // prefix[y][x] = sum of inside[0..y, 0..x] (half-open)
    let mut prefix = vec![vec![0u32; w + 1]; h + 1];
    for y in 0..h {
        let mut row_sum = 0u32;
        for x in 0..w {
            row_sum += inside[y][x];
            prefix[y + 1][x + 1] = prefix[y][x + 1] + row_sum;
        }
    }

    // helper to get sum in [y0..y1) x [x0..x1) in cell indices
    // It uses simple fact that to find the smaller area size of a sub-rectangle, we can
    // take the full size, and subtract 2 rectangles from it, but add back in the
    // overlapping part that was subtracted twice.
    // . . x x
    // . . x x
    // d d . .
    // d d . .
    // the sum of the x's is sum of whole grid, subtract sum of first two columns, subtract sum of first two rows,
    // then add back in the overlapping part that was subtracted twice (the d's)
    let rect_sum = |y0: usize, y1: usize, x0: usize, x1: usize| -> u32 {
        let a = prefix[y1][x1] as i64;
        let b = prefix[y0][x1] as i64;
        let c = prefix[y1][x0] as i64;
        let d = prefix[y0][x0] as i64;
        (a - b - c + d) as u32
    };
    

    // maps from original coordinates to compressed indices
    let x_index: HashMap<u64, usize> = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_index: HashMap<u64, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    // 4) Iterate over all pairs, and keep the best rectangle whose interior is fully inside.
    //
    // For a pair (x1,y1), (x2,y2):
    //  - let minx <= maxx, miny <= maxy
    //  - it corresponds to cell ranges:
    //      cols  [ix_min .. ix_max)
    //      rows  [iy_min .. iy_max)
    //    where ix_xxx = index of that coordinate in xs, and similarly for ys.
    //  - total cells in that region = (ix_max - ix_min) * (iy_max - iy_min)
    //  - if rect_sum == total_cells, then all those cells are inside.
    //
    // We then use the same discrete area metric as part 1:
    //      width  = maxx - minx + 1
    //      height = maxy - miny + 1
    //      area   = width * height
    //
    // We ignore degenerate rectangles where maxx == minx or maxy == miny here; in practice
    // the maximal area will be a 2D rectangle, and part 1 already handles the "thin" cases.

    let mut best_area: u64 = 0;

    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];

            let (minx, maxx) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            let (miny, maxy) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

            // skip degenerate rectangles (purely horizontal or vertical) - optimization as the solution won't be a straight line.
            if minx == maxx || miny == maxy {
                continue;
            }

            let ix_min = match x_index.get(&minx) {
                Some(&idx) => idx,
                None => continue,
            };
            let ix_max = match x_index.get(&maxx) {
                Some(&idx) => idx,
                None => continue,
            };
            let iy_min = match y_index.get(&miny) {
                Some(&idx) => idx,
                None => continue,
            };
            let iy_max = match y_index.get(&maxy) {
                Some(&idx) => idx,
                None => continue,
            };

            if ix_max <= ix_min || iy_max <= iy_min {
                continue;
            }

            let cols = ix_max - ix_min;
            let rows = iy_max - iy_min;
            let total_cells = (cols * rows) as u32;

            let inside_cells = rect_sum(iy_min, iy_max, ix_min, ix_max);
            // Ensure that all the cells of the compressed grid are inside the polygon. If not, at least one is outside the polygon, so we skip this rectangle.
            if inside_cells != total_cells {
                continue;
            }

            // fully inside: compute the rectangle area and check if it's the largest so far
            let width = maxx - minx + 1;
            let height = maxy - miny + 1;
            let area = width * height;

            if area > best_area {
                best_area = area;
            }
        }
    }

    best_area
}

// Standard even-odd rule (ray casting) point-in-polygon test.
// Assumes polygon vertices are given in order
fn point_in_polygon(px: f64, py: f64, poly: &[(u64, u64)]) -> bool {
    let mut inside = false;
    let n = poly.len();
    if n < 3 {
        return false;
    }

    for i in 0..n {
        let (x1_u, y1_u) = poly[i];
        let (x2_u, y2_u) = poly[(i + 1) % n];

        let (x1, y1, x2, y2) = (
            x1_u as f64,
            y1_u as f64,
            x2_u as f64,
            y2_u as f64,
        );

        // Check if the horizontal ray at y = py crosses the edge (x1,y1) -> (x2,y2)
        let intersects = ((y1 > py) != (y2 > py))
            && (px < (x2 - x1) * (py - y1) / (y2 - y1 + 1e-12) + x1);

        if intersects {
            inside = !inside;
        }
    }

    inside
}
