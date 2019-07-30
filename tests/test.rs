extern crate solution;

use solution::*;

#[test]
fn test_change() {
    let brd = &mut Board::new();
    brd.initialize();
    assert_eq!(Board::change(&mut Board::new(),3,3),false);
    brd.initialize();
    assert_eq!(Board::change(brd,2,1),true);
    brd.initialize();
    assert_eq!(Board::change(brd,2,2),true);
    brd.initialize();
    assert_eq!(Board::change(brd,2,0),true);
    brd.initialize();
    assert_eq!(Board::change(brd,1,1),true);
    brd.initialize();
    assert_eq!(Board::change(brd,1,0),true);
    brd.initialize();
    assert_eq!(Board::change(brd,1,2),true);
    assert_eq!(Board::change(brd,1,2),false);

    assert_eq!(Board::change(brd,1,5),false);
    assert_eq!(Board::change(brd,1,87),false);
    assert_eq!(Board::change(brd,1,22),false);
    assert_eq!(Board::change(brd,21,5),false);
    assert_eq!(Board::change(brd,7,0),false);
    assert_eq!(Board::change(brd,3,0),false);
}

#[test]
fn test_copy(){

    let mut brd = Board::new();
    brd.initialize();
    assert_eq!(Board::copy(&brd),brd);

}

#[test]

fn test_min(){
    // подаваме дъска, когато е играл първи потребителя и изчаква ход от AI
    let brd = Board {board: vec![vec!['o','_','_'], vec!['_','_','_'],vec!['_','_','_']], next : false };
    // задаваме изходът от фунцкията след анализ до възможния край на играта и следващия ход на AI
    let brd1 = Board{ board: vec![vec!['o','_','_'], vec!['_','x','_'],vec!['_','_','_']], next : true };
    let brd2 = Board{ board: vec![vec!['o','o','x'], vec!['x','x','o'],vec!['o','o','x']], next : false };

    assert_eq!(Board::max(brd),(brd1,brd2));
    
    // подаваме дъска, която е на 4ти ход и изчаква отговор от AI
    let b = Board {board: vec![vec!['o','o','_'], vec!['_','x','_'],vec!['_','_','_']], next : false };
    // задаваме изходът от фунцкията след анализ до възможния край на играта и следващия ход на AI
    let b1 = Board{ board: vec![vec!['o','o','x'], vec!['_','x','_'],vec!['_','_','_']], next : true };
    let b2 = Board{ board: vec![vec!['o','o','x'], vec!['x','x','o'],vec!['o','o','x']], next : false };

    assert_eq!(Board::max(b),(b1,b2));


// value при 1 -> x(ако е false), при -1 -> о(ако е true)
}

#[test]

fn test_value(){
    // -1 Comp губи, 1 Me губи, 0 равни.

    // ред, Comp губи
    let brd = Board {board: vec![vec!['o','o','o'], vec!['_','_','_'],vec!['_','_','_']], next : false };
    assert_eq!(Board::value(&brd),-1);

    // колона, Comp печели
    let brd2 = Board {board: vec![vec!['o','x','o'], vec!['_','x','_'],vec!['_','x','_']], next : false };
    assert_eq!(Board::value(&brd2),1);
    
    // главен диагонал, Comp губи
    let brd1 = Board {board: vec![vec!['o','x','_'], vec!['x','o','_'],vec!['_','_','o']], next : false };
    assert_eq!(Board::value(&brd1),-1);
    
    // обратен диагонал, Comp губи
    let brd3 = Board {board: vec![vec!['_','x','o'], vec!['_','o','_'],vec!['o','x','x']], next : false };
    assert_eq!(Board::value(&brd3),-1);

    // равни
    let b2 = Board{ board: vec![vec!['o','o','x'], vec!['x','x','o'],vec!['o','o','x']], next : false };
    assert_eq!(Board::value(&b2),0);

}

#[test]
fn test_is_game_finished(){
    // проверява дали дъската е пълна(не съдържа свободни позиции), без да се интересува от това кой печели
    let b2 = Board{ board: vec![vec!['o','o','x'], vec!['x','x','o'],vec!['o','o','x']], next : false };
    assert_eq!(Board::is_game_finished(&b2),true);

    let b = Board{ board: vec![vec!['x','x','x'], vec!['x','x','x'],vec!['x','x','x']], next : false };
    assert_eq!(Board::is_game_finished(&b),true);

    let b1 = Board{ board: vec![vec!['x','x','_'], vec!['x','x','x'],vec!['x','x','x']], next : false };
    assert_eq!(Board::is_game_finished(&b1),false);

}
