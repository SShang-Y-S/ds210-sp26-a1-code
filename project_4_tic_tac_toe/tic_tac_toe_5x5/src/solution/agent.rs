use std::thread::park_timeout;
use std::time::Duration;

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use tic_tac_toe_stencil::board::Cell;
// i added this use statement to access the cell

// Your solution solution.
pub struct SolutionAgent {}

fn heuristic(board: &mut Board) -> i32{
    let cells = board.get_cells();
    let rows = cells.len();
    let cols = cells[0].len();
    let mut score: i32 = 0;

    // helper: this scores a single 3 cell window
    // takes three cell values, returns the score contribution
    let eval_window = |a: &Cell, b: &Cell, c: &Cell| -> i32 {
        let window = [a, b, c];
        let mut x_count = 0;
        let mut o_count = 0;
        let mut wall = false;

        for cell in window.iter() {
            match cell {
                Cell::X => x_count += 1,
                Cell::O => o_count += 1,
                Cell::Wall => wall = true,
                Cell::Empty => {}
            }
        }

        // Dead window: has a wall, or has both X and O
        if wall || (x_count > 0 && o_count > 0) {
            return 0;
        }

        match (x_count, o_count) {
            (3, 0) => 100,
            (2, 0) => 10,
            (1, 0) => 1,
            (0, 3) => -100,
            (0, 2) => -10,
            (0, 1) => -1,
            _ => 0,
        }
    };

    // this is the horizontal windows (rows)
    for r in 0..rows {
        for c in 0..cols.saturating_sub(2) {
            score += eval_window(&cells[r][c], &cells[r][c+1], &cells[r][c+2]);
        }
    }

    // this is the vertical windows (columns)
    for r in 0..rows.saturating_sub(2) {
        for c in 0..cols {
            score += eval_window(&cells[r][c], &cells[r+1][c], &cells[r+2][c]);
        }
    }

    // this is for diagonal windows, from the top left to the bottom right
    for r in 0..rows.saturating_sub(2) {
        for c in 0..cols.saturating_sub(2) {
            score += eval_window(&cells[r][c], &cells[r+1][c+1], &cells[r+2][c+2]);
        }
    }

    // this is the reverse diagonal windows, from top right to bottom left
    for r in 0..rows.saturating_sub(2) {
        for c in 2..cols {
            score += eval_window(&cells[r][c], &cells[r+1][c-1], &cells[r+2][c-2]);
        }
    }

    // Says the score
    score
}
// fn immediate_move(board: &mut Board, player: Player) -> Option<(usize, usize)>{
//     let moves = board.moves();
    
//     for m in moves.iter(){
//         board.apply_move(*m, player);
//         let score = board.score();
//         board.undo_move(*m, player);

//         match player{
//             Player::X => {
//                 if score > 0 {
//                     return Some(*m);
//                 }
//             }
//             Player::O => {
//                 if score < 0{
//                     return Some(*m);
//                 }
//             }
//         }
//     }
//     let opponent = player.flip();
//     for m in moves.iter(){
//         board.apply_move(*m, player);
//         let score = board.score();
//         board.undo_move(*m, player);

//         match opponent {
//             Player::X => {
//                 if score > 0 {
//                     return Some(*m);
//                 }
//             }
//             Player::O => {
//                 if score < 0{
//                     return Some(*m);
//                 }
//             }
//         }
//     }
//     return None;
// }

fn minmax_depth(board: &mut Board, player: Player, depth: u32, max_depth: u32, mut alpha: i32, mut beta: i32) -> (i32, usize, usize){
    if board.game_over(){
        return (board.score(), 0, 0)
    }

    if depth == max_depth{
        return (heuristic(board), 0, 0);
    }

    let moves: Vec<(usize, usize)> = board.moves();
    let mut best_move = moves[0];

            match player{
                Player::X => {
                    let mut best_score = i32::MIN;
                    for m in moves{
                        board.apply_move(m, player);
                        let (score, _, _) = minmax_depth(board, player.flip(), depth+1, max_depth, alpha, beta);
                        board.undo_move(m, player);

                        if score > best_score{ 
                            best_score = score;
                            best_move = m;

                        }

                        if best_score > alpha{
                            alpha = best_score;
                        }

                        if alpha >= beta{
                            break;
                        }
                    }
                    return (best_score, best_move.0, best_move.1);
                }
                Player::O => {
                    let mut best_score = i32::MAX;

                    for m in moves{
                        board.apply_move(m, player);
                        let (score, _, _) = minmax_depth(board, player.flip(), depth + 1, max_depth, alpha, beta);
                        board.undo_move(m, player);
                        if score < best_score{
                            best_score = score;
                            best_move = m;
                            }
                        if best_score < beta {
                            beta = best_score;
                        }
                        if alpha >= beta{
                            break;
                        }
                        
                    }
                    return (best_score, best_move.0 as usize, best_move.1 as usize);

                }
            }
    }
// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.

    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // if let Some(m) = immediate_move(board, player){
        //     return (board.score(), m.0, m.1)
        // }

        let max_depth: u32 = 4;
        // if board.get_cells().len() == 3{
        //     max_depth = board.moves().len() as u32;
        // }
        // else{
        //     max_depth = 4;
        // }
        return minmax_depth(board, player, 0, max_depth, i32::MIN, i32::MAX);
    }
}
