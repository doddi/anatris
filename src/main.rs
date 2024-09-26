mod core;
mod widgets;

use anathema::{
    backend::tui::events::CTKeyCode,
    component::{ComponentId, KeyCode, KeyEvent},
    prelude::*,
    widgets::components::events::KeyState,
};
use std::fs::read_to_string;
use widgets::{
    game::{GameComponent, GameComponentMessage, GameComponentState},
    game_arena::{GameArenaComponent, GameArenaComponentMessage, GameArenaComponentState},
    game_type::{GameTypeComponent, GameTypeState},
    line_count::{LineCountComponent, LineCountState},
    main_menu::{MainMenuComponent, MainMenuComponentMessage, MainMenuComponentState},
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

    let game_arena = runtime
        .register_component(
            "GameArena",
            "src/templates/game_arena.aml",
            GameArenaComponent::new(),
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

    let runtime =
        runtime.set_global_event_handler(GameStateManagement::new(&main_menu, &game, &game_arena));
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
    game_arena: &'a ComponentId<GameArenaComponentMessage>,
}

impl<'a> GlobalEvents for GameStateManagement<'a> {
    fn handle(
        &mut self,
        event: anathema::component::Event,
        _elements: &mut anathema::widgets::Elements<'_, '_>,
        ctx: &mut GlobalContext<'_>,
    ) -> Option<anathema::component::Event> {
        if let Some(exit) = self.check_for_exit(&event) {
            return Some(exit);
        }

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
        game_arena: &'a ComponentId<GameArenaComponentMessage>,
    ) -> Self {
        Self {
            state: GameState::default(),
            main_menu,
            game,
            game_arena,
        }
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

    fn handle_main_menu(
        &mut self,
        event: anathema::component::Event,
        ctx: &mut GlobalContext<'_>,
    ) -> Option<anathema::component::Event> {
        if let anathema::component::Event::Key(keyevent) = event {
            match keyevent {
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    ctx.emit(*self.main_menu, MainMenuComponentMessage::Invisible);
                    ctx.emit(*self.game, GameComponentMessage::Visible);
                    self.state = GameState::Playing;
                }
                KeyEvent {
                    code: KeyCode::Char('w'),
                    ..
                } => {
                    ctx.emit(*self.main_menu, MainMenuComponentMessage::KeyUp);
                }
                KeyEvent {
                    code: KeyCode::Char('s'),
                    ..
                } => {
                    ctx.emit(*self.main_menu, MainMenuComponentMessage::KeyDown);
                }
                _ => (),
            }
        }
        None
    }

    fn handle_pause(
        &mut self,
        event: anathema::component::Event,
        ctx: &mut GlobalContext,
    ) -> Option<anathema::component::Event> {
        match event {
            anathema::component::Event::Key(keyevent) => {
                let KeyEvent {
                    code,
                    ctrl: _,
                    state: _,
                } = keyevent;

                if let KeyCode::Esc = code {
                    ctx.emit(*self.game, GameComponentMessage::Running);
                    self.state = GameState::Playing;
                } else if let KeyCode::Enter = code {
                    ctx.emit(*self.main_menu, MainMenuComponentMessage::Visible);
                    ctx.emit(*self.game, GameComponentMessage::Invisible);
                    self.state = GameState::MainMenu;
                }
            }
            _ => (),
        }
        None
    }

    fn handle_playing(
        &mut self,
        event: anathema::component::Event,
        ctx: &mut GlobalContext,
    ) -> Option<anathema::component::Event> {
        if let anathema::component::Event::Key(keyevent) = event {
            let KeyEvent {
                code,
                ctrl: _,
                state: _,
            } = keyevent;

            match code {
                KeyCode::Esc => {
                    ctx.emit(*self.game, GameComponentMessage::Paused);
                    self.state = GameState::Paused;
                }
                KeyCode::Char(' ') => ctx.emit(*self.game_arena, GameArenaComponentMessage::Rotate),
                KeyCode::Char('a') => {
                    ctx.emit(*self.game_arena, GameArenaComponentMessage::MoveLeft)
                }
                KeyCode::Char('d') => {
                    ctx.emit(*self.game_arena, GameArenaComponentMessage::MoveRight)
                }
                KeyCode::Char('s') => ctx.emit(*self.game_arena, GameArenaComponentMessage::Drop),
                _ => (),
            }
        }
        None
    }

    fn handle_game_over(
        &self,
        _event: anathema::component::Event,
        _ctx: &mut GlobalContext,
    ) -> Option<anathema::component::Event> {
        todo!()
    }
}
