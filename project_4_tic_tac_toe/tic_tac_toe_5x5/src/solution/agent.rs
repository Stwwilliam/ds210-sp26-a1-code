use tic_tac_toe_stencil::board::Cell;
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

impl SolutionAgent {
    fn minimax(board: &mut Board, player: Player, _time_limit: u64, depth: usize, max_depth: usize) -> (i32, usize, usize) {
        if board.game_over() {
            let score = board.score();
            return (score, 0, 0);


        } else if depth == max_depth {
            let score = SolutionAgent::heruistic(board);
            return (score, 0, 0);


        } else {
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
                        position = possibilities[i];
                    }
                }    
                return (win, position.0, position.1);
            }
            else {
                let mut win = 2;
                let mut position = possibilities[0];
                for i in 0..possibilities.len() {
                    let mut diff_board = board.clone();
                    diff_board.apply_move(possibilities[i], player);
                    let (score, _, _) = SolutionAgent::solve(&mut diff_board, Player::O, _time_limit);
                    if score < win {
                        win = score;
                        position = possibilities[i];
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

        //rows
        for row in 0..n {
            for i in 0..3 {
                if cells[row][i] == cells[row][i + 1] && cells[row][i + 1] == cells[row][i + 2] {
               
                    if cells[row][i] == Cell::X {
                        score += 1;
                    }
                    else if cells[row][0] == Cell::O{
                        score -= 1;
                    }
                }
            }
        }

        //columns
        for col in 0..n {
            for i in 0..3 {
                if cells[i][col] == cells[i + 1][col] && cells[i + 1][col] == cells[i + 2][col] {
               
                    if cells[i][col] == Cell::X {
                        score += 1;
                    }
                    else if cells[i][col] == Cell::O{
                        score -= 1;
                    }
                }
            }
        }

        //diagonal \
        for row in 0..n-2 {
            for col in 0..n-2 {
                //0, 0 and 1, 1 and 2, 2
                //0, 1 and 1, 2 and 2, 3
                //0, 2 and 1, 3 and 2, 4


                //1, 0 and 2, 1 and 3, 2
                //1, 1 and 2, 2 and 3, 3
                //1, 2 and 2, 3 and 3, 4


                //2, 0 and 3, 1 and 4, 2
                //2, 1 and 3, 2 and 4, 3
                //2, 2 and 3, 3 and 4, 4
                if cells[row][col] == cells[row + 1][col + 1] && cells[row + 1][col + 1] == cells[row + 2][col + 2] {
               
                    if cells[row][col] == Cell::X {
                        score += 1;
                    }
                    else if cells[row][col] == Cell::O{
                        score -= 1;
                    }
                }
            }
        }

        //diagonal /
        for row in 2..n {
            for col in 0..n-2 {
                //2, 0 and 1, 1 and 0, 2
                //2, 1 and 1, 2 and 0, 3
                //2, 2 and 1, 3 and 0, 4


                //3, 0 and 2, 1 and 1, 2
                //3, 1 and 2, 2 and 1, 3
                //3, 2 and 2, 3 and 1, 4


                //4, 0 and 3, 1 and 2, 2
                //4, 1 and 3, 2 and 2, 3
                //4, 2 and 3, 3 and 2, 4
                if cells[row][col] == cells[row - 1][col + 1] && cells[row - 1][col + 1] == cells[row - 2][col + 2] {
               
                    if cells[row][col] == Cell::X {
                        score += 1;
                    }
                    else if cells[row][col] == Cell::O{
                        score -= 1;
                    }
                }
            }
        }
        return score;
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


        //if move_count < 10 {
        //  depth_limit = move_count;
        //} else {
        //  depth_limit = 4;
        //}
       
        if move_count > 15 {
            depth_limit = 4;
        } else if move_count > 10 {
            depth_limit = 6;
        } else {
            depth_limit = move_count;
        };


        SolutionAgent::minimax(board, player, _time_limit, 0, depth_limit)
    }
}
