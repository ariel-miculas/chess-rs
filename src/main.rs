use chess_game::{Board, Position};
// use chess_game::{Bishop, King, Knight, Pawn, Queen, Rook};

use eframe::egui;
use eframe::egui::Color32;
use egui::{Pos2, Rect};

fn main() {
    let gui_board = GuiBoard::new_game();

    println!("{}", gui_board.board);

    let options = eframe::NativeOptions::default();
    eframe::run_native("Chess game", options, Box::new(|_cc| Box::new(gui_board)));
}

const CHESS_SQUARE_SIZE: usize = 40;
const LIGHT_BROWN: Color32 = Color32::from_rgb(239, 218, 180);
const DARK_BROWN: Color32 = Color32::from_rgb(178, 134, 101);
const BOARD_COLORS: [Color32; 2] = [LIGHT_BROWN, DARK_BROWN];

#[derive(Default)]
struct GuiBoard {
    pub board: Board,
    prev_clicked_pos: Option<Position>,
    available_positions: Vec<Position>,
}

impl GuiBoard {
    fn new_game() -> Self {
        GuiBoard {
            board: Board::new_game(),
            ..Default::default()
        }
    }

    fn handle_clicked(&mut self, pos: Position) {
        if let Some(prev_clicked_pos) = self.prev_clicked_pos {
            println!("prev clicked was: {:?}", prev_clicked_pos);
            self.prev_clicked_pos = None;
            self.available_positions.clear();
        } else {
            if let Some(ref _piece) = self.board.squares[pos.get_row()][pos.get_column()] {
                self.prev_clicked_pos = Some(pos);
                self.available_positions = self.board.get_available_moves(pos);
            }
        }
    }

    fn get_ui_pos(&self, pos: Position) -> Rect {
        Rect {
            min: Pos2 {
                x: (pos.get_column() * CHESS_SQUARE_SIZE) as f32,
                y: ((7 - pos.get_row()) * CHESS_SQUARE_SIZE) as f32,
            },
            max: Pos2 {
                x: ((pos.get_column() * CHESS_SQUARE_SIZE) + CHESS_SQUARE_SIZE) as f32,
                y: (((7 - pos.get_row()) * CHESS_SQUARE_SIZE) + CHESS_SQUARE_SIZE) as f32,
            },
        }
    }

    fn get_bg_color(&self, pos: Position) -> Color32 {
        if pos.get_row() % 2 == 0 {
            BOARD_COLORS[pos.get_column() % 2]
        } else {
            BOARD_COLORS[1 - pos.get_column() % 2]
        }
    }
}

impl eframe::App for GuiBoard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for row in 0..8 {
                for column in 0..8 {
                    let mut button = egui::Button::new(
                        match self.board.squares[row][column] {
                            Some(ref square) => square.draw_piece(),
                            _ => ' ',
                        }
                        .to_string(),
                    );
                    let mut bg_color = self.get_bg_color(Position::try_new(row, column).unwrap());

                    if self
                        .available_positions
                        .contains(&Position::try_new(row, column).unwrap())
                    {
                        bg_color = Color32::LIGHT_GREEN;
                    }

                    if let Some(clicked_pos) = self.prev_clicked_pos {
                        if clicked_pos == (Position::try_new(row, column).unwrap()) {
                            bg_color = Color32::GREEN;
                        }
                    }

                    button = button.fill(bg_color);

                    let resp = ui.put(
                        self.get_ui_pos(Position::try_new(row, column).unwrap()),
                        button,
                    );
                    if resp.clicked() {
                        self.handle_clicked(Position::try_new(row, column).unwrap());
                        println!("clicked {} {}", row, column);
                    }
                }
            }
        });
    }
}
