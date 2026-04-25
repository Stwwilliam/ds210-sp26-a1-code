use tic_tac_toe_stencil::board::Cell;
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

impl SolutionAgent {
    //https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
    fn minimax(board: &mut Board, player: Player, _time_limit: u64, depth: usize, max_depth: usize, mut alpha: i32, mut beta: i32) -> (i32, usize, usize) {
        if board.game_over() {
            let score = board.score();
            return (score, 0, 0);

        } else if depth == max_depth {
            let score = SolutionAgent::heruistic(board);
            return (score, 0, 0);

        } else {
            let possibilities = board.moves();
            if player == Player::X{
                let mut win = i32::MIN;
                let mut position = possibilities[0];
                for moves in possibilities{
                    board.apply_move(moves, player);
                    let (score, _, _) = SolutionAgent::minimax(board, Player::O, _time_limit, depth + 1, max_depth, alpha, beta);
                    board.undo_move(moves, player);
                    if score > win {
                        win = score;
                        position = moves;
                    }
                    alpha = alpha.max(win);
                    if alpha >= beta { 
                        break; 
                    }
                }    
                return (win, position.0, position.1);
            }
            else {
                let mut win = i32::MAX;
                let mut position = possibilities[0];
                for moves in possibilities {
                    board.apply_move(moves, player);
                    let (score, _, _) = SolutionAgent::minimax(board, Player::X, _time_limit,depth + 1, max_depth, alpha, beta);
                    board.undo_move(moves, player);
                    if score < win {
                        win = score;
                        position = moves;
                    }
                    beta = beta.min(win);
                    if alpha >= beta { 
                        break; 
                    }
                }
                return (win, position.0,position.1);
            }
        }
    }

    fn heruistic(board: &Board) -> i32 {
        let cells = board.get_cells();
        let n = cells.len();
        let mut score = 0;
        //row
        for i in 0..n {
            for j in 0..n-2 {
                score += Self::score_triple(
                    cells[i][j].clone(), 
                    cells[i][j+1].clone(), 
                    cells[i][j+2].clone(),
                );
            }
        }

        //column
        for i in 0..n {
            for j in 0..n-2 {
                score += Self::score_triple(
                    cells[j][i].clone(), 
                    cells[j+1][i].clone(), 
                    cells[j+2][i].clone(),
                );
            }
        }

        //diagonal \
        for i in 0..n-2 {
            for j in 0..n-2 {
                score += Self::score_triple(
                    cells[i][j].clone(),
                    cells[i+1][j+1].clone(),
                    cells[i+2][j+2].clone(),
                );
            }
        }

        //diagonal /
        for i in 2..n {
            for j in 0..n-2 {
                score += Self::score_triple(
                    cells[i][j].clone(),
                    cells[i-1][j+1].clone(),
                    cells[i-2][j+2].clone(),
                );
            }
        }
        return score;
    }

    //helper function used in heruistic
    fn score_triple(a: Cell, b:Cell, c:Cell)->i32{
        let mut x = 0;
        let mut o = 0;
        let mut empty = 0;

        for cell in [a, b, c] {
            match cell {
                Cell::X => x += 1,
                Cell::O => o += 1,
                Cell::Empty => empty += 1,
                Cell::Wall => {}
            }
        }
        //both
        if x > 0 && o > 0 {
            return 0;
        }
        //just x
        else if x == 3 {
            return 100;
        }
        else if x == 2 && empty == 1 {
            return 5;
        }
        else if x == 1 && empty == 2 {
            return 1;
        }
        //just o
        else if o == 3 {
            return -100;
        }
        else if o == 2 && empty == 1 {
            return -5;
        }
        else if o == 1 && empty == 2 {
            return -1;
        }
        return 0
    }



}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let move_count = board.moves().len();
        let depth_limit: usize;
       
        if move_count > 20 {
            depth_limit = 4;
        } else if move_count > 10 {
            depth_limit = 6;
        } else {
            depth_limit = move_count;
        };

        SolutionAgent::minimax(board, player, _time_limit, 0, depth_limit, i32::MIN, i32::MAX)
    }
}