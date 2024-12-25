use std::io;

fn main() {

    let starting_player = Mark::X;

    let mut board = Board::new();
    let mut current_player = starting_player;
    

    board.display_board();

    loop {

        println!("");

        match current_player {
            Mark::X => println!("Player [ X ], enter X-Y coordinates."),
            Mark::O => println!("Player [ O ], enter X-Y coordinates."),
            Mark::Empty => println!("Lost track of the current player."),
        }

        let mut string_coordinates: String = String::new();
        io::stdin().read_line(&mut string_coordinates).expect("Failed to read input");

        
        if string_coordinates.trim().split_once('-').is_none() {
            println!("Invalid Input: Input must be in the format 'X-Y'.");
            continue;
        } else if

            let Some((x_str, y_str)) = string_coordinates.trim().split_once('-') {

            if x_str.parse::<usize>().is_err() {
                println!("Invalid Input: First coordinate must be a number.");
                continue; 
            }

            if y_str.parse::<usize>().is_err() {
                println!("Invalid Input: Second coordinate must be a number.");
                continue;
            }

            let x = x_str.parse::<usize>().unwrap();
            let y = y_str.parse::<usize>().unwrap();

            if ![0, 1, 2].contains(&x) || ![0, 1, 2].contains(&y) {
                println!("Invalid Input: Coordinates must be in the range 0-2.");
                continue;
            }

            if ![0, 1, 2].contains(&x) || ![0, 1, 2].contains(&y) {
                println!("Invalid Input: Coordinates must be 0, 1 or 2.");
                continue;
            }

            if !board.make_a_move(x, y, current_player) {
                println!("Invalid Move, please select an empty spot.");
                continue;
            };
            board.display_board();
        
        }

        if board.player_wins(current_player) {
            
            match current_player {
                Mark::X => { println!("Player [ X ] wins!") }
                Mark::O => { println!("Player [ O ] wins!") }
                Mark::Empty => { println!("Lost track of the current player.") }
            }

            break;

        }

        if board.is_full() {

            println!("It's a draw!");

            break;

        }

        current_player = if current_player == Mark::X { Mark::O } else { Mark::X }

    }


}

#[derive(Clone, Copy, Default, PartialEq)]
enum Mark {
    X,
    O,
    #[default] Empty,
}

struct Board {
    cells: [Mark; 9]
}

impl Board {
    
    fn new() -> Self {
        
        Board {
            cells: [Mark::Empty; 9],
        }

    }

    fn display_board(&self) {

        let command_status = std::process::Command::new("clear").status();

        match command_status {
            Ok(..) => {}
            Err(..) => { std::process::Command::new("cls").status().unwrap(); }
        }

        for y in 0..3 {

            for x in 0..3 {

                let cell = self.cells[3 * y + x];
                match cell {
                    Mark::X => print!(" X "),
                    Mark::O => print!(" O "),
                    Mark::Empty => print!(" . "),
                }

            }

            println!("");

        }

    }

    fn move_is_valid(&mut self, x: usize, y: usize) -> bool {

        self.cells[3 * y + x] == Mark::Empty

    }

    fn make_a_move(&mut self, x: usize, y: usize, mark: Mark) -> bool {

        if self.move_is_valid(x, y) {
            self.cells[3 * y + x] = mark;
            return true;
        }

        false

    }

    fn player_wins(&self, mark: Mark) -> bool {

        for y in 0..3 {

            if
                self.cells[3 * y + 0] == mark &&
                self.cells[3 * y + 1] == mark &&
                self.cells[3 * y + 2] == mark
            {
                return true;
            }        

        }

        for x in 0..3 {

            if
                self.cells[x + 0] == mark &&
                self.cells[x + 3] == mark &&
                self.cells[x + 6] == mark
            {
                return true;
            }

        }

        if 
            self.cells[4] == mark &&
            self.cells[0] == mark &&
            self.cells[8] == mark 
        {
            return true;
        }

        if
            self.cells[4] == mark &&
            self.cells[2] == mark &&
            self.cells[6] == mark
        {
            return true;
        }

        false

    }

    fn is_full(&self) -> bool {

        !self.cells.iter().any(|&cell| cell == Mark::Empty)

    }

}
