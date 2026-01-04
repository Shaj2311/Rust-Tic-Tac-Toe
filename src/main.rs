use std::io::Write;

#[derive(PartialEq, PartialOrd)]
pub enum CellValue
{
    BLANK,
    PLAYER1,
    PLAYER2
}
use CellValue::*;

fn main()
{
    //initialize board
    let mut board: [[CellValue; 3]; 3] = [
        [BLANK,BLANK,BLANK],
        [BLANK,BLANK,BLANK],
        [BLANK,BLANK,BLANK]
    ];
    //player 1 starts
    let mut is_player1_turn = true;

    //turn-based loop
    loop
    {
        //print board
        draw_board(&board);

        //print message
        println!("Player {}'s Turn",
            if is_player1_turn {1}
            else {2}
            );



        //Take position input
        let mut pos_input = String::new();
        print!("Enter position (1-9): ");
        let _=std::io::stdout().flush();
        let _=std::io::stdin().read_line(&mut pos_input);

        //Get index from position
        //TEST
        let pos_int: i32 = pos_input.trim().parse().expect("Invalid bruh");

        //Handle invalid positions
        if pos_int <= 0 || pos_int > 9
        {
            println!("Invalid input!");
            continue;
        }

        //get row and column index
        let row_index:i32 = (pos_int-1) / 3;
        let col_index:i32 = (pos_int-1) % 3;

        //debug
        println!("Row: {row_index}\n Column: {col_index}\n");

        //next player's turn
        is_player1_turn = !is_player1_turn;
        



    }
}

//print board
fn draw_board(board: &[[CellValue;3];3])
{
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
