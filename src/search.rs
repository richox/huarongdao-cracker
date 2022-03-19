use crate::state::Block;
use crate::state::State;
use crate::state::MoveOp;

pub fn search(state: &State, b4_target_y: usize, b4_target_x: usize) -> Option<Vec<State>> {
    let is_target = |state: &State| {
        state.get_block(b4_target_y, b4_target_x) == Some(Block::B4) &&
            state.find_block_rect(b4_target_y, b4_target_x)[0] == (b4_target_y, b4_target_x)
    };
    if is_target(state) {
        return Some(vec![*state]);
    }

    let init_state = *state;
    let mut searched = std::collections::HashMap::<State, State>::new(); // state, prev-state
    let mut searching = vec![init_state];
    searched.insert(init_state, init_state);

    while !searching.is_empty() {
        let mut next_searching = vec![];

        for &state in &searching {
            for (y0, x0) in state.find_block0s() {
                for ((sibling_y, sibling_x), op) in [
                    ((y0 - 1, x0), MoveOp::D),
                    ((y0 + 1, x0), MoveOp::U),
                    ((y0, x0 - 1), MoveOp::R),
                    ((y0, x0 + 1), MoveOp::L),
                ] {
                    let dblock = state.get_block(sibling_y, sibling_x);

                    if dblock.is_some() && dblock != Some(Block::B0) {
                        let [
                            (dblock_y1, dblock_x1),
                            (dblock_y2, dblock_x2),
                        ] = state.find_block_rect(sibling_y, sibling_x);

                        if let Some(next_state) = state.move_block(dblock_y1, dblock_x1, dblock_y2, dblock_x2, op) {
                            if !searched.contains_key(&next_state) {
                                searched.insert(next_state, state);
                                next_searching.push(next_state);

                                if is_target(&next_state) {
                                    let mut solved_states_seq = vec![];
                                    let mut state = next_state;
                                    while solved_states_seq.last() != Some(&init_state) {
                                        solved_states_seq.push(state);
                                        state = searched[&state];
                                    }
                                    solved_states_seq.reverse();
                                    return Some(solved_states_seq);
                                }
                            }
                        }
                    }
                }
            }
        }
        searching = next_searching;
    }
    None
}
