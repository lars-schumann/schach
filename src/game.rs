use crate::board::Board;
use crate::coord::Square;
use crate::mov::NewMove;
use crate::mov::Threat;
use crate::player::PlayerKind;
use alloc::vec;
use alloc::vec::Vec;
use core::num::NonZeroU64;

pub static REPETITIONS_TO_FORCED_DRAW_COUNT: usize = 5;
pub static FIFTY_MOVE_RULE_COUNT: FiftyMoveRuleClock = FiftyMoveRuleClock(100);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CastlingSide {
    Kingside,
    Queenside,
}

pub enum DrawKind {
    Stalemate,
    ThreefoldRepetition,
    FiftyMove,
}

pub enum GameResultKind {
    Draw(DrawKind),
    Win,
}
pub struct GameResult {
    pub kind: GameResultKind,
    pub final_game_state: GameState,
}

pub enum StepResult {
    Terminated(GameResult),
    Continued(GameState),
}

#[derive(Clone, Copy, Debug)]
pub struct FullMoveCount(pub NonZeroU64); // non-zero & unsized because this always starts at 1 and cant decrease 
impl const Default for FullMoveCount {
    fn default() -> Self {
        Self::new()
    }
}
impl FullMoveCount {
    #[must_use]
    pub const fn new() -> Self {
        Self(NonZeroU64::new(1).unwrap())
    }
    pub const fn increase(&mut self) {
        self.0 = self.0.checked_add(1).expect("how did you overflow this");
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FiftyMoveRuleClock(pub u64);
impl FiftyMoveRuleClock {
    #[must_use]
    pub const fn new(initial: u64) -> Self {
        Self(initial)
    }
    pub const fn increase(&mut self) {
        self.0 += 1;
    }
    pub const fn reset(&mut self) {
        self.0 = 0;
    }
}

#[derive(Default, Clone)]
pub struct GameState {
    pub board: Board,
    pub fifty_move_rule_clock: FiftyMoveRuleClock,
    pub castling_rights: CastlingRights,
    pub position_history: Vec<Position>,
    pub en_passant_target: Option<Square>,
    pub active_player: PlayerKind,
    pub is_perft: bool,
    pub full_move_count: FullMoveCount,
}

impl GameState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn perft() -> Self {
        Self {
            is_perft: true,
            ..Default::default()
        }
    }

    #[must_use]
    pub const fn is_castling_allowed(&self, castling_side: CastlingSide) -> bool {
        match (self.active_player, castling_side) {
            (PlayerKind::White, CastlingSide::Kingside) => self.castling_rights.white_kingside,
            (PlayerKind::White, CastlingSide::Queenside) => self.castling_rights.white_queenside,
            (PlayerKind::Black, CastlingSide::Kingside) => self.castling_rights.black_kingside,
            (PlayerKind::Black, CastlingSide::Queenside) => self.castling_rights.black_queenside,
        }
    }

    pub const fn deny_castling(&mut self, for_player: PlayerKind, castling_side: CastlingSide) {
        match (for_player, castling_side) {
            (PlayerKind::White, CastlingSide::Kingside) => {
                self.castling_rights.white_kingside = false;
            }
            (PlayerKind::White, CastlingSide::Queenside) => {
                self.castling_rights.white_queenside = false;
            }
            (PlayerKind::Black, CastlingSide::Kingside) => {
                self.castling_rights.black_kingside = false;
            }
            (PlayerKind::Black, CastlingSide::Queenside) => {
                self.castling_rights.black_queenside = false;
            }
        }
    }

    #[must_use]
    pub const fn with_opponent_active(mut self) -> Self {
        self.active_player = self.active_player.opponent();
        self
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}
impl CastlingRights {
    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)]
    pub const fn new(
        white_kingside: bool,
        white_queenside: bool,
        black_kingside: bool,
        black_queenside: bool,
    ) -> Self {
        Self {
            white_kingside,
            white_queenside,
            black_kingside,
            black_queenside,
        }
    }
    pub fn all() -> impl Iterator<Item = Self> + Clone {
        [false, true]
            .into_iter()
            .flat_map(|a| vec![(a, false), (a, true)])
            .flat_map(|a| vec![(a, false), (a, true)])
            .flat_map(|a| vec![(a, false), (a, true)])
            .map(|(((a, b), c), d)| Self::new(a, b, c, d))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub board: Board,
    pub possible_moves: Vec<NewMove>,
}

#[must_use]
pub fn attacked_squares(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> Vec<Threat> {
    let Some(piece) = board[starting_square] else {
        return vec![];
    };
    if piece.owner != active_player {
        return vec![];
    }
    let (directions, range_upper_bound) = piece.threat_directions();
    let range_upper_bound = i32::from(range_upper_bound);

    let rays = directions.iter().map(move |direction| {
        (0..range_upper_bound)
            .map(move |i| starting_square + *direction * (i + 1))
            .take_while(Result::is_ok) // ugly but right, once this is Err(_) once, it'll _always_ be out of bounds!
            .map(Result::unwrap)
    });

    let mut out = vec![];
    for ray in rays {
        for target_square in ray {
            match board[target_square] {
                None => out.push(target_square),
                Some(piece) if piece.owner == active_player => {
                    break;
                }
                Some(piece) if piece.owner != active_player => {
                    out.push(target_square);
                    break;
                }
                _ => unreachable!(),
            }
        }
    }
    out.into_iter()
        .map(|target_square| Threat {
            piece,
            start: starting_square,
            target: target_square,
        })
        .collect()
}
