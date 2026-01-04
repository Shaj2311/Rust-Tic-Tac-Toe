#[derive(PartialEq, PartialOrd)]
pub enum CellValue
{
    BLANK,
    PLAYER1,
    PLAYER2
}
use CellValue::*;

fn main() {
    //initialize board
    let mut board: [[CellValue; 3]; 3] = [
        [BLANK,BLANK,BLANK],
        [BLANK,BLANK,BLANK],
        [BLANK,BLANK,BLANK]
    ];
    //player 1 starts
    let mut player_1_turn = true;

    draw_board(&board);
}

//print board
fn draw_board(board: &[[CellValue;3];3]) {
    for _i in 0..7
    {
        print!("-");
    }
    println!("");

    for i in 0..3
    {

        print!("|");

        for j in 0..3
        {
            print!("{}|",
                 if board[i][j] == PLAYER1 {'O'}
                else if board[i][j] == PLAYER2 {'X'}
                else {' '}
            );
        }

        println!("");
        for _i in 0..7
        {
            print!("-");
        }
        println!("");

    }
}
