use std::fs;

#[derive(PartialEq, Clone)]
enum Tile {
    Empty,
    Roll,
    RemovableRoll,
}

#[derive(Clone)]
struct Board {
    board: Vec<Vec<Tile>>,
}
impl Board {
    fn contains(&self, x: usize, y: usize) -> bool{
        return x < self.board[0].len() && y < self.board.len();
    }
    fn check_tile(&self, x: i32, y: i32) -> Tile{
        let mut counter = 0;
        let tile = &self.board[x as usize][y as usize]; 
        if *tile == Tile::Empty || *tile == Tile::RemovableRoll{
            return Tile::Empty;
        }
        let directions: [[i32; 2]; 8] = [
            [-1, -1], [0, -1], [1, -1], 
            [-1, 0],           [1, 0],
            [-1, 1], [0, 1], [1, 1]];

        for [dx, dy] in directions{
            let x_check = (x+dx) as usize;
            let y_check = (y+dy) as usize;
            if self.contains(x_check, y_check) && self.board[x_check][y_check] == Tile::Roll{
                counter += 1;
            }
        }
        if counter < 4 {
            return Tile::RemovableRoll;
        }
        else{
            return Tile::Roll;
        }
    }
    fn next(&mut self) -> Self{
        let mut new_board = self.clone();

        for y_index in 0..self.board.len(){
            for x_index in 0..self.board[0].len(){
                new_board.board[x_index][y_index] = self.check_tile(x_index as i32, y_index as i32); 
            }
        }
        new_board
    }
    fn get_removable(&self) -> i32{
        let mut counter = 0;
        for row in &self.board{
            for tile in row{
                if *tile == Tile::RemovableRoll{
                    counter += 1;
                }
            }
        }
        counter
    }
    fn output(&self){
        for y_index in 0..self.board.len(){
            for x_index in 0..self.board[0].len(){
                match self.board[y_index][x_index]{
                    Tile::Empty => print!("."),
                    Tile::Roll => print!("@"),
                    Tile::RemovableRoll => print!("X"),
                }
            }
            println!("");
        }
    }
}
fn convert_to_tile(character: char) -> Tile{
    return match character{
        '.' => Tile::Empty,
        'x' => Tile::RemovableRoll,
        '@' => Tile::Roll,
        _ => panic!("Tile type inputted that isn't covered in convert_to_tile")
    }

}
fn get_board() -> String{
    let path = "resources/day4/roll_map";
    fs::read_to_string(path).unwrap()
}
fn split_board(raw_board: String) -> Board{
    let lines = raw_board.lines();
    let chars = lines.map(|lines| lines.chars());
    let tiles = chars.map(|lines| lines.map(|unenum| convert_to_tile(unenum)).collect()).collect();

    return Board{
        board: tiles,
    };
}
pub fn remove_rolls(){
    let buffer = get_board(); 
    let mut board = split_board(buffer);
    let mut counter = 0;
    let mut removable = 1;
    while removable != 0{
        board = board.next();
        removable = board.get_removable();
        counter += removable;
    }
    board.output();
    println!("There are {} total removable rolls", counter);
}
