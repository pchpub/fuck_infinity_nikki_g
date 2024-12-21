#[derive(Clone)]
pub enum Element {
    Empty,
    Elf(Elf),
    EndPoint(EndPoint),
    Tree, // same as Wall
    RoadSign(RoadSign),
}

#[derive(Clone, PartialEq)]
pub struct Elf {
    pub color: Color,
    // pub position: (i32, i32),
    pub direction: Direction,
    pub is_end: bool,
}

#[derive(Clone, PartialEq)]
pub enum Color {
    Yellow,
    Blue,
    Green,
    All, // All means contains all colors
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
pub struct EndPoint {
    pub color: Color,
    pub has_elf: bool,
}

#[derive(Clone)]
pub struct Checkerboard {
    pub board: Vec<Vec<Element>>,
}

#[derive(Clone)]
pub struct RoadSign {
    pub direction: Vec<(Direction, Color)>,
}

impl Checkerboard {
    pub fn new(width: usize, height: usize) -> Checkerboard {
        Checkerboard {
            board: vec![vec![Element::Empty; width]; height],
        }
    }

    pub fn set_element(&mut self, position: (usize, usize), element: Element) {
        self.board[position.0][position.1] = element;
    }

    pub fn get_element(&self, position: (usize, usize)) -> &Element {
        &self.board[position.0][position.1]
    }

    pub fn get_element_mut(&mut self, position: (usize, usize)) -> &mut Element {
        &mut self.board[position.0][position.1]
    }
}

pub fn solve_game(_init_board: Vec<Vec<&str>>, _init_road_signs: Vec<String>) -> Checkerboard {
    // let (mut checkerboard, road_signs) = init_checkerboard(init_board, init_road_signs);

    // let mut correct_board: Checkerboard;
    // try all possible road signs
    todo!();
    // correct_board
}

pub fn print_board(checkerboard: &Checkerboard) {
    for _j in 0..checkerboard.board[0].len() {
        print!("==");
    }
    println!();
    for i in 0..checkerboard.board.len() {
        for j in 0..checkerboard.board[0].len() {
            match checkerboard.board[i][j] {
                Element::Empty => print!("  "),
                Element::Elf(_) => print!("E "),
                Element::EndPoint(_) => print!("P "),
                Element::Tree => print!("T "),
                Element::RoadSign(_) => print!("R "),
            }
        }
        println!();
    }
    for _j in 0..checkerboard.board[0].len() {
        print!("==");
    }
    println!();
}

pub fn init_checkerboard(
    init_board: Vec<Vec<&str>>,
    init_road_signs: Vec<String>,
) -> (Checkerboard, Vec<RoadSign>) {
    let mut checkerboard: Checkerboard = Checkerboard::new(init_board[0].len(), init_board.len());
    let mut road_signs: Vec<RoadSign> = Vec::new();
    for i in 0..init_board.len() {
        for j in 0..init_board[0].len() {
            let element: Element;
            if init_board[i][j].starts_with("E_") {
                let color = match init_board[i][j] {
                    "E_Y" => Color::Yellow,
                    "E_B" => Color::Blue,
                    "E_G" => Color::Green,
                    _ => Color::All,
                };
                element = Element::Elf(Elf {
                    color,
                    // position: (i as i32, j as i32),
                    direction: Direction::Right,
                    is_end: false,
                });
            } else {
                element = match init_board[i][j] {
                    "Y_Point" => Element::EndPoint(EndPoint {
                        color: Color::Yellow,
                        has_elf: false,
                    }),
                    "B_Point" => Element::EndPoint(EndPoint {
                        color: Color::Blue,
                        has_elf: false,
                    }),
                    "G_Point" => Element::EndPoint(EndPoint {
                        color: Color::Green,
                        has_elf: false,
                    }),
                    "T" => Element::Tree,
                    // "R" => Element::RoadSign,
                    _ => Element::Empty,
                };
            }
            checkerboard.set_element((i, j), element);
        }
    }

    for road_sign in init_road_signs {
        let directions: Vec<&str> = road_sign.split(' ').collect();
        let mut direction = Vec::new();
        for d in directions {
            let color = match d.chars().nth(0).unwrap() {
                'Y' => Color::Yellow,
                'B' => Color::Blue,
                'G' => Color::Green,
                _ => Color::All,
            };
            let dir = match d.chars().nth(1).unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => Direction::Right,
            };
            direction.push((dir, color));
        }
        road_signs.push(RoadSign { direction });
    }

    (checkerboard, road_signs)
}

pub fn verify_game(checkerboard: &Checkerboard) -> bool {
    let height = checkerboard.board.len();
    let width = checkerboard.board[0].len();

    let mut checkerboard = checkerboard.clone();
    let mut checkerboard_step1 = Checkerboard::new(width, height);
    let mut checkerboard_step2 = Checkerboard::new(width, height);

    // remove elfs from board2

    for i in 0..height {
        for j in 0..width {
            match &checkerboard.board[i][j] {
                Element::Elf(elf) => {
                    checkerboard_step1.set_element((i, j), Element::Elf(elf.clone()));
                    checkerboard.set_element((i, j), Element::Empty);
                }
                _ => (),
            }
        }
    }

    // print_board(&checkerboard);
    // print_board(&checkerboard_step1);
    // print_board(&checkerboard_step2);

    let mut not_end = true;
    let mut iter_num = 20;

    while not_end && iter_num > 0 {
        not_end = false;
        for i in 0..height {
            for j in 0..width {
                let next_hop: (usize, usize);
                match &checkerboard_step1.board[i][j] {
                    Element::Elf(elf) => {
                        match elf.direction {
                            Direction::Up => {
                                if i == 0 {
                                    return false;
                                } else {
                                    next_hop = (i - 1, j);
                                }
                            }
                            Direction::Down => {
                                if i == height - 1 {
                                    return false;
                                } else {
                                    next_hop = (i + 1, j);
                                }
                            }
                            Direction::Left => {
                                if j == 0 {
                                    return false;
                                } else {
                                    next_hop = (i, j - 1);
                                }
                            }
                            Direction::Right => {
                                if j == width - 1 {
                                    return false;
                                } else {
                                    next_hop = (i, j + 1);
                                }
                            }
                        }

                        // if the upper cell is elf, return false
                        match checkerboard_step2.board[next_hop.0][next_hop.1] {
                            Element::Elf(_) => return false,
                            _ => (),
                        }
                        match &checkerboard.board[next_hop.0][next_hop.1] {
                            Element::Empty => {
                                checkerboard_step2.set_element(
                                    (next_hop.0, next_hop.1),
                                    Element::Elf(elf.clone()),
                                );
                                not_end = true;
                            }
                            Element::EndPoint(end_point) => {
                                if end_point.color != elf.color {
                                    return false;
                                } else if end_point.has_elf {
                                    return false;
                                } else {
                                    checkerboard.set_element(
                                        (next_hop.0, next_hop.1),
                                        Element::EndPoint(EndPoint {
                                            color: end_point.color.clone(),
                                            has_elf: true,
                                        }),
                                    );
                                }
                            }
                            Element::Tree => return false,
                            Element::RoadSign(road_signs) => {
                                // check road sign
                                let mut road_sign = None;
                                for sign in road_signs.direction.iter() {
                                    if sign.1 == elf.color {
                                        road_sign = Some(sign);
                                    }
                                }
                                match road_sign {
                                    Some((dir, color)) => {
                                        let elf = Elf {
                                            color: color.clone(),
                                            // position: (i as i32, j as i32),
                                            direction: dir.clone(),
                                            is_end: false,
                                        };
                                        checkerboard_step2.set_element(
                                            (next_hop.0, next_hop.1),
                                            Element::Elf(elf),
                                        );
                                    }
                                    None => {
                                        checkerboard_step2.set_element(
                                            (next_hop.0, next_hop.1),
                                            Element::Elf(elf.clone()),
                                        );
                                    }
                                }
                                not_end = true;
                            }
                            _ => return false,
                        }
                    }
                    _ => (),
                }
            }
        }
        checkerboard_step1 = checkerboard_step2.clone();
        checkerboard_step2 = Checkerboard::new(width, height);
        iter_num -= 1;
        // println!("iter_num: {}", iter_num);
        // print_board(&checkerboard_step1);
    }

    if iter_num == 0 {
        return false;
    } else {
        return true;
    }
}
