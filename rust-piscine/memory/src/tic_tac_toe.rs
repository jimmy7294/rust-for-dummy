/*
Create some functions for a tic_tac_toe checker.
fn tic_tac_toe receives a tic-tac-toe board. It returns either: "player 0 won", "player X won" or "tie".
fn "diagonals, horizontal, vertical" which accepts a player and a board. These functions should return true
if the player has won in the given direction.

players: "X", "O" or "none"
board: 3x3 matrix of strings (or 4x4)
tic_tac_toe(vec![
            vec!["O", "X", "O"],
            vec!["O", "O", "X"],
            vec!["X", "#", "X"]
        ])
-> "tie"
- the board is always square
- the game has been played until the end
- the task is about examining the board and not about generating it
- the point of the task is to practice ownership and borrowing
- ownership means that you can only use the board once
- borrowing means that you can only read the board
- function tic_tac_toe has the ownership of the board
- functions diagonals, horizontal, vertical have the borrowing of the board, meaning that they can only read it
- the diagonals returns true if the player has won in any of the diagonals ((0, 0), (1, 1), (2, 2)) or ((0, 2), (1, 1), (2, 0))
- the horizontal returns true if the player has played in any of the rows ((0, 0), (0, 1), (0, 2)) or ((1, 0), (1, 1), (1, 2)) or ((2, 0), (2, 1), (2, 2))
- the vertical returns true if the player has played in any of the columns ((0, 0), (1, 0), (2, 0)) or ((0, 1), (1, 1), (2, 1)) or ((0, 2), (1, 2), (2, 2))
- the board is always square:
((0, 0), (0, 1), (0, 2))
((1, 0), (1, 1), (1, 2))
((2, 0), (2, 1), (2, 2))
- if the 'player' has appearances in any of the above combination, the function should return true
*/

pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let mut result = String::new();
    let player = vec!["O", "X", "none"];
    for p in player {
        if diagonals(p, &table) || horizontal(p, &table) || vertical(p, &table) {
            result = format!("player {} won", p);
            break;
        } else {
            result = "tie".to_string();
        }
    }
    result
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    match player {
        "X" | "O" => {
            let d1 = ((0, 0), (1, 1), (2, 2));
            let d2 = ((0, 2), (1, 1), (2, 0));

            if player == table[d1.0 .0][d1.0 .1]
                && player == table[d1.1 .0][d1.1 .1]
                && player == table[d1.2 .0][d1.2 .1]
            {
                return true;
            } else if player == table[d2.0 .0][d2.0 .1]
                && player == table[d2.1 .0][d2.1 .1]
                && player == table[d2.2 .0][d2.2 .1]
            {
                return true;
            } else {
                return false;
            }
        }
        _ => return false,
    }
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    match player {
        "X" | "O" => {
            let h1 = ((0, 0), (0, 1), (0, 2));
            let h2 = ((1, 0), (1, 1), (1, 2));
            let h3 = ((2, 0), (2, 1), (2, 2));

            if player == table[h1.0 .0][h1.0 .1]
                && player == table[h1.1 .0][h1.1 .1]
                && player == table[h1.2 .0][h1.2 .1]
            {
                return true;
            } else if player == table[h2.0 .0][h2.0 .1]
                && player == table[h2.1 .0][h2.1 .1]
                && player == table[h2.2 .0][h2.2 .1]
            {
                return true;
            } else if player == table[h3.0 .0][h3.0 .1]
                && player == table[h3.1 .0][h3.1 .1]
                && player == table[h3.2 .0][h3.2 .1]
            {
                return true;
            } else {
                return false;
            }
        }

        _ => return false,
    }
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    match player {
        "X" | "O" => {
            let v1 = ((0, 0), (1, 0), (2, 0));
            let v2 = ((0, 1), (1, 1), (2, 1));
            let v3 = ((0, 2), (1, 2), (2, 2));

            if player == table[v1.0 .0][v1.0 .1]
                && player == table[v1.1 .0][v1.1 .1]
                && player == table[v1.2 .0][v1.2 .1]
            {
                return true;
            } else if player == table[v2.0 .0][v2.0 .1]
                && player == table[v2.1 .0][v2.1 .1]
                && player == table[v2.2 .0][v2.2 .1]
            {
                return true;
            } else if player == table[v3.0 .0][v3.0 .1]
                && player == table[v3.1 .0][v3.1 .1]
                && player == table[v3.2 .0][v3.2 .1]
            {
                return true;
            } else {
                return false;
            }
        }
        _ => return false,
    }
}
