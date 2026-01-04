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
        //clear screen, move cursor to top-left
        println!("\x1b[2J\x1b[H");

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
        let pos_int: i32 = pos_input.trim().parse().expect("Invalid Cell Value Input");

        //Handle invalid positions
        if pos_int <= 0 || pos_int > 9
        {
            println!("Invalid input!");
            continue;
        }

        //get row and column index
        let row_index= ((pos_int-1) / 3) as usize;
        let col_index = ((pos_int-1) % 3) as usize;

        //mark cell
        if board[row_index][col_index] == BLANK
        {
            board[row_index][col_index] = if is_player1_turn {PLAYER1} else {PLAYER2};
        }
        else
        {
            continue;
        }
        
        //check win
        if player_wins(is_player1_turn, &board)
        {
            println!("\x1b[2J\x1b[H");
            draw_board(&board);
            println!("Player {} wins!", if is_player1_turn {1} else {2});
            break;
        }

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

fn player_wins(p1_turn: bool, board: &[[CellValue;3];3]) -> bool
{
    let target_cell_value: CellValue = 
        if p1_turn {PLAYER1}
        else {PLAYER2};

    for i in 0..3
    {
        //check horizontal lines
        if board[0][i] == target_cell_value && 
            board[1][i] == target_cell_value && 
                board[2][i] == target_cell_value
        {
            return true;
        }


        //check vertical lines
        if board[i][0] == target_cell_value && 
            board[i][1] == target_cell_value && 
                board[i][2] == target_cell_value
        {
            return true;
        }

    }

    //check diagonals
    if board[0][0] == target_cell_value && 
        board[1][1] == target_cell_value && 
            board[2][2] == target_cell_value
    {
        return true;
    }

    if board[0][2] == target_cell_value && 
        board[1][1] == target_cell_value && 
            board[2][0] == target_cell_value
    {
        return true;
    }

    return false;
}
