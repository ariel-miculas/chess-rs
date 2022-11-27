use std::{fmt, fmt::Debug};

type Result<T> = std::result::Result<T, MoveError>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub struct MoveError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid move")
    }
}

#[derive(Default)]
pub struct Board {
    pub squares: [[Option<ChessPiece>; 8]; 8],
}

mod position;
pub use position::Position;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub enum Color {
    Black,
    #[default]
    White,
}

impl Color {
    pub fn switch(&mut self) {
        if *self == Color::Black {
            *self = Color::White
        } else {
            *self = Color::Black
        }
    }
}

#[derive(Debug)]
pub struct Pawn;

#[derive(Debug)]
pub struct Knight;

#[derive(Debug)]
pub struct Queen;

#[derive(Debug)]
pub struct King;

#[derive(Debug)]
pub struct Rook;

#[derive(Debug)]
pub struct Bishop;

#[derive(Debug)]
struct Piece;

#[derive(Debug)]
pub enum ChessPieceType {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

pub struct ChessPiece {
    pub color: Color,
    chess_piece: ChessPieceType,
}

const WHITE_PAWN_ROW: usize = 1;
const BLACK_PAWN_ROW: usize = 6;
const LAST_ROW: usize = 7;

impl ChessPiece {
    pub fn new(chess_piece: ChessPieceType, color: Color) -> Self {
        ChessPiece { color, chess_piece }
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: Default::default(),
        }
    }

    pub fn new_game() -> Board {
        let mut board = Self::new();
        board.init_board();
        board
    }

    pub fn init_board(&mut self) {
        let first_row = &mut self.squares[0];
        first_row[0] = Some(ChessPiece::new(ChessPieceType::Rook(Rook), Color::White));
        first_row[1] = Some(ChessPiece::new(
            ChessPieceType::Knight(Knight),
            Color::White,
        ));
        first_row[2] = Some(ChessPiece::new(
            ChessPieceType::Bishop(Bishop),
            Color::White,
        ));
        first_row[3] = Some(ChessPiece::new(ChessPieceType::Queen(Queen), Color::White));
        first_row[4] = Some(ChessPiece::new(ChessPieceType::King(King), Color::White));
        first_row[5] = Some(ChessPiece::new(
            ChessPieceType::Bishop(Bishop),
            Color::White,
        ));
        first_row[6] = Some(ChessPiece::new(
            ChessPieceType::Knight(Knight),
            Color::White,
        ));
        first_row[7] = Some(ChessPiece::new(ChessPieceType::Rook(Rook), Color::White));

        for square in &mut self.squares[1] {
            *square = Some(ChessPiece::new(ChessPieceType::Pawn(Pawn), Color::White));
        }

        for square in &mut self.squares[6] {
            *square = Some(ChessPiece::new(ChessPieceType::Pawn(Pawn), Color::Black));
        }

        let last_row = &mut self.squares[LAST_ROW];
        last_row[0] = Some(ChessPiece::new(ChessPieceType::Rook(Rook), Color::Black));
        last_row[1] = Some(ChessPiece::new(
            ChessPieceType::Knight(Knight),
            Color::Black,
        ));
        last_row[2] = Some(ChessPiece::new(
            ChessPieceType::Bishop(Bishop),
            Color::Black,
        ));
        last_row[3] = Some(ChessPiece::new(ChessPieceType::Queen(Queen), Color::Black));
        last_row[4] = Some(ChessPiece::new(ChessPieceType::King(King), Color::Black));
        last_row[5] = Some(ChessPiece::new(
            ChessPieceType::Bishop(Bishop),
            Color::Black,
        ));
        last_row[6] = Some(ChessPiece::new(
            ChessPieceType::Knight(Knight),
            Color::Black,
        ));
        last_row[7] = Some(ChessPiece::new(ChessPieceType::Rook(Rook), Color::Black));
    }

    pub fn get_piece(&self, pos: Position) -> &Option<ChessPiece> {
        &self.squares[pos.get_row()][pos.get_column()]
    }

    pub fn add_piece(&mut self, piece: ChessPiece, pos: Position) -> Result<()> {
        self.squares[pos.get_row()][pos.get_column()] = Some(piece);
        Ok(())
    }

    pub fn move_piece(&mut self, initial_position: Position, final_position: Position) {
        self.squares[final_position.get_row()][final_position.get_column()] =
            self.squares[initial_position.get_row()][initial_position.get_column()].take();
    }

    fn get_orthogonal_moves(&self, piece: &ChessPiece, pos: Position) -> Vec<Position> {
        let mut available_moves = Vec::new();
        for square in pos.get_left_squares() {
            match self.get_piece(square) {
                Some(p) => {
                    if p.color != piece.color {
                        available_moves.push(square)
                    }
                    break;
                }
                None => available_moves.push(square),
            }
        }
        for square in pos.get_right_squares() {
            match self.get_piece(square) {
                Some(p) => {
                    if p.color != piece.color {
                        available_moves.push(square)
                    }
                    break;
                }
                None => available_moves.push(square),
            }
        }
        for square in pos.get_up_squares() {
            match self.get_piece(square) {
                Some(p) => {
                    if p.color != piece.color {
                        available_moves.push(square)
                    }
                    break;
                }
                None => available_moves.push(square),
            }
        }
        for square in pos.get_down_squares() {
            match self.get_piece(square) {
                Some(p) => {
                    if p.color != piece.color {
                        available_moves.push(square)
                    }
                    break;
                }
                None => available_moves.push(square),
            }
        }
        available_moves
    }

    fn get_diagonal_moves(&self, piece: &ChessPiece, pos: Position) -> Vec<Position> {
        let mut available_moves = Vec::new();
        for square in pos.get_principal_diagonal_up_squares() {
            match self.get_piece(square) {
                Some(p) => {
                    if p.color != piece.color {
                        available_moves.push(square)
                    }
                    break;
                }
                None => available_moves.push(square),
            }
        }
        for square in pos.get_principal_diagonal_down_squares() {
            match self.get_piece(square) {
                Some(p) => {
                    if p.color != piece.color {
                        available_moves.push(square)
                    }
                    break;
                }
                None => available_moves.push(square),
            }
        }
        for square in pos.get_secondary_diagonal_up_squares() {
            match self.get_piece(square) {
                Some(p) => {
                    if p.color != piece.color {
                        available_moves.push(square)
                    }
                    break;
                }
                None => available_moves.push(square),
            }
        }
        for square in pos.get_secondary_diagonal_down_squares() {
            match self.get_piece(square) {
                Some(p) => {
                    if p.color != piece.color {
                        available_moves.push(square)
                    }
                    break;
                }
                None => available_moves.push(square),
            }
        }
        available_moves
    }

    pub fn get_available_moves(&self, pos: Position) -> Vec<Position> {
        let mut available_moves = Vec::<Position>::new();
        fn filter_same_color_collision(chess_piece: &Option<ChessPiece>, col: Color) -> bool {
            match chess_piece {
                Some(piece) => piece.color != col,
                None => true,
            }
        }

        if let Some(piece) = self.get_piece(pos) {
            match &piece.chess_piece {
                ChessPieceType::Pawn(p) => {
                    if let Some(x) = p
                        .move_up(pos, 1, piece.color)
                        .filter(|x| self.get_piece(*x).is_none())
                    {
                        available_moves.push(x)
                    }

                    if p.get_starting_row(piece.color) == pos.get_row() {
                        if let Some(x) = p
                            .move_up(pos, 2, piece.color)
                            .filter(|x| self.get_piece(*x).is_none())
                        {
                            available_moves.push(x)
                        }
                    }

                    available_moves.extend(
                        p.get_attacking_squares(pos, piece.color)
                            .into_iter()
                            .filter_map(|pos| {
                                self.get_piece(pos).as_ref().map(|p| {
                                    if p.color != piece.color {
                                        Some(pos)
                                    } else {
                                        None
                                    }
                                })
                            })
                            .flatten()
                            .collect::<Vec<Position>>(),
                    );
                }
                ChessPieceType::Rook(_r) => {
                    available_moves.append(&mut self.get_orthogonal_moves(piece, pos));
                }
                ChessPieceType::Knight(_k) => {
                    let available_positions = vec![
                        (-2, -1),
                        (-2, 1),
                        (-1, -2),
                        (-1, 2),
                        (1, -2),
                        (1, 2),
                        (2, -1),
                        (2, 1),
                    ];

                    available_moves = available_positions
                        .iter()
                        .filter_map(|available_position| pos.try_add(*available_position).ok())
                        .filter(|x| filter_same_color_collision(self.get_piece(*x), piece.color))
                        .collect::<Vec<Position>>();
                }
                ChessPieceType::Bishop(_b) => {
                    available_moves.append(&mut self.get_diagonal_moves(piece, pos));
                }
                ChessPieceType::Queen(_q) => {
                    available_moves.append(&mut self.get_orthogonal_moves(piece, pos));
                    available_moves.append(&mut self.get_diagonal_moves(piece, pos));
                }
                ChessPieceType::King(_k) => {
                    for square in pos.get_surrounding_squares() {
                        match self.get_piece(square) {
                            Some(p) => {
                                if p.color != piece.color {
                                    available_moves.push(square)
                                }
                            }
                            None => available_moves.push(square),
                        }
                    }
                }
            }
        }
        available_moves
    }
}

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        for line in self.squares.iter().rev() {
            for column in line {
                match column {
                    Some(column) => write!(f, "{} ", column.draw_piece())?,
                    None => write!(f, "  ")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl ChessPiece {
    pub fn draw_piece(&self) -> char {
        match &self.chess_piece {
            ChessPieceType::Pawn(_p) => match self.color {
                Color::White => '\u{2659}',
                Color::Black => '\u{265F}',
            },
            ChessPieceType::Knight(_k) => match self.color {
                Color::White => '\u{2658}',
                Color::Black => '\u{265E}',
            },
            ChessPieceType::Bishop(_b) => match self.color {
                Color::White => '\u{2657}',
                Color::Black => '\u{265D}',
            },
            ChessPieceType::Rook(_r) => match self.color {
                Color::White => '\u{2656}',
                Color::Black => '\u{265C}',
            },
            ChessPieceType::King(_k) => match self.color {
                Color::White => '\u{2654}',
                Color::Black => '\u{265A}',
            },
            ChessPieceType::Queen(_q) => match self.color {
                Color::White => '\u{2655}',
                Color::Black => '\u{265B}',
            },
        }
    }
}

impl Pawn {
    fn get_starting_row(&self, color: Color) -> usize {
        if color == Color::White {
            WHITE_PAWN_ROW
        } else {
            BLACK_PAWN_ROW
        }
    }

    fn move_up(&self, pos: Position, distance: usize, color: Color) -> Option<Position> {
        if color == Color::White {
            Position::try_new(pos.get_row() + distance, pos.get_column()).ok()
        } else {
            Position::try_new(pos.get_row() - distance, pos.get_column()).ok()
        }
    }

    fn get_attacking_squares(&self, pos: Position, color: Color) -> Vec<Position> {
        let mut attacking_squares = Vec::new();
        if color == Color::White {
            if let Some(pos) = pos.get_principal_diagonal_up_squares().get(0) {
                attacking_squares.push(*pos);
            }
            if let Some(pos) = pos.get_secondary_diagonal_up_squares().get(0) {
                attacking_squares.push(*pos);
            }
        } else {
            if let Some(pos) = pos.get_principal_diagonal_down_squares().get(0) {
                attacking_squares.push(*pos);
            }
            if let Some(pos) = pos.get_secondary_diagonal_down_squares().get(0) {
                attacking_squares.push(*pos);
            }
        }
        attacking_squares
    }
}
