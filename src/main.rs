mod widgets;

use anathema::prelude::*;
use std::fs::read_to_string;
use widgets::{
    game::{GameComponent, GameState},
    game_type::{GameTypeComponent, GameTypeState},
    line_count::{LineCountComponent, LineCountState},
    next_piece::{NextPieceComponent, NextPieceState},
    scoreboard::{ScoreBoardComponent, ScoreBoardState},
    statistics::{StatisticsComponent, StatisticsState},
};

fn main() {
    let template = read_to_string("src/templates/index.aml").unwrap();

    let doc = Document::new(template);

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    runtime
        .register_component(
            "Game",
            "src/templates/game.aml",
            GameComponent {},
            GameState {},
        )
        .unwrap();

    runtime
        .register_component(
            "ScoreBoard",
            "src/templates/scoreboard.aml",
            ScoreBoardComponent {},
            ScoreBoardState {},
        )
        .unwrap();

    runtime
        .register_component(
            "NextPiece",
            "src/templates/next_piece.aml",
            NextPieceComponent {},
            NextPieceState {},
        )
        .unwrap();

    runtime
        .register_component(
            "Statistics",
            "src/templates/statistics.aml",
            StatisticsComponent {},
            StatisticsState {},
        )
        .unwrap();

    runtime
        .register_component(
            "LineCount",
            "src/templates/line_count.aml",
            LineCountComponent {},
            LineCountState {},
        )
        .unwrap();

    runtime
        .register_component(
            "GameType",
            "src/templates/game_type.aml",
            GameTypeComponent {},
            GameTypeState {},
        )
        .unwrap();

    runtime.finish().unwrap().run();
}
