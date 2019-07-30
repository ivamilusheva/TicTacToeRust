use std::cmp::Ordering;


#[derive(Eq,Clone,Debug)]
pub struct Board{
    pub board : Vec<Vec<char>>,
    pub next : bool
}

impl Ord for Board {
    fn cmp(&self, other: &Board) -> Ordering {
        self.board.cmp(&other.board)
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Board) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        self.board == other.board
    }
}

impl Board{
    pub fn new() -> Self{
        let vec : Vec<Vec<char>> = Vec::new();
        Board{ board : vec, next : false }
    }

    pub fn initialize(&mut self){
        for _j in 0..3{
            let vec = vec!['_'; 3];
            self.board.push(vec);
        }
    }

    pub fn change(&mut self, i : usize, j : usize) -> bool{
        if i >= 3 {
            return false;
        }

        if j >= 3 {
            return false;
        }

        let board = &self.board;
        if board[i][j] != '_' {
            return false;
        }
        self.board[i][j] = 'o';
        self.next = false;
        return true;
    }

    pub fn print_matrix(&mut self){
        for i in 0..3 {
            for j in 0..3 {
                print!("{} ", self.board[i][j]);
            }
            println!();
        }

        println!();
    }

    pub fn copy(&self) -> Board {
        let mut b = Board::new();
        b.initialize();

        for i in 0..3 {
            for j in 0..3 {
                b.board[i][j] = self.board[i][j];
            }
        }
        b.next = self.next;
        b
    }

    fn get_possible_boards(&mut self) -> Vec<Board> {
        let mut list : Vec<Board> = Vec::new();
        
        for i in 0..3{
            for j in 0..3{
                if self.board[i][j] == '_'{
                    let mut b = self.copy();
                    if self.next == false {
                        b.board[i][j] = 'x';
                    }
                    else
                    {
                        b.board[i][j] = 'o';
                    }
                    b.next = !self.next;
                    list.push(b);
                }
            }
        }
        return list;
    }

    pub fn value(&self) -> i32{
        for i in 0..3 {
            if self.board[i][1] == self.board[i][2] && self.board[i][1] == self.board[i][0] {
                if self.board[i][0] == 'x' {
                    return 1;
                }
                else if self.board[i][0] == 'o' {
                    return -1;
                }
            }
            if self.board[1][i] == self.board[2][i] && self.board[1][i] == self.board[0][i] {
                if self.board[0][i] == 'x' {
                    return 1;
                }
                else if self.board[0][i] == 'o' {
                    return -1;
                }
            }
        }

        // first diagonal
        if self.board[1][1] == self.board[0][0] && self.board[1][1] == self.board[2][2] {
            if self.board[1][1] == 'x' {
                return 1;
            }
            else if self.board[1][1] == 'o' {
                return -1;
            }
        }
        
        // second diagonal
        if self.board[1][1] == self.board[0][2] && self.board[1][1] == self.board[2][0] {
            if self.board[1][1] == 'x' {
                return 1;
            }
            else if self.board[1][1] == 'o' {
                return -1;
            }
        }
                
        return 0;
    }

    pub fn min(mut b : Board) -> (Board, Board){
        let a;
        if b.value() == -1 || b.value() == 1 {
            a = b.copy();
            return (a, b);
        }

        let list = b.get_possible_boards();
        if list.len() == 0 {
            a = b.copy();
            return (a, b);
        }

        let min : &mut Vec<(Board, Board)> = &mut Vec::new();

        for item in list {
            let result = Self::max(item.clone());
            min.push((item, result.1))
        }
        
        min.sort_by_key(|x| x.1.clone().value());
        return min[0].clone();
    }

    pub fn max(mut b : Board) -> (Board, Board) {
        let a;
        if b.value() == -1 || b.value() == 1{
            a = b.copy();
            return (a, b);
        }
        let list = b.get_possible_boards();

        if list.len() == 0 {
            a = b.copy();
            return (a,b);
        }

        let mut max : Vec<(Board, Board)> = Vec::new();

        for item in list {
            let result = Self::min(item.clone());
            max.push((item, result.1))
        }
        max.sort_by_key(|x| x.1.clone().value());
        max.reverse();
        return max[0].clone();
    }

    pub fn is_game_finished(&self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == '_'{
                    return false;
                }
            }
        }
        return true;
    }
}
