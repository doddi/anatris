use std::process;

use anathema::component::{ComponentId, Emitter, KeyCode, KeyEvent};
use smol::channel::{Receiver, Sender};

use crate::widgets::{
    game::GameComponentMessage,
    game_arena::GameArenaComponentMessage,
    game_over::GameOverComponentMessage,
    line_count::LineCountComponentMessage,
    main_menu::{MainMenuAction, MainMenuComponentMessage},
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
    tx: Sender<GlobalStateManagementMessage>,
    rx: Receiver<GlobalStateManagementMessage>,
    game_state_component_ids: GameStateComponentIds,
) {
    smol::spawn(async move {
        let mut state = GameState::MainMenu;

        while let Ok(message) = rx.recv().await {
            match message {
                GlobalStateManagementMessage::MainMenu => {
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
                GlobalStateManagementMessage::Paused => {
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
                GlobalStateManagementMessage::Playing => {
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
                GlobalStateManagementMessage::GameOver => {
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
                GlobalStateManagementMessage::Event(event) => match state {
                    GameState::MainMenu => {
                        handle_main_menu(game_state_component_ids.main_menu_id, event, &emitter)
                    }
                    GameState::Paused => handle_pause(event, &tx),
                    GameState::Playing => {
                        handle_playing(event, &tx, &emitter, game_state_component_ids.game_arena_id)
                    }
                    GameState::GameOver => handle_game_over(),
                },
                GlobalStateManagementMessage::UpdateScore(score) => {
                    handle_update_score(&emitter, score, game_state_component_ids.score_board_id)
                }
                GlobalStateManagementMessage::UpdateLines(value) => {
                    handle_update_lines(&emitter, value, game_state_component_ids.lines_count_id)
                }
                GlobalStateManagementMessage::UpdateNextTetronimo(tetronimo) => {
                    handle_update_next_tetronimo(
                        &emitter,
                        game_state_component_ids.next_piece_id,
                        tetronimo,
                    )
                }
                GlobalStateManagementMessage::UpdateStatistics(data) => {
                    handle_update_statistics(&emitter, game_state_component_ids.statistics_id, data)
                }
                GlobalStateManagementMessage::Exit => {
                    process::exit(0);
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
    main_menu_id: ComponentId<MainMenuComponentMessage>,
    event: anathema::component::Event,
    tx: &Emitter,
) {
    if let anathema::component::Event::Key(keyevent) = event {
        match keyevent {
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                let _ = tx.emit(
                    main_menu_id,
                    MainMenuComponentMessage::Change(MainMenuAction::Enter),
                );
            }
            KeyEvent {
                code: KeyCode::Char('w'),
                ..
            } => {
                let _ = tx.emit(
                    main_menu_id,
                    MainMenuComponentMessage::Change(MainMenuAction::Up),
                );
            }
            KeyEvent {
                code: KeyCode::Char('s'),
                ..
            } => {
                let _ = tx.emit(
                    main_menu_id,
                    MainMenuComponentMessage::Change(MainMenuAction::Down),
                );
            }
            _ => (),
        }
    }
}

fn handle_pause(event: anathema::component::Event, tx: &Sender<GlobalStateManagementMessage>) {
    if let anathema::component::Event::Key(keyevent) = event {
        let KeyEvent {
            code,
            ctrl: _,
            state: _,
        } = keyevent;

        if let KeyCode::Esc = code {
            let _ = tx.try_send(GlobalStateManagementMessage::Playing);
        } else if let KeyCode::Enter = code {
            let _ = tx.try_send(GlobalStateManagementMessage::MainMenu);
        }
    }
}

fn handle_playing(
    event: anathema::component::Event,
    tx: &Sender<GlobalStateManagementMessage>,
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
                let _ = tx.try_send(GlobalStateManagementMessage::Paused);
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

#[derive(Debug)]
pub(crate) enum GlobalStateManagementMessage {
    MainMenu,
    Paused,
    Playing,
    GameOver,
    Event(anathema::component::Event),
    UpdateScore(u16),
    UpdateLines(u16),
    UpdateNextTetronimo(TetronimoShape),
    UpdateStatistics(StatisticsComponentMessage),
    Exit,
}

#[derive(Debug)]
enum GameState {
    MainMenu,
    Paused,
    Playing,
    GameOver,
}

impl From<GlobalStateManagementMessage> for GameState {
    fn from(value: GlobalStateManagementMessage) -> Self {
        match value {
            GlobalStateManagementMessage::MainMenu => GameState::MainMenu,
            GlobalStateManagementMessage::Paused => GameState::Paused,
            GlobalStateManagementMessage::Playing => GameState::Playing,
            GlobalStateManagementMessage::GameOver => GameState::GameOver,
            _ => {
                panic!("Key handling state is not a valid state to transition to")
            }
        }
    }
}
