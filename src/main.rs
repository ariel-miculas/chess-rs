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
        } else {
            if self.board.squares[pos.y][pos.x].is_some() {
                self.prev_clicked_pos = Some(pos);
            }
        }
    }

    fn get_ui_pos(&self, pos: Position) -> Rect {
        Rect {
            min: Pos2 {
                x: (pos.x * CHESS_SQUARE_SIZE) as f32,
                y: ((7 - pos.y) * CHESS_SQUARE_SIZE) as f32,
            },
            max: Pos2 {
                x: ((pos.x * CHESS_SQUARE_SIZE) + CHESS_SQUARE_SIZE) as f32,
                y: (((7 - pos.y) * CHESS_SQUARE_SIZE) + CHESS_SQUARE_SIZE) as f32,
            },
        }
    }

    fn get_bg_color(&self, pos: Position) -> Color32 {
        if pos.y % 2 == 0 {
            BOARD_COLORS[pos.x % 2]
        } else {
            BOARD_COLORS[1 - pos.x % 2]
        }
    }
}

impl eframe::App for GuiBoard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for y in 0..8 {
                for x in 0..8 {
                    let mut button = egui::Button::new(
                        match self.board.squares[y][x] {
                            Some(ref square) => square.draw_piece(),
                            _ => ' ',
                        }
                        .to_string(),
                    );
                    let mut bg_color = self.get_bg_color(Position { x, y });

                    if let Some(clicked_pos) = self.prev_clicked_pos {
                        if clicked_pos == (Position { x, y }) {
                            bg_color = Color32::GREEN;
                        }
                    }

                    button = button.fill(bg_color);

                    let resp = ui.put(self.get_ui_pos(Position { x, y }), button);
                    if resp.clicked() {
                        self.handle_clicked(Position { x, y });
                        println!("clicked {} {}", y, x);
                    }
                }
            }
        });
    }
}
