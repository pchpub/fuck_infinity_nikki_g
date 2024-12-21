use fuck_infinity_nikki_g::libs::game::checkerboard::{
    init_checkerboard, verify_game, Color, Direction, Element, RoadSign,
};

fn main() {
    let board: Vec<Vec<&str>> = vec![
        vec!["", "", "", "", "E_Y", "", "", "", ""],
        vec!["", "", "", "", "", "", "", "", ""],
        vec!["", "", "", "", "T", "T", "", "", ""],
        vec!["", "", "", "", "", "", "", "", "Y_Point"],
        vec!["", "", "", "", "", "", "", "", ""],
        vec!["", "", "", "", "", "", "", "", ""],
        vec!["", "", "", "", "", "", "", "", ""],
    ];
    let (mut board, _) = init_checkerboard(board, vec![]);
    board.set_element(
        (0, 8),
        Element::RoadSign(RoadSign {
            direction: vec![(Direction::Down, Color::Yellow)],
        }),
    );
    board.set_element(
        (0, 0),
        Element::RoadSign(RoadSign {
            direction: vec![(Direction::Right, Color::Yellow)],
        }),
    );

    if verify_game(&board) {
        println!("OK");
    } else {
        println!("Error");
    }
}
