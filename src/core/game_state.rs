use anathema::component::{ComponentId, Emitter, KeyCode, KeyEvent};
use smol::channel::Receiver;

use crate::widgets::{
    game::GameComponentMessage,
    game_arena::GameArenaComponentMessage,
    game_over::GameOverComponentMessage,
    main_menu::{MainMenuComponentMessage, MainMenuComponentSelection},
};

pub(crate) fn start(
    emitter: anathema::component::Emitter,
    rx: Receiver<GameStateManagementMessage>,
    main_menu_id: ComponentId<MainMenuComponentMessage>,
    game_id: ComponentId<GameComponentMessage>,
    game_arena_id: ComponentId<GameArenaComponentMessage>,
    game_over_id: ComponentId<GameOverComponentMessage>,
) {
    smol::spawn(async move {
        let mut main_menu_choice = MainMenuChoice::Start;
        let mut state = GameState::MainMenu;

        while let Ok(message) = rx.recv().await {
            match message {
                GameStateManagementMessage::MainMenu => {
                    let _ = emitter.emit(main_menu_id, MainMenuComponentMessage::Visible);
                    let _ = emitter.emit(game_id, GameComponentMessage::Invisible);
                    let _ = emitter.emit(game_id, GameComponentMessage::Paused);
                    let _ = emitter.emit(game_over_id, GameOverComponentMessage::Invisible);
                    state = message.into();
                }
                GameStateManagementMessage::Paused => {
                    let _ = emitter.emit(main_menu_id, MainMenuComponentMessage::Invisible);
                    let _ = emitter.emit(game_id, GameComponentMessage::Visible);
                    let _ = emitter.emit(game_id, GameComponentMessage::Paused);
                    let _ = emitter.emit(game_over_id, GameOverComponentMessage::Invisible);
                    state = message.into();
                }
                GameStateManagementMessage::Playing => {
                    let _ = emitter.emit(main_menu_id, MainMenuComponentMessage::Invisible);
                    let _ = emitter.emit(game_id, GameComponentMessage::Visible);
                    let _ = emitter.emit(game_id, GameComponentMessage::Running);
                    let _ = emitter.emit(game_over_id, GameOverComponentMessage::Invisible);
                    state = message.into();
                }
                GameStateManagementMessage::GameOver => {
                    let _ = emitter.emit(main_menu_id, MainMenuComponentMessage::Invisible);
                    let _ = emitter.emit(game_id, GameComponentMessage::Invisible);
                    let _ = emitter.emit(game_over_id, GameOverComponentMessage::Visible);
                    state = message.into();
                }
                GameStateManagementMessage::Event(event) => match state {
                    GameState::MainMenu => handle_main_menu(
                        game_id,
                        game_over_id,
                        main_menu_id,
                        &mut main_menu_choice,
                        event,
                        &mut state,
                        &emitter,
                    ),
                    GameState::Paused => handle_pause(event, &mut state),
                    GameState::Playing => {
                        handle_playing(event, &emitter, &mut state, game_arena_id)
                    }
                    GameState::GameOver => handle_game_over(),
                },
            }
        }
    })
    .detach();
}

fn handle_main_menu(
    game_id: ComponentId<GameComponentMessage>,
    game_over_id: ComponentId<GameOverComponentMessage>,
    main_menu_id: ComponentId<MainMenuComponentMessage>,
    main_menu_choice: &mut MainMenuChoice,
    event: anathema::component::Event,
    game_state: &mut GameState,
    tx: &Emitter,
) {
    if let anathema::component::Event::Key(keyevent) = event {
        match keyevent {
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => match main_menu_choice {
                MainMenuChoice::Start => {
                    let _ = tx.emit(main_menu_id, MainMenuComponentMessage::Invisible);
                    let _ = tx.emit(game_id, GameComponentMessage::Visible);
                    let _ = tx.emit(game_id, GameComponentMessage::Running);
                    let _ = tx.emit(game_over_id, GameOverComponentMessage::Invisible);
                    *game_state = GameState::Playing;
                }
                MainMenuChoice::Exit => std::process::exit(0),
            },
            KeyEvent {
                code: KeyCode::Char('w'),
                ..
            } => {
                *main_menu_choice = main_menu_choice.up();
                let _ = tx.emit(
                    main_menu_id,
                    MainMenuComponentMessage::ChangeTo(main_menu_choice.clone().into()),
                );
            }
            KeyEvent {
                code: KeyCode::Char('s'),
                ..
            } => {
                *main_menu_choice = main_menu_choice.down();
                let _ = tx.emit(
                    main_menu_id,
                    MainMenuComponentMessage::ChangeTo(main_menu_choice.clone().into()),
                );
            }
            _ => (),
        }
    }
}

fn handle_pause(event: anathema::component::Event, state: &mut GameState) {
    if let anathema::component::Event::Key(keyevent) = event {
        let KeyEvent {
            code,
            ctrl: _,
            state: _,
        } = keyevent;

        if let KeyCode::Esc = code {
            *state = GameState::Playing;
        } else if let KeyCode::Enter = code {
            *state = GameState::MainMenu;
        }
    }
}

fn handle_playing(
    event: anathema::component::Event,
    tx: &Emitter,
    state: &mut GameState,
    game_arena: ComponentId<GameArenaComponentMessage>,
) {
    if let anathema::component::Event::Key(keyevent) = event {
        let KeyEvent {
            code,
            ctrl: _,
            state: _,
        } = keyevent;

        match code {
            KeyCode::Esc => {
                *state = GameState::Paused;
            }
            KeyCode::Char(' ') => {
                let _ = tx.emit(game_arena, GameArenaComponentMessage::Rotate);
            }
            KeyCode::Char('a') => {
                let _ = tx.emit(game_arena, GameArenaComponentMessage::MoveLeft);
            }
            KeyCode::Char('d') => {
                let _ = tx.emit(game_arena, GameArenaComponentMessage::MoveRight);
            }
            KeyCode::Char('s') => {
                let _ = tx.emit(game_arena, GameArenaComponentMessage::Drop);
            }
            _ => (),
        }
    }
}

fn handle_game_over() {}

#[derive(Clone)]
enum MainMenuChoice {
    Start,
    Exit,
}

impl MainMenuChoice {
    fn up(&mut self) -> Self {
        match self {
            MainMenuChoice::Start => MainMenuChoice::Exit,
            MainMenuChoice::Exit => MainMenuChoice::Start,
        }
    }

    fn down(&mut self) -> Self {
        match self {
            MainMenuChoice::Start => MainMenuChoice::Exit,
            MainMenuChoice::Exit => MainMenuChoice::Start,
        }
    }
}

impl From<MainMenuChoice> for MainMenuComponentSelection {
    fn from(value: MainMenuChoice) -> Self {
        match value {
            MainMenuChoice::Start => MainMenuComponentSelection::Start,
            MainMenuChoice::Exit => MainMenuComponentSelection::Exit,
        }
    }
}

#[derive(Default, Debug)]
pub(crate) enum GameStateManagementMessage {
    #[default]
    MainMenu,
    Paused,
    Playing,
    GameOver,
    Event(anathema::component::Event),
}

#[derive(Default, Debug)]
enum GameState {
    #[default]
    MainMenu,
    Paused,
    Playing,
    GameOver,
}

impl From<GameStateManagementMessage> for GameState {
    fn from(value: GameStateManagementMessage) -> Self {
        match value {
            GameStateManagementMessage::MainMenu => GameState::MainMenu,
            GameStateManagementMessage::Paused => GameState::Paused,
            GameStateManagementMessage::Playing => GameState::Playing,
            GameStateManagementMessage::GameOver => GameState::GameOver,
            GameStateManagementMessage::Event(_) => {
                panic!("Key handling state is not a valid state to transition to")
            }
        }
    }
}
