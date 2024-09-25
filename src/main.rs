mod widgets;

use anathema::{
    component::{ComponentId, Event, KeyCode, KeyEvent},
    prelude::*,
};
use std::{collections::HashMap, default, fs::read_to_string};
use widgets::{
    game::{GameComponent, GameComponentMessage, GameComponentState},
    game_arena::GameArenaComponent,
    game_type::{GameTypeComponent, GameTypeState},
    line_count::{LineCountComponent, LineCountState},
    main_menu::{self, MainMenuComponent, MainMenuComponentMessage, MainMenuComponentState},
    next_piece::{NextPieceComponent, NextPieceState},
    scoreboard::{ScoreBoardComponent, ScoreBoardState},
    static_piece::{StaticPieceComponent, StaticPieceState},
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

    let main_menu = runtime
        .register_component(
            "MainMenu",
            "src/templates/main_menu.aml",
            MainMenuComponent {},
            MainMenuComponentState::new(),
        )
        .unwrap();

    let game = runtime
        .register_component(
            "Game",
            "src/templates/game.aml",
            GameComponent {},
            GameComponentState::new(),
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

    runtime
        .register_component(
            "GameArena",
            "src/templates/game_arena.aml",
            GameArenaComponent::new(),
            (),
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

    let runtime = runtime.set_global_event_handler(GameStateManagement::new(&main_menu, &game));
    runtime.finish().unwrap().run();
}

#[derive(Default)]
enum GameState {
    #[default]
    MainMenu,
    Paused,
    Playing,
    GameOver,
}

struct GameStateManagement<'a> {
    state: GameState,
    main_menu: &'a ComponentId<MainMenuComponentMessage>,
    game: &'a ComponentId<GameComponentMessage>,
}

impl<'a> GlobalEvents for GameStateManagement<'a> {
    fn handle(
        &mut self,
        event: anathema::component::Event,
        _elements: &mut anathema::widgets::Elements<'_, '_>,
        ctx: &mut GlobalContext<'_>,
    ) -> Option<anathema::component::Event> {
        match self.state {
            GameState::MainMenu => self.handle_main_menu(event, ctx),
            GameState::Paused => self.handle_pause(event, ctx),
            GameState::Playing => self.handle_playing(event, ctx),
            GameState::GameOver => self.handle_game_over(event, ctx),
        }
    }
}

impl<'a> GameStateManagement<'a> {
    fn new(
        main_menu: &'a ComponentId<MainMenuComponentMessage>,
        game: &'a ComponentId<GameComponentMessage>,
    ) -> Self {
        Self {
            state: GameState::default(),
            main_menu,
            game,
        }
    }

    fn handle_main_menu(
        &self,
        event: anathema::component::Event,
        ctx: &mut GlobalContext<'_>,
    ) -> Option<anathema::component::Event> {
        match event {
            anathema::component::Event::Key(keyevent) => {
                let KeyEvent {
                    code,
                    ctrl: _,
                    state: _,
                } = keyevent;
                {
                    if let KeyCode::Enter = code {
                        ctx.emit(*self.main_menu, MainMenuComponentMessage::Invisible);
                        ctx.emit(*self.game, GameComponentMessage::Visible);
                    }
                    None
                }
            }
            _ => None,
        }
    }

    fn handle_pause(
        &self,
        event: anathema::component::Event,
        ctx: &mut GlobalContext,
    ) -> Option<anathema::component::Event> {
        todo!()
    }

    fn handle_playing(
        &self,
        event: anathema::component::Event,
        ctx: &mut GlobalContext,
    ) -> Option<anathema::component::Event> {
        todo!()
    }

    fn handle_game_over(
        &self,
        event: anathema::component::Event,
        ctx: &mut GlobalContext,
    ) -> Option<anathema::component::Event> {
        todo!()
    }
}
