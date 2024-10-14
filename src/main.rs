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
use smol::channel::Sender;
use std::fs::read_to_string;
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
    let template = read_to_string("src/templates/index.aml").unwrap();

    let doc = Document::new(template);

    let (tx, rx) = smol::channel::unbounded::<GlobalStateManagementMessage>();
    let game_loop = GameLoop::new(10, 20, tx.clone());

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    let main_menu_id = runtime
        .register_component(
            "MainMenu",
            "src/templates/main_menu.aml",
            MainMenuComponent::new(tx.clone()),
            MainMenuComponentState::new(),
        )
        .unwrap();

    let game_id = runtime
        .register_component(
            "Game",
            "src/templates/game.aml",
            GameComponent {},
            GameComponentState::new(),
        )
        .unwrap();

    let score_board_id = runtime
        .register_component(
            "ScoreBoard",
            "src/templates/scoreboard.aml",
            ScoreBoardComponent {},
            ScoreBoardComponentState::new(),
        )
        .unwrap();

    let next_piece_id = runtime
        .register_component(
            "NextPiece",
            "src/templates/next_piece.aml",
            NextPieceComponent {},
            NextPieceState::new(),
        )
        .unwrap();

    let statistics_id = runtime
        .register_component(
            "Statistics",
            "src/templates/statistics.aml",
            StatisticsComponent {},
            StatisticsState::new(),
        )
        .unwrap();

    runtime
        .register_prototype(
            "Statistic",
            "src/templates/statistic.aml",
            || StatisticComponent {},
            StatisticComponentState::new,
        )
        .unwrap();

    let lines_count_id = runtime
        .register_component(
            "LineCount",
            "src/templates/line_count.aml",
            LineCountComponent {},
            LineCountState::new(),
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

    let game_arena_id = runtime
        .register_component(
            "GameArena",
            "src/templates/game_arena.aml",
            GameArenaComponent::new(tx.clone(), game_loop),
            GameArenaComponentState::new(),
        )
        .unwrap();

    runtime
        .register_prototype(
            "StaticPiece",
            "src/templates/static_piece.aml",
            || StaticPieceComponent {},
            || StaticPieceState {},
        )
        .unwrap();

    let game_over_id = runtime
        .register_component(
            "GameOver",
            "src/templates/game_over.aml",
            GameOverComponent::new(tx.clone()),
            GameOverComponentState::new(),
        )
        .unwrap();

    let _paused_id = runtime
        .register_component("Paused", "src/templates/paused.aml", (), ())
        .unwrap();

    let runtime = runtime.set_global_event_handler(GlobalEventHandler::new(tx.clone()));

    let emitter = runtime.emitter().clone();
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

    global_state::start(emitter, tx, rx, component_ids);
    runtime.finish().unwrap().run();
}

struct GlobalEventHandler {
    tx: Sender<GlobalStateManagementMessage>,
}

// All key events are simply passed on to the global global handler
impl GlobalEvents for GlobalEventHandler {
    fn handle(
        &mut self,
        event: anathema::component::Event,
        _elements: &mut anathema::widgets::Elements<'_, '_>,
        _ctx: &mut GlobalContext<'_>,
    ) -> Option<anathema::component::Event> {
        if let Some(exit) = self.check_for_exit(&event) {
            return Some(exit);
        }
        let _ = self.tx.try_send(GlobalStateManagementMessage::Event(event));
        None
    }
}

impl GlobalEventHandler {
    fn new(tx: Sender<GlobalStateManagementMessage>) -> Self {
        Self { tx }
    }

    fn check_for_exit(
        &self,
        event: &anathema::component::Event,
    ) -> Option<anathema::component::Event> {
        if let anathema::component::Event::Key(key_event) = event {
            return match key_event {
                KeyEvent {
                    code: KeyCode::Char('c'),
                    ctrl: true,
                    state: KeyState::Press,
                } => Some(anathema::component::Event::Stop),
                _ => None,
            };
        }
        None
    }
}
