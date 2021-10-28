use rand::prelude::*;

/*
seen:
0 if covered
1 if flagged
anything else is uncovered
*/

fn print_board(grid: &Vec<Vec<i32>>, seen: &Vec<Vec<u32>>, reveal_all: bool) {
    let ylen = grid[0].len();
    for x in 0..grid.len() {
        let mut line = String::from("| ");
        for y in 0..ylen {
            if seen[x][y]  == 0 && !reveal_all {
                line.push('?');
            } else if seen[x][y] == 1 && !reveal_all {
                line.push('!');
            } else if grid[x][y] == -1 {
                line.push('X');
            } else {
                line.push_str(&grid[x][y].to_string());
            }
            line.push(' ');
        }

        line.push('|');
        println!("{}", line);
    }
}

fn incr_counter(grid: &mut Vec<Vec<i32>>, xc: usize, yc:usize) {
    // First validate xc and yc
    // No need to check zero case, as it would overflow to be > grid.len()
    if xc >= grid.len() {
        return
    }

    if yc >= grid[xc].len() {
        return
    }

    // Do not update if the square is a bomb
    if grid[xc][yc] == -1 {
        return
    }

    grid[xc][yc] += 1;
}

fn place_bomb(grid: &mut Vec<Vec<i32>>, xc: usize, yc: usize) {
    // Place a bomb at (xc, yc)
    grid[xc][yc] = -1;

    // Update counters of nearby squares
    if xc > 0 {
        incr_counter(grid, xc - 1, yc);
        incr_counter(grid, xc - 1, yc + 1);
    }

    if xc > 0 && yc > 0 {
        incr_counter(grid, xc - 1, yc - 1);
    }

    if yc > 0 {
        incr_counter(grid, xc + 1, yc - 1);
        incr_counter(grid, xc, yc - 1);
    }

    incr_counter(grid, xc + 1, yc);
    incr_counter(grid, xc + 1, yc + 1);
    incr_counter(grid, xc, yc + 1);
}

fn gen_board(x: usize, y: usize, nbombs: i32) -> (Vec<Vec<i32>>, Vec<Vec<u32>>) {
    let mut grid = Vec::with_capacity(x);
    let mut seen = Vec::with_capacity(x);
    let mut rng = rand::thread_rng();
    for i in 0..x {
        grid.push(Vec::with_capacity(y));
        seen.push(Vec::with_capacity(y));
        for _ in 0..y {
            grid[i].push(0);
            seen[i].push(0);
        }
    }

    for _ in 0..nbombs {
        let mut bomb_placed: bool = false;
        while !bomb_placed {
            let xc = rng.gen_range(0..x);
            let yc = rng.gen_range(0..y);

            // If there is already a bomb, try again
            if grid[xc][yc] == -1 {
                continue
            }
            
            bomb_placed = true;
            place_bomb(&mut grid, xc, yc);
        }
    }

    (grid, seen)
}

fn input() -> usize {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse().unwrap()
}

fn uncover(grid: &Vec<Vec<i32>>, seen: &mut Vec<Vec<u32>>, xc: usize, yc: usize) {
    if seen[xc][yc] >= 2 {
        return
    }

    seen[xc][yc] = 2;

    // If we found zero, uncover adjacents
    if grid[xc][yc] == 0 {
        if xc > 0 {
            uncover(&grid, seen, xc - 1, yc);
        }
        
        if xc > 0 && (yc+1 < grid[0].len()) {
            uncover(&grid, seen, xc - 1, yc + 1);
        }

        if yc > 0 {
            uncover(&grid, seen, xc, yc - 1);
        }
    
        if xc > 0 && yc > 0 {
            uncover(&grid, seen, xc - 1, yc - 1);
        }

        if yc > 0 && (xc+1 < grid.len()) {
            uncover(&grid, seen, xc + 1, yc - 1);
        }
    
        if xc+1 < grid.len() {
            uncover(&grid, seen, xc + 1, yc);
        }

        if (xc+1 < grid.len()) && (yc+1 < grid[0].len()) {
            uncover(&grid, seen, xc + 1, yc + 1);
        }

        if yc+1 < grid[0].len() {
            uncover(&grid, seen, xc, yc + 1);
        }
    }
}

fn check_flag_win_condition(grid: &Vec<Vec<i32>>, seen: &Vec<Vec<u32>>) -> bool {
    let ylen = grid[0].len();
    for x in 0..grid.len() {
        for y in 0..ylen {
            // Flag with no bomb
            if seen[x][y] == 1 && grid[x][y] != -1 {
                return false
            }

            // Bomb with no flag
            if seen[x][y] != 1 && grid[x][y] == -1 {
                return false
            }
        }
    }

    true
}

fn main() {
    let (board, mut seen) = gen_board(8, 8, 8);
    print_board(&board, &seen, false);

    loop {
        println!("Enter command (1 to flag, 2 to uncover, 3 to quit):");
        let cmd = input();
        if cmd >= 3 {
            break;
        }

        if cmd < 1 {
            continue;
        }

        println!("Enter row:");
        let xc = input();

        println!("Enter column:");
        let yc = input();

        if cmd == 2 {
            uncover(&board, &mut seen, xc, yc);
        } else {
            seen[xc][yc] = 1;
        }

        let victory = check_flag_win_condition(&board, &seen);
        let loss = cmd >= 2 && board[xc][yc] == -1;

        print_board(&board, &seen, victory || loss);

        if victory {
            println!("You win!");
            break;
        }

        if loss {
            println!("Game over");
            break;
        }
    }
}
