use chess_game::Board;
// use chess_game::{Bishop, King, Knight, Pawn, Queen, Rook};

use eframe::egui;
use eframe::egui::{Color32, Rgba};
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
}

impl GuiBoard {
    fn new_game() -> Self {
        let mut gui_board = Self::default();
        gui_board.board = Board::new_game();
        gui_board
    }
}

impl eframe::App for GuiBoard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for y in 0..8 {
                for x in 0..8 {
                    let button = egui::Button::new(
                        match self.board.squares[y][x] {
                            Some(ref square) => square.draw_piece(),
                            _ => ' ',
                        }
                        .to_string(),
                    );

                    let button = if y % 2 == 0 {
                        button.fill(BOARD_COLORS[x % 2])
                    } else {
                        button.fill(BOARD_COLORS[1 - x % 2])
                    };

                    let resp = ui.put(
                        Rect {
                            min: Pos2 {
                                x: ((7 - x) * CHESS_SQUARE_SIZE) as f32,
                                y: ((7 - y) * CHESS_SQUARE_SIZE) as f32,
                            },
                            max: Pos2 {
                                x: (((7 - x) * CHESS_SQUARE_SIZE) + CHESS_SQUARE_SIZE) as f32,
                                y: (((7 - y) * CHESS_SQUARE_SIZE) + CHESS_SQUARE_SIZE) as f32,
                            },
                        },
                        button,
                    );
                    if resp.clicked() {
                        println!("clicked {:?}", resp.id);
                    }
                }
            }
        });
    }
}
