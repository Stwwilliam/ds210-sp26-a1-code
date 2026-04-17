use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
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
                let score = board.score();
                return (score, 0, 0)
            }
            else{
                let possibilities = board.moves();
                if player == Player::X{
                    let mut win = -2;
                    let mut position = possibilities[0];
                    for i in 0..possibilities.len(){
                        let mut diff_board = board.clone();
                        diff_board.apply_move(possibilities[i], player);
                        let (score, _, _) = SolutionAgent::solve(&mut diff_board, Player::O, _time_limit);
                        if score > win {
                            win = score;
                            position = possibilities[i]
                        }
                    }    
                    return (win, position.0, position.1);
                }
                else {
                    let mut win = 2;
                    let mut position = possibilities[0];
                    for i in 0..possibilities.len(){
                        let mut diff_board: Board = board.clone();
                        diff_board.apply_move(possibilities[i], player);
                        let (score, _, _) = SolutionAgent::solve(&mut diff_board, Player::X, _time_limit);
                        if score < win {
                            win = score;
                            position = possibilities[i]
                        }
                    }
                    return (win, position.0,position.1);
                }
            }
        }
}
