use std::num::NonZeroU64;
use std::ops::Not;

use crate::board::Board;
use crate::coord::Square;
use crate::mov::{Move, Threat};
use crate::piece::{Piece, PieceKind};
use crate::player::PlayerKind;

pub static REPETITIONS_TO_FORCED_DRAW_COUNT: usize = 5;
pub static FIFTY_MOVE_RULE_COUNT: u64 = 100;

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::Display)]
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
impl Default for FullMoveCount {
    fn default() -> Self {
        Self(NonZeroU64::new(1).unwrap())
    }
}

#[derive(Default, Clone)]
pub struct GameState {
    pub board: Board,
    pub fifty_move_rule_clock: u64,
    pub white_castling_rights: CastlingRights,
    pub black_castling_rights: CastlingRights,
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
    pub fn testing() -> Self {
        Self {
            board: Board::filled(false),
            ..Default::default()
        }
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
            (PlayerKind::White, CastlingSide::Kingside) => self.white_castling_rights.kingside,
            (PlayerKind::White, CastlingSide::Queenside) => self.white_castling_rights.queenside,
            (PlayerKind::Black, CastlingSide::Kingside) => self.black_castling_rights.kingside,
            (PlayerKind::Black, CastlingSide::Queenside) => self.black_castling_rights.queenside,
        }
    }

    pub const fn deny_castling(&mut self, castling_side: CastlingSide) {
        match (self.active_player, castling_side) {
            (PlayerKind::White, CastlingSide::Kingside) => {
                self.white_castling_rights.kingside = false;
            }
            (PlayerKind::White, CastlingSide::Queenside) => {
                self.white_castling_rights.queenside = false;
            }
            (PlayerKind::Black, CastlingSide::Kingside) => {
                self.black_castling_rights.kingside = false;
            }
            (PlayerKind::Black, CastlingSide::Queenside) => {
                self.black_castling_rights.queenside = false;
            }
        }
    }

    pub fn legal_moves(&self) -> impl Iterator<Item = Move> + Clone + use<'_> {
        self.threatening_move_candidates()
            .chain(self.pawn_step_candidates())
            .chain(self.castle_candidates())
            .filter(move |mov| {
                self.clone()
                    .apply_move_to_board(*mov)
                    .board
                    .is_king_checked(self.active_player)
                    .not()
            })
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn step(self, mov: Move, all_legal_moves: Vec<Move>) -> StepResult {
        let mut new_game = self.clone().apply_move_to_board(mov);

        let current_position = Position {
            board: new_game.board,
            possible_moves: all_legal_moves,
        };

        match mov {
            Move::Normal {
                piece_kind: PieceKind::Pawn,
                start: _,
                target: _,
                is_capture: _,
            }
            | Move::DoubleStep { .. }
            | Move::Normal {
                piece_kind: _,
                start: _,
                target: _,
                is_capture: true,
            }
            | Move::Promotion { .. }
            | Move::EnPassant { .. } => new_game.fifty_move_rule_clock = 0,
            Move::Normal { .. } | Move::Castling(_) => new_game.fifty_move_rule_clock += 1,
        }

        match mov {
            Move::Castling(_)
            | Move::Normal {
                piece_kind: PieceKind::King,
                start: _,
                target: _,
                is_capture: _,
            } => {
                new_game.deny_castling(CastlingSide::Kingside);
                new_game.deny_castling(CastlingSide::Queenside);
            }
            Move::Normal {
                piece_kind: PieceKind::Rook,
                start,
                target: _,
                is_capture: _,
            } => {
                for castling_side in [CastlingSide::Kingside, CastlingSide::Queenside] {
                    if start == new_game.active_player.rook_start(castling_side) {
                        new_game.deny_castling(castling_side);
                    }
                }
            }
            Move::Normal { .. }
            | Move::DoubleStep { .. }
            | Move::Promotion { .. }
            | Move::EnPassant { .. } => {} //nothing
        }

        if let Move::DoubleStep { start: _, target } = mov {
            new_game.en_passant_target = Some(target);
        } else {
            new_game.en_passant_target = None;
        }

        if self.is_perft.not() {
            new_game.position_history.push(current_position.clone());

            if new_game
                .position_history
                .iter()
                .filter(|&position| *position == current_position)
                .count()
                == REPETITIONS_TO_FORCED_DRAW_COUNT
            {
                return StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::ThreefoldRepetition),
                    final_game_state: new_game,
                });
            }

            if new_game.fifty_move_rule_clock == FIFTY_MOVE_RULE_COUNT {
                return StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::FiftyMove),
                    final_game_state: new_game,
                });
            }
        }

        if new_game.legal_moves().count() == 0 {
            return if new_game
                .board
                .is_king_checked(self.active_player.opponent())
            {
                StepResult::Terminated(GameResult {
                    kind: GameResultKind::Win,
                    final_game_state: new_game,
                })
            } else {
                StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::Stalemate),
                    final_game_state: new_game,
                })
            };
        }

        new_game.active_player = new_game.active_player.opponent();
        StepResult::Continued(new_game)
    }

    fn castle_candidates(&self) -> impl Iterator<Item = Move> + Clone {
        [CastlingSide::Kingside, CastlingSide::Queenside]
            .into_iter()
            .filter(|castling_side| self.is_castling_allowed(*castling_side))
            .filter(|castling_side| {
                let threatened_squares = self
                    .board
                    .threatened_squares_by(self.active_player.opponent())
                    .collect::<Vec<_>>();

                self.active_player
                    .castling_non_check_needed_squares(*castling_side)
                    .iter()
                    .all(|castle_square| {
                        threatened_squares
                            .iter()
                            .all(|threatened_square| threatened_square != castle_square)
                    })
                    && self
                        .active_player
                        .castling_free_needed_squares(*castling_side)
                        .iter()
                        .all(|square| self.board[*square].is_none())
            })
            .map(Move::Castling)
    }

    fn threatening_move_candidates(&self) -> impl Iterator<Item = Move> + Clone + use<'_> {
        self.board
            .threatening_moves_by(self.active_player)
            .flat_map(|threat| self.threat_to_move_candidate(threat))
    }

    #[must_use]
    fn threat_to_move_candidate(&self, threat: Threat) -> Vec<Move> {
        match (threat.piece.kind, self.board[threat.target]) {
            (
                PieceKind::Knight
                | PieceKind::Bishop
                | PieceKind::Rook
                | PieceKind::Queen
                | PieceKind::King,
                target_square,
            ) => vec![Move::Normal {
                piece_kind: threat.piece.kind,
                start: threat.start,
                target: threat.target,
                is_capture: target_square.is_some(),
            }],
            (PieceKind::Pawn, Some(_)) => {
                if threat.target.row == self.active_player.pawn_promotion_row() {
                    PieceKind::PROMOTION_OPTIONS
                        .iter()
                        .map(|promotion_option| Move::Promotion {
                            start: threat.start,
                            target: threat.target,
                            is_capture: true,
                            replacement: *promotion_option,
                        })
                        .collect()
                } else {
                    vec![Move::Normal {
                        piece_kind: threat.piece.kind,
                        start: threat.start,
                        target: threat.target,
                        is_capture: true,
                    }]
                }
            }
            (PieceKind::Pawn, None) => {
                //en passant case, this is never gonna lead to promotion
                if let Some(en_passant_target) = self.en_passant_target
                    && threat.target == en_passant_target
                {
                    vec![Move::EnPassant {
                        start: threat.start,
                        target: threat.target,
                        affected_square: (threat.target + self.active_player.backwards_one_row())
                            .expect("how is this not on the board?"),
                    }]
                } else {
                    vec![]
                }
            }
        }
    }

    #[must_use]
    fn pawn_step_candidates(&self) -> Vec<Move> {
        let mut candidates = vec![];
        for square in Square::all() {
            let player = self.active_player;
            if !(self.board[square]
                == Some(Piece {
                    kind: PieceKind::Pawn,
                    owner: player,
                }))
            {
                continue;
            }

            let one_in_front = (square + self.active_player.forwards_one_row())
                .expect("a pawn cannot exist on the last row");

            if self.board[one_in_front].is_some() {
                continue; // pawns cant capture moving forward!
            }

            if one_in_front.row == self.active_player.pawn_promotion_row() {
                PieceKind::PROMOTION_OPTIONS
                    .iter()
                    .map(|promotion_option| Move::Promotion {
                        start: square,
                        target: one_in_front,
                        is_capture: false,
                        replacement: *promotion_option,
                    })
                    .collect_into(&mut candidates);
            } else {
                candidates.push(Move::Normal {
                    piece_kind: PieceKind::Pawn,
                    start: square,
                    target: one_in_front,
                    is_capture: false,
                });
            }

            let Ok(two_in_front) = square + self.active_player.forwards_one_row() * 2 else {
                continue; // this one can def be out of range.
            };

            if square.row != self.active_player.pawn_starting_row() {
                continue; // pawns can only double-move when they havent moved yet!
            }

            if self.board[two_in_front].is_some() {
                continue; // pawns cant capture moving forward!
            }

            candidates.push(Move::DoubleStep {
                start: square,
                target: two_in_front,
            });
        }
        candidates
    }
    #[must_use]
    pub const fn apply_move_to_board(mut self, m: Move) -> Self {
        match m {
            Move::Normal {
                piece_kind: _,
                start,
                target,
                is_capture: _,
            }
            | Move::DoubleStep { start, target } => self.board.mov(start, target),
            Move::EnPassant {
                start,
                target,
                affected_square,
            } => {
                self.board.mov(start, target);
                self.board[affected_square] = None;
            }
            Move::Promotion {
                start,
                target,
                is_capture: _,
                replacement,
            } => {
                self.board.mov(start, target);
                self.board[target] = Some(Piece {
                    kind: replacement,
                    owner: self.active_player,
                });
            }
            Move::Castling(castling_side) => {
                self.board.mov(
                    self.active_player.king_start(),
                    self.active_player.king_castling_target(castling_side),
                );
                self.board.mov(
                    self.active_player.rook_start(castling_side),
                    self.active_player.rook_castling_target(castling_side),
                );
            }
        }
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct CastlingRights {
    pub kingside: bool,
    pub queenside: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub board: Board,
    pub possible_moves: Vec<Move>,
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
            .take_while(Result::is_ok) // ugly but right, once this is Err(_) once, itll _always_ be out of bounds!
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
