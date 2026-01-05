use std::io;
use std::io::Write;

#[derive(PartialEq, PartialOrd)]
enum CellValue
{
    BLANK,
    PLAYER1,
    PLAYER2
}
use CellValue::*;

#[derive(Debug)]
enum State
{
    MENU,
    PLAY,
    WIN,
    DRAW,
    EXIT
}


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

    //start with menu
    let mut game_state = State::MENU;
    
    //full program loop
    'state_loop: loop
    {

        match game_state
        {
            State::MENU =>
            {
                //draw menu and take input
                //display start menu
                print_start_menu();

                //take input
                let mut menu_input = String::new();
                let _=io::stdout().flush();
                let _=io::stdin().read_line(&mut menu_input);

                //validate input
                let menu_int = menu_input.trim().parse().unwrap_or(0);
                if menu_int == 1
                {
                    //switch to play state
                    switch_to_state(&mut game_state, State::PLAY);
                }
                else if menu_int == 2
                {
                    //switch to exit state
                    switch_to_state(&mut game_state, State::EXIT);
                }
            }
            State::PLAY =>
            {
                //turn-based loop
                let mut _game_loop_iterator = 0;
                while _game_loop_iterator < 9
                {
                    _game_loop_iterator+=1;
                    //clear screen, move cursor to top-left
                    println!("\x1b[2J\x1b[H");

                    //print board
                    draw_board(&board);

                    //DEBUG
                    println!("Current state: {:?}", game_state);

                    //print message
                    println!("Player {}'s Turn",
                        if is_player1_turn {1}
                        else {2}
                    );



                    //Take position input
                    let mut pos_input = String::new();
                    print!("Enter position (1-9): ");
                    let _=io::stdout().flush();
                    let _=io::stdin().read_line(&mut pos_input);
                    if pos_input.trim() == ""
                    {
                        _game_loop_iterator-=1;
                        continue;
                    }

                    //Get index from position
                    let pos_int: i32 = pos_input.trim().parse().unwrap_or(0);

                    //Handle invalid positions
                    if pos_int <= 0 || pos_int > 9
                    {
                        _game_loop_iterator -= 1;
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
                        _game_loop_iterator -= 1;
                        continue;
                    }

                    //check win
                    if player_wins(is_player1_turn, &board)
                    {
                        //switch to win state
                        switch_to_state(&mut game_state, State::WIN);
                        continue 'state_loop;
                    }

                    //next player's turn
                    is_player1_turn = !is_player1_turn;


                }

                //game draw
                switch_to_state(&mut game_state, State::DRAW);

            }
            State::WIN =>
            {
                println!("\x1b[2J\x1b[H");
                //draw final board
                draw_board(&board);
                //print win message
                println!("Player {} wins!", if is_player1_turn {1} else {2});
                //pause
                println!("Press Enter to Continue...");
                let _=io::stdin().read_line(&mut String::new());
                //return to menu
                switch_to_state(&mut game_state, State::MENU);
            }
            State::DRAW =>
            {
                //game draw
                println!("\x1b[2J\x1b[H");
                //draw final board
                draw_board(&board);
                //print draw message
                println!("Draw!");
                //pause
                println!("Press Enter to Continue...");
                let _=io::stdin().read_line(&mut String::new());
                //return to menu
                switch_to_state(&mut game_state, State::MENU);

            }
            State::EXIT =>
            {
                println!("\x1b[2J\x1b[H");
                println!("Thanks for playing!");
                println!("Exiting...");
                std::process::exit(0);
            }
        }




    }
}

fn print_start_menu()
{
    println!("\x1b[2J\x1b[H");
    println!("TIC TAC TOE");
    println!("1. Start");
    println!("2. Exit");
    print!("Enter your choice: ");
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

fn switch_to_state(curr_state: &mut State, new_state: State)
{
    //update state
    *curr_state = new_state;
    println!("Switching to state {:?}", curr_state);
    let _=io::stdout().flush();
    let _=io::stdin().read_line(&mut String::new());
    match curr_state
    {
        State::MENU =>
        {

        }
        State::PLAY =>
        {

        }
        State::WIN =>
        {

        }
        State::DRAW =>
        {

        }
        State::EXIT =>
        {

        }
    }
}
