fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut grid: Vec<Vec<i8>> = vec![vec![0; size]; size];
    let center = match size {
        size if size % 2 == 0 => size / 2 - 1,
        _ => size / 2,
    };

    // Walk the diagonal of the grid, from top left corner,
    // to the center. This means we can solve the problem on
    // about a number of steps about half of the size of the grid
    for layer in 0..=center {
        let fill = match layer {
            layer if layer % 2 == 0 => 1,
            _ => 0,
        };
        // do a round on a square grid starting at (layer, layer)
        for index in layer..(size - layer) {
            // upper edge
            grid[layer][index] = fill;
            // right edge
            grid[index][size - layer - 1] = fill;
            // lower edge
            grid[size - layer - 1][index] = fill;
            // left edge
            grid[index][layer] = fill;
        }

        // unite with previous layer
        grid[layer + 1][layer] = match fill {
            1 => 0,
            _ => {
                if layer == center {
                    continue;
                }
                1
            }
        };

        pretry_print(&grid);
    }

    return grid;
}

fn pretry_print(grid: &Vec<Vec<i8>>) -> () {
    for row in grid {
        println!(
            "{}",
            row.iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

fn main() {
    let size: usize = 5;
    println!("Spiral or order: {} \n {:?}", size, spiralize(size));
}

#[test]
fn test5() {
    assert_eq!(
        spiralize(5),
        [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ],
    );
}

#[test]
fn test6() {
    assert_eq!(
        spiralize(6),
        [
            [1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 0, 1],
            [1, 0, 0, 1, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1],
        ],
    );
}

#[test]
fn test8() {
    assert_eq!(
        spiralize(8),
        [
            [1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 1, 0, 1],
            [1, 0, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
        ],
    );
}
