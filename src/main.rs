mod game;

use game::input::Input;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.paint(5, 5);

    while game.get_in_game() {
        let c = Input::input_usize();
        let r = Input::input_usize();
        game.turn(c, r);
    }

    if game.get_cause_of_end() == 1 {
        println!("Draw!");
    }
    else if game.get_player() == 1 && game.get_cause_of_end() != 1 {
        println!("First player won!");
    }
    else if game.get_player() == 2 && game.get_cause_of_end() != 1 {
        println!("Second player won!");
    }

    let exit = Input::input_i8();
}
