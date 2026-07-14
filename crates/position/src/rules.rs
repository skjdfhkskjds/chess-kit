use crate::State;
use crate::position::DefaultPosition;
use chess_kit_attack_table::AttackTable;
use chess_kit_primitives::{Black, Pieces, Sides, White};

impl<AT, StateT> DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State,
{
    /// is_draw checks if the position is a draw
    ///
    /// @return: true if the position is a draw, false otherwise
    pub fn is_draw(&self) -> bool {
        self.is_draw_by_fifty_moves()
            || self.is_draw_by_insufficient_material()
            || self.is_draw_by_repetition()
    }

    /// is_draw_by_fifty_moves checks if the position is a draw according to the
    /// 50-move rule
    ///
    /// @return: true if the position is a draw by the rule, false otherwise
    #[inline]
    pub fn is_draw_by_fifty_moves(&self) -> bool {
        // Note: 100 since we are using the halfmove clock
        self.state().halfmoves() >= 100
    }

    /// is_draw_by_insufficient_material checks if the position is a draw according
    /// to the draw by insufficient material rule
    ///
    /// @return: true if the position is a draw by the rule, false otherwise
    pub fn is_draw_by_insufficient_material(&self) -> bool {
        self.state().draw_state().is_material_draw()
    }

    /// is_draw_by_repetition checks if the position is a draw according to the
    /// draw by repetition rule
    ///
    /// @return: true if the position is a draw by the rule, false otherwise
    pub fn is_draw_by_repetition(&self) -> bool {
        self.state().draw_state().is_threefold_repetition()
    }

    /// is_draw_by_repetition_in_search checks if the position is a repetition
    /// relative to the root of the current search
    ///
    /// note: a second occurrence is a draw only when the previous occurrence is
    ///       strictly after the search root. a threefold repetition is a draw
    ///       regardless of where its previous occurrences are relative to root
    ///
    /// @param: ply - ply of the current position relative to the search root
    /// @return: true if the position is a search repetition, false otherwise
    #[inline]
    pub fn is_draw_by_repetition_in_search(&self, ply: usize) -> bool {
        self.state().draw_state().is_repetition(ply)
    }

    /// can_force_checkmate checks if either side can force checkmate
    ///
    /// @return: true if either side can force checkmate, false otherwise
    pub fn can_force_checkmate(&self) -> bool {
        !self.state().draw_state().is_material_draw()
    }

    /// update_material_draw_state updates the cached material draw flag from the
    /// current board material
    ///
    /// note: the result only needs to be recomputed when a capture or promotion
    ///       changes the material on the board. all other moves retain the value
    ///       copied from the previous state
    ///
    /// @return: void
    /// @side-effects: modifies the current `state`
    pub(crate) fn update_material_draw_state(&mut self) {
        let is_material_draw = !self.compute_can_force_checkmate();
        let draw_state = self
            .state()
            .draw_state()
            .with_material_draw(is_material_draw);
        self.state_mut().set_draw_state(draw_state);
    }

    /// update_repetition_state updates the signed distance to the previous
    /// occurrence of the current position
    ///
    /// note: only states with the same side to move can repeat, so the history
    ///       is searched in two-ply steps. four plies is the shortest possible
    ///       reversible cycle in chess
    ///
    /// note: a positive distance represents a second occurrence. if the matching
    ///       state was itself repeated, the distance is stored as negative to
    ///       represent a third or later occurrence
    ///
    /// @return: void
    /// @side-effects: modifies the current `state`
    pub(crate) fn update_repetition_state(&mut self) {
        let repetition = {
            let states = self.history.as_slice();
            let current_index = states.len() - 1;
            let current = &states[current_index];
            let end = (current.halfmoves() as usize).min(current_index);
            let mut repetition = 0;

            if end >= 4 {
                for distance in (4..=end).step_by(2) {
                    let historic_state = &states[current_index - distance];
                    if historic_state.key() != current.key() {
                        continue;
                    }

                    let distance = i16::try_from(distance)
                        .expect("position history distance should fit in i16");
                    repetition = if historic_state.draw_state().repetition() == 0 {
                        distance
                    } else {
                        -distance
                    };
                    break;
                }
            }

            repetition
        };

        let draw_state = self.state().draw_state().with_repetition(repetition);
        self.state_mut().set_draw_state(draw_state);
    }

    /// compute_can_force_checkmate computes if either side has sufficient
    /// material to force checkmate from the current board material
    ///
    /// @return: true if either side can force checkmate, false otherwise
    fn compute_can_force_checkmate(&self) -> bool {
        let w = self.bitboards[Sides::White];
        let b = self.bitboards[Sides::Black];

        // check if either side has sufficient solo material to deliver
        // checkmate
        //
        // that is, if either side has a queen, rook, or a pawn.
        let sufficient_solo_material = w[Pieces::Queen].not_empty()
            || w[Pieces::Rook].not_empty()
            || w[Pieces::Pawn].not_empty()
            || b[Pieces::Queen].not_empty()
            || b[Pieces::Rook].not_empty()
            || b[Pieces::Pawn].not_empty();

        // if either side has sufficient solo material or a bishop pair,
        // then that side can force checkmate
        if sufficient_solo_material
            || self.has_bishop_pair::<White>()
            || self.has_bishop_pair::<Black>()
        {
            return true;
        }

        let white_knights = w[Pieces::Knight].count_ones();
        let black_knights = b[Pieces::Knight].count_ones();

        // if either side has a knight-bishop pair, OR they have at least 3
        // knights, then that side can force checkmate
        (w[Pieces::Bishop].not_empty() && white_knights > 0)
            || (b[Pieces::Bishop].not_empty() && black_knights > 0)
            || white_knights >= 3
            || black_knights >= 3
    }
}
