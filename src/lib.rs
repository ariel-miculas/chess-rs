use std::{fmt, fmt::Debug};
use enum_dispatch::enum_dispatch;

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

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug)]
pub struct Pawn {
    piece: Piece,
}

#[derive(Debug)]
pub struct Knight {
    piece: Piece,
}

#[derive(Debug)]
pub struct Queen {
    piece: Piece,
}

#[derive(Debug)]
pub struct King {
    piece: Piece,
}

#[derive(Debug)]
pub struct Rook {
    piece: Piece,
}

#[derive(Debug)]
pub struct Bishop {
    piece: Piece,
}

#[derive(Debug)]
struct Piece {
    color: Color,
}

#[enum_dispatch]
pub trait ChessPieceTrait: Debug {
    // fn can_move(&mut self, initial_pos: Position, final_pos: Position) -> bool;
    fn draw_piece(&self) -> char;
}

#[enum_dispatch(ChessPieceTrait)]
#[derive(Debug)]
pub enum ChessPiece {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}


impl Piece {
    pub fn new(color: Color) -> Piece {
        Piece { color }
    }
}

impl Board {
    pub fn new() -> Board {
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
        first_row[0] = Some(ChessPiece::Rook(Rook {
            piece: Piece::new(Color::White),
        }));
        first_row[1] = Some(ChessPiece::Knight(Knight {
            piece: Piece::new(Color::White),
        }));
        first_row[2] = Some(ChessPiece::Bishop(Bishop {
            piece: Piece::new(Color::White),
        }));
        first_row[3] = Some(ChessPiece::Queen(Queen {
            piece: Piece::new(Color::White),
        }));
        first_row[4] = Some(ChessPiece::King(King {
            piece: Piece::new(Color::White),
        }));
        first_row[5] = Some(ChessPiece::Bishop(Bishop {
            piece: Piece::new(Color::White),
        }));
        first_row[6] = Some(ChessPiece::Knight(Knight {
            piece: Piece::new(Color::White),
        }));
        first_row[7] = Some(ChessPiece::Rook(Rook {
            piece: Piece::new(Color::White),
        }));

        for square in &mut self.squares[1] {
            *square = Some(ChessPiece::Pawn(Pawn {
                piece: Piece::new(Color::White),
            }));
        }

        for square in &mut self.squares[6] {
            *square = Some(ChessPiece::Pawn(Pawn {
                piece: Piece::new(Color::Black),
            }));
        }

        let last_row = &mut self.squares[7];
        last_row[0] = Some(ChessPiece::Rook(Rook {
            piece: Piece::new(Color::Black),
        }));
        last_row[1] = Some(ChessPiece::Knight(Knight {
            piece: Piece::new(Color::Black),
        }));
        last_row[2] = Some(ChessPiece::Bishop(Bishop {
            piece: Piece::new(Color::Black),
        }));
        last_row[3] = Some(ChessPiece::Queen(Queen {
            piece: Piece::new(Color::Black),
        }));
        last_row[4] = Some(ChessPiece::King(King {
            piece: Piece::new(Color::Black),
        }));
        last_row[5] = Some(ChessPiece::Bishop(Bishop {
            piece: Piece::new(Color::Black),
        }));
        last_row[6] = Some(ChessPiece::Knight(Knight {
            piece: Piece::new(Color::Black),
        }));
        last_row[7] = Some(ChessPiece::Rook(Rook {
            piece: Piece::new(Color::Black),
        }));
    }

    pub fn add_piece(&mut self, piece: ChessPiece, pos: Position) -> Result<()> {
        self.squares[pos.x][pos.y] = Some(piece);
        Ok(())
    }

    pub fn move_piece(
        &mut self,
        initial_position: Position,
        final_position: Position,
    ) -> Result<()> {
        self.squares[final_position.x][final_position.y] =
            self.squares[initial_position.x][initial_position.y].take();
        Ok(())
    }

    pub fn get_available_moves(&self, pos: Position) -> Vec<Position> {
        if let Some(ref _piece) = self.squares[pos.y][pos.x] {
            todo!("return possible moves!");
        } else {
            return Vec::new();
        }
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

impl ChessPieceTrait for Pawn {
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2659}',
            Color::Black => '\u{265F}',
        }
    }
}

impl ChessPieceTrait for Knight {
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2658}',
            Color::Black => '\u{265E}',
        }
    }
}

impl ChessPieceTrait for Bishop {
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2657}',
            Color::Black => '\u{265D}',
        }
    }
}

impl ChessPieceTrait for Rook {
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2656}',
            Color::Black => '\u{265C}',
        }
    }
}

impl ChessPieceTrait for King {
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2654}',
            Color::Black => '\u{265A}',
        }
    }
}

impl ChessPieceTrait for Queen {
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2655}',
            Color::Black => '\u{265B}',
        }
    }
}
