mod game;

use game::input::Input;
use game::Game;

fn main() {
    let mut stop = String::from("n");

    while stop != "y" {
        let mut game = Game::new();
        game.paint(5, 5);

        while game.get_in_game() {
            println!("Enter row number:");
            let c = Input::input_usize();
            println!("Enter column number: ");
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

        println!("Want to exit? y/n");
        stop = Input::input_string();
    }
}
