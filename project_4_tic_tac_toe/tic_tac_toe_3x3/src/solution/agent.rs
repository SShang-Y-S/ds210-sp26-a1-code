use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
#[warn(unconditional_recursion)]
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        if board.game_over(){
            return (board.score(), 0 , 0)
        }

        let moves: Vec<(usize, usize)> = board.moves();

        let mut best_move = moves[0];
        board.apply_move(best_move, player);
        let (mut best_score, _, _) = SolutionAgent::solve(board, player.flip(), _time_limit);
        board.undo_move(best_move, player);

        for i in 1..moves.len(){
            let player_move = moves[i];
            board.apply_move(player_move, player);
            let (score, _, _) = SolutionAgent::solve(board, player.flip(), _time_limit);
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
}
