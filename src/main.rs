fn main() {
    let mut state = seed_board();
    print_board(&state);

    for i in 1..20 {
        println!("Iteration {}", i);
        state = get_next_state(state);
        print_board(&state);
    }
}

fn seed_board() -> [[char; 10]; 10] {
    let empty_row = [' '; 10];
    let mut state = [empty_row; 10];

    for i in 1..10 {
        for j in 1..10 {
            // TODO why don't we need to bring this into the namespace?
            if rand::random::<f32>() < 0.4 {
                state[i][j] = 'X';
            }
        }
    }
    state
}

fn get_next_state(state: [[char; 10]; 10]) -> [[char; 10]; 10] {
    let mut next_state = state;

    for row_index in 0..10 {
        for col_index in 0..10 {
            let cell_alive = state[row_index as usize][col_index as usize] == 'X';
            let live_neighbours = count_live_neighbours(&state, row_index, col_index);

            let mut next_cell_state = ' ';
            // dead cells are born if they have exactly three neighbours
            if !cell_alive && live_neighbours == 3 {
                next_cell_state = 'X';
            // live cells stay alive with two or three neighbours
            } else if cell_alive && (live_neighbours == 2 || live_neighbours == 3) {
                next_cell_state = 'X';
            }
            next_state[row_index as usize][col_index as usize] = next_cell_state;
        }
    }
    next_state
}

fn count_live_neighbours(state: &[[char; 10]; 10], row: i32, col: i32) -> i32 {
    let mut live_neighbours = 0;

    for i in row - 1..row + 1 {
        for j in col - 1..row + 1 {
            if (i < 0) || (i > 10) || (j < 0) || (j > 10) || (i == row && j == col) {
                continue;
            } else if state[i as usize][j as usize] == 'X' {
                live_neighbours += 1;
            }
        }
    }
    live_neighbours
}

// array arg has to have a fixed size?
fn print_board(state: &[[char; 10]; 10]) {
    println!("{}", state.len());

    for row in state {
        // :? is from the debug trait and implemented on basic types
        // otherwise would need to implment a display(?) trait on this array type
        println!("{:?}", row);
    }
}
