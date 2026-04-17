use std::thread::park_timeout;
use std::time::Duration;

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

fn heuristic(board: &mut Board) -> i32{
    return board.score();
}

fn minmax_depth(board: &mut Board, player: Player, depth: u32, max_depth:u32) -> (i32, usize, usize){
    if board.game_over(){
        return (board.score(), 0, 0)
    }

    if depth == max_depth{
        return (heuristic(board), 0, 0);
    }

    let moves: Vec<(usize, usize)> = board.moves();
    let mut best_move = moves[0];
    board.apply_move(best_move, player);
    let (mut best_score, _, _) = minmax_depth(board, player.flip(), depth+1, max_depth);
    board.undo_move(best_move, player);

    for i in 1..moves.len(){
            let player_move = moves[i];
            board.apply_move(player_move, player);
            let (score, _, _) = minmax_depth(board, player.flip(), depth+1, max_depth);
            board.undo_move(player_move, player);

            match player{
                Player::X => {
                    if score > best_score{ 
                        best_move = player_move;
                        best_score = score;
                    }
                }
                Player::O => {
                    if score < best_score{
                        best_move = player_move;
                        best_score = score;
                        }
                    }
                }
        }
        return (best_score, best_move.0 as usize, best_move.1 as usize);
    }
// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.

    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let max_depth: u32 = 4;
        return minmax_depth(board, player, 0, max_depth);
    }
}
