mod core;
mod widgets;

use anathema::{
    component::{KeyCode, KeyEvent},
    prelude::*,
    widgets::components::events::KeyState,
};
use core::{
    game_loop::GameLoop,
    global_state::{self, GameStateComponentIds, GlobalStateManagementMessage},
};
use anathema::component::Event;
use widgets::{
    game::{GameComponent, GameComponentState},
    game_arena::{GameArenaComponent, GameArenaComponentState},
    game_over::{GameOverComponent, GameOverComponentState},
    game_type::{GameTypeComponent, GameTypeState},
    line_count::{LineCountComponent, LineCountState},
    main_menu::{MainMenuComponent, MainMenuComponentState},
    next_piece::{NextPieceComponent, NextPieceState},
    scoreboard::{ScoreBoardComponent, ScoreBoardComponentState},
    static_piece::{StaticPieceComponent, StaticPieceState},
    statistic::{StatisticComponent, StatisticComponentState},
    statistics::{StatisticsComponent, StatisticsState},
};

fn main() {
    // let template = read_to_string("src/templates/index.aml").unwrap();


    let (tx, rx) = smol::channel::unbounded::<GlobalStateManagementMessage>();
    let game_loop = GameLoop::new(10, 20, tx.clone());

    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let doc = Document::new("@index");
    let mut builder = Runtime::builder(doc, &backend)
        .with_global_event_handler(|event, _tabindex, _components| {
            if let Some(exit) = check_for_exit(&event) {
                return Some(exit);
            }
            let _ = tx.try_send(GlobalStateManagementMessage::Event(event));
            None
    });
    builder.default::<()>("index", "src/templates/index.aml").unwrap();

    let main_menu_id = builder
        .component(
            "MainMenu",
            "src/templates/main_menu.aml",
            MainMenuComponent::new(tx.clone()),
            MainMenuComponentState::new(),
        )
        .unwrap();

    let game_id = builder
        .component(
            "Game",
            "src/templates/game.aml",
            GameComponent {},
            GameComponentState::new(),
        )
        .unwrap();

    let score_board_id = builder
        .component(
            "ScoreBoard",
            "src/templates/scoreboard.aml",
            ScoreBoardComponent {},
            ScoreBoardComponentState::new(),
        )
        .unwrap();

    let next_piece_id = builder
        .component(
            "NextPiece",
            "src/templates/next_piece.aml",
            NextPieceComponent {},
            NextPieceState::new(),
        )
        .unwrap();

    let statistics_id = builder
        .component(
            "Statistics",
            "src/templates/statistics.aml",
            StatisticsComponent {},
            StatisticsState::new(),
        )
        .unwrap();

    builder
        .prototype(
            "Statistic",
            "src/templates/statistic.aml",
            || StatisticComponent {},
            StatisticComponentState::new,
        )
        .unwrap();

    let lines_count_id = builder
        .component(
            "LineCount",
            "src/templates/line_count.aml",
            LineCountComponent {},
            LineCountState::new(),
        )
        .unwrap();

    builder
        .component(
            "GameType",
            "src/templates/game_type.aml",
            GameTypeComponent {},
            GameTypeState {},
        )
        .unwrap();

    let game_arena_id = builder
        .component(
            "GameArena",
            "src/templates/game_arena.aml",
            GameArenaComponent::new(tx.clone(), game_loop),
            GameArenaComponentState::new(),
        )
        .unwrap();

    builder
        .prototype(
            "StaticPiece",
            "src/templates/static_piece.aml",
            || StaticPieceComponent {},
            || StaticPieceState {},
        )
        .unwrap();

    let game_over_id = builder
        .component(
            "GameOver",
            "src/templates/game_over.aml",
            GameOverComponent::new(tx.clone()),
            GameOverComponentState::new(),
        )
        .unwrap();

    let _paused_id = builder
        .component("Paused", "src/templates/paused.aml", (), ())
        .unwrap();

    let emitter = builder.emitter().clone();
    let component_ids = GameStateComponentIds::new(
        main_menu_id,
        game_id,
        game_arena_id,
        game_over_id,
        score_board_id,
        lines_count_id,
        next_piece_id,
        statistics_id,
    );

    global_state::start(emitter, tx.clone(), rx, component_ids);
    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .unwrap();
}

fn check_for_exit(
    event: &Event,
) -> Option<Event> {
    if let Event::Key(key_event) = event {
        return match key_event {
            KeyEvent {
                code: KeyCode::Char('c'),
                ctrl: true,
                state: KeyState::Press,
            } => Some(Event::Stop),
            _ => None,
        };
    }
    None
}
