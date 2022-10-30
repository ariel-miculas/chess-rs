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
    pub squares: [[Option<Box<dyn ChessPiece>>; 8]; 8],
}

#[derive(Debug)]
pub struct Position {
    x: usize,
    y: usize,
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

pub trait ChessPiece: Debug {
    fn get_available_moves(&mut self, board: Board, pos: Position) -> Result<Vec<Position>>;
    // fn can_move(&mut self, initial_pos: Position, final_pos: Position) -> bool;
    fn draw_piece(&self) -> char;
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
        first_row[0] = Some(Box::new(Rook {
            piece: Piece::new(Color::White),
        }));
        first_row[1] = Some(Box::new(Knight {
            piece: Piece::new(Color::White),
        }));
        first_row[2] = Some(Box::new(Bishop {
            piece: Piece::new(Color::White),
        }));
        first_row[3] = Some(Box::new(Queen {
            piece: Piece::new(Color::White),
        }));
        first_row[4] = Some(Box::new(King {
            piece: Piece::new(Color::White),
        }));
        first_row[5] = Some(Box::new(Bishop {
            piece: Piece::new(Color::White),
        }));
        first_row[6] = Some(Box::new(Knight {
            piece: Piece::new(Color::White),
        }));
        first_row[7] = Some(Box::new(Rook {
            piece: Piece::new(Color::White),
        }));

        for square in &mut self.squares[1] {
            *square = Some(Box::new(Pawn {
                piece: Piece::new(Color::White),
            }));
        }

        for square in &mut self.squares[6] {
            *square = Some(Box::new(Pawn {
                piece: Piece::new(Color::Black),
            }));
        }

        let last_row = &mut self.squares[7];
        last_row[0] = Some(Box::new(Rook {
            piece: Piece::new(Color::Black),
        }));
        last_row[1] = Some(Box::new(Knight {
            piece: Piece::new(Color::Black),
        }));
        last_row[2] = Some(Box::new(Bishop {
            piece: Piece::new(Color::Black),
        }));
        last_row[3] = Some(Box::new(Queen {
            piece: Piece::new(Color::Black),
        }));
        last_row[4] = Some(Box::new(King {
            piece: Piece::new(Color::Black),
        }));
        last_row[5] = Some(Box::new(Bishop {
            piece: Piece::new(Color::Black),
        }));
        last_row[6] = Some(Box::new(Knight {
            piece: Piece::new(Color::Black),
        }));
        last_row[7] = Some(Box::new(Rook {
            piece: Piece::new(Color::Black),
        }));
    }

    pub fn add_piece(&mut self, piece: Box<dyn ChessPiece>, pos: Position) -> Result<()> {
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

impl ChessPiece for Pawn {
    fn get_available_moves(&mut self, board: Board, pos: Position) -> Result<Vec<Position>> {
        println!("moving Pawn");
        Err(MoveError)
    }
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2659}',
            Color::Black => '\u{265F}',
        }
    }
}

impl ChessPiece for Knight {
    fn get_available_moves(&mut self, board: Board, pos: Position) -> Result<Vec<Position>> {
        println!("moving Knight");
        Err(MoveError)
    }
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2658}',
            Color::Black => '\u{265E}',
        }
    }
}

impl ChessPiece for Bishop {
    fn get_available_moves(&mut self, board: Board, pos: Position) -> Result<Vec<Position>> {
        println!("moving Bishop");
        Err(MoveError)
    }
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2657}',
            Color::Black => '\u{265D}',
        }
    }
}

impl ChessPiece for Rook {
    fn get_available_moves(&mut self, board: Board, pos: Position) -> Result<Vec<Position>> {
        println!("moving Rook");
        Err(MoveError)
    }
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2656}',
            Color::Black => '\u{265C}',
        }
    }
}

impl ChessPiece for King {
    fn get_available_moves(&mut self, board: Board, pos: Position) -> Result<Vec<Position>> {
        println!("moving King");
        Err(MoveError)
    }
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2654}',
            Color::Black => '\u{265A}',
        }
    }
}

impl ChessPiece for Queen {
    fn get_available_moves(&mut self, board: Board, pos: Position) -> Result<Vec<Position>> {
        println!("moving Queen");
        Err(MoveError)
    }
    fn draw_piece(&self) -> char {
        match self.piece.color {
            Color::White => '\u{2655}',
            Color::Black => '\u{265B}',
        }
    }
}
