use anathema::component::{ComponentId, Emitter, KeyCode, KeyEvent};
use smol::channel::{Receiver, Sender};

use crate::widgets::{
    game::GameComponentMessage,
    game_arena::GameArenaComponentMessage,
    game_over::GameOverComponentMessage,
    line_count::LineCountComponentMessage,
    main_menu::{MainMenuComponentMessage, MainMenuComponentSelection},
    next_piece::NextPieceComponentMessage,
    scoreboard::ScoreBoardComponentMessage,
    statistics::StatisticsComponentMessage,
};

use super::tetronimo::TetronimoShape;

pub(crate) struct GameStateComponentIds {
    main_menu_id: ComponentId<MainMenuComponentMessage>,
    game_id: ComponentId<GameComponentMessage>,
    game_arena_id: ComponentId<GameArenaComponentMessage>,
    game_over_id: ComponentId<GameOverComponentMessage>,
    score_board_id: ComponentId<ScoreBoardComponentMessage>,
    lines_count_id: ComponentId<LineCountComponentMessage>,
    next_piece_id: ComponentId<NextPieceComponentMessage>,
    statistics_id: ComponentId<StatisticsComponentMessage>,
}

#[allow(clippy::too_many_arguments)]
impl GameStateComponentIds {
    pub(crate) fn new(
        main_menu_id: ComponentId<MainMenuComponentMessage>,
        game_id: ComponentId<GameComponentMessage>,
        game_arena_id: ComponentId<GameArenaComponentMessage>,
        game_over_id: ComponentId<GameOverComponentMessage>,
        score_board_id: ComponentId<ScoreBoardComponentMessage>,
        lines_count_id: ComponentId<LineCountComponentMessage>,
        next_piece_id: ComponentId<NextPieceComponentMessage>,
        statistics_id: ComponentId<StatisticsComponentMessage>,
    ) -> Self {
        Self {
            main_menu_id,
            game_id,
            game_arena_id,
            game_over_id,
            score_board_id,
            lines_count_id,
            next_piece_id,
            statistics_id,
        }
    }
}

pub(crate) fn start(
    emitter: anathema::component::Emitter,
    tx: Sender<GameStateManagementMessage>,
    rx: Receiver<GameStateManagementMessage>,
    game_state_component_ids: GameStateComponentIds,
) {
    smol::spawn(async move {
        let mut main_menu_choice = MainMenuChoice::Start;
        let mut state = GameState::MainMenu;

        while let Ok(message) = rx.recv().await {
            match message {
                GameStateManagementMessage::MainMenu => {
                    let _ = emitter.emit(
                        game_state_component_ids.main_menu_id,
                        MainMenuComponentMessage::Visible,
                    );
                    let _ = emitter.emit(
                        game_state_component_ids.game_id,
                        GameComponentMessage::Invisible,
                    );
                    let _ = emitter.emit(
                        game_state_component_ids.game_id,
                        GameComponentMessage::Paused,
                    );
                    let _ = emitter.emit(
                        game_state_component_ids.game_over_id,
                        GameOverComponentMessage::Invisible,
                    );
                    state = message.into();
                }
                GameStateManagementMessage::Paused => {
                    let _ = emitter.emit(
                        game_state_component_ids.main_menu_id,
                        MainMenuComponentMessage::Invisible,
                    );
                    let _ = emitter.emit(
                        game_state_component_ids.game_id,
                        GameComponentMessage::Visible,
                    );
                    let _ = emitter.emit(
                        game_state_component_ids.game_id,
                        GameComponentMessage::Paused,
                    );
                    let _ = emitter.emit(
                        game_state_component_ids.game_over_id,
                        GameOverComponentMessage::Invisible,
                    );
                    state = message.into();
                }
                GameStateManagementMessage::Playing => {
                    let _ = emitter.emit(
                        game_state_component_ids.main_menu_id,
                        MainMenuComponentMessage::Invisible,
                    );
                    let _ = emitter.emit(
                        game_state_component_ids.game_id,
                        GameComponentMessage::Visible,
                    );
                    let _ = emitter.emit(
                        game_state_component_ids.game_id,
                        GameComponentMessage::Running,
                    );
                    let _ = emitter.emit(
                        game_state_component_ids.game_over_id,
                        GameOverComponentMessage::Invisible,
                    );
                    state = message.into();
                }
                GameStateManagementMessage::GameOver => {
                    let _ = emitter.emit(
                        game_state_component_ids.main_menu_id,
                        MainMenuComponentMessage::Invisible,
                    );
                    let _ = emitter.emit(
                        game_state_component_ids.game_id,
                        GameComponentMessage::Invisible,
                    );
                    let _ = emitter.emit(
                        game_state_component_ids.game_over_id,
                        GameOverComponentMessage::Visible,
                    );
                    state = message.into();
                }
                GameStateManagementMessage::Event(event) => match state {
                    GameState::MainMenu => handle_main_menu(
                        game_state_component_ids.game_id,
                        game_state_component_ids.game_over_id,
                        game_state_component_ids.main_menu_id,
                        &mut main_menu_choice,
                        event,
                        &mut state,
                        &emitter,
                    ),
                    GameState::Paused => handle_pause(event, &tx),
                    GameState::Playing => {
                        handle_playing(event, &tx, &emitter, game_state_component_ids.game_arena_id)
                    }
                    GameState::GameOver => handle_game_over(),
                },
                GameStateManagementMessage::UpdateScore(score) => {
                    handle_update_score(&emitter, score, game_state_component_ids.score_board_id)
                }
                GameStateManagementMessage::UpdateLines(value) => {
                    handle_update_lines(&emitter, value, game_state_component_ids.lines_count_id)
                }
                GameStateManagementMessage::UpdateNextTetronimo(tetronimo) => {
                    handle_update_next_tetronimo(
                        &emitter,
                        game_state_component_ids.next_piece_id,
                        tetronimo,
                    )
                }
                GameStateManagementMessage::UpdateStatistics(data) => {
                    handle_update_statistics(&emitter, game_state_component_ids.statistics_id, data)
                }
            }
        }
    })
    .detach();
}

fn handle_update_statistics(
    emitter: &Emitter,
    statistics_id: ComponentId<StatisticsComponentMessage>,
    data: StatisticsComponentMessage,
) {
    let _ = emitter.emit(statistics_id, data);
}

fn handle_update_next_tetronimo(
    emitter: &Emitter,
    next_piece_id: ComponentId<NextPieceComponentMessage>,
    tetronimo: TetronimoShape,
) {
    let _ = emitter.emit(next_piece_id, NextPieceComponentMessage::new(tetronimo));
}

fn handle_update_lines(
    emitter: &Emitter,
    value: u16,
    lines_count_id: ComponentId<LineCountComponentMessage>,
) {
    let _ = emitter.emit(lines_count_id, LineCountComponentMessage::Count(value));
}

fn handle_update_score(
    emitter: &Emitter,
    score: u16,
    score_board_id: ComponentId<ScoreBoardComponentMessage>,
) {
    let _ = emitter.emit(score_board_id, ScoreBoardComponentMessage::Score(score));
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

fn handle_pause(event: anathema::component::Event, tx: &Sender<GameStateManagementMessage>) {
    if let anathema::component::Event::Key(keyevent) = event {
        let KeyEvent {
            code,
            ctrl: _,
            state: _,
        } = keyevent;

        if let KeyCode::Esc = code {
            tx.try_send(GameStateManagementMessage::Playing);
        } else if let KeyCode::Enter = code {
            tx.try_send(GameStateManagementMessage::MainMenu);
        }
    }
}

fn handle_playing(
    event: anathema::component::Event,
    tx: &Sender<GameStateManagementMessage>,
    emitter: &Emitter,
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
                tx.try_send(GameStateManagementMessage::Paused);
            }
            KeyCode::Char(' ') => {
                let _ = emitter.emit(game_arena, GameArenaComponentMessage::Rotate);
            }
            KeyCode::Char('a') => {
                let _ = emitter.emit(game_arena, GameArenaComponentMessage::MoveLeft);
            }
            KeyCode::Char('d') => {
                let _ = emitter.emit(game_arena, GameArenaComponentMessage::MoveRight);
            }
            KeyCode::Char('s') => {
                let _ = emitter.emit(game_arena, GameArenaComponentMessage::Drop);
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
    UpdateScore(u16),
    UpdateLines(u16),
    UpdateNextTetronimo(TetronimoShape),
    UpdateStatistics(StatisticsComponentMessage),
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
            _ => {
                panic!("Key handling state is not a valid state to transition to")
            }
        }
    }
}
