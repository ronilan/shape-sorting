use std::cell::Cell;

use incredible::*;
use incredible_elements::{Frame, FrameKind};
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;
use incredible_macros_decl::element_for;

use crate::core::State;
use crate::core::constants::{EMPTY_SYMBOLS, SYMBOLS};
use crate::core::enums::{GameState, GameType};
use crate::elements::ColoredShape;
use crate::game::ui;

const FLOW_CONFIG_DEFAULT: FlowConfig = FlowConfig {
    direction: Direction::Right,
    wrap_direction: Direction::Down,
    align: Align::Start,
    item_space: 0,
    wrap_space: 0,
};

// focus mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusMode {
    ShapeBox,
    DropBoxes,
    InsideDropBox,
}

// element state
element_for! {
  pub struct ShapeSorting for State {
    focus_mode: Cell<FocusMode> = Cell::new(FocusMode::ShapeBox),
    focused_drop_box: Cell<Option<usize>> = Cell::new(None),
    focused_shape: Cell<Option<usize>> = Cell::new(None),
    focused_shape_in_drop_box: Cell<Option<usize>> = Cell::new(None),
  }
}

impl ShapeSorting {
    /// Builds the shape sorting game element and wires input handlers.
    pub fn new() -> Self {
        let el = Self::blank();
        el.look(Look::from((80, 24)));

        el
            // Keyboard input drives focus movement and pick/drop actions in Key/Mix modes.
            // Starts the timer by moving Ready -> Running on the first navigation key.
            .internal_on_key(|el, state, event| {
                // Disable Ctrl+x and Ctrl+v if game_type is Mouse
                if state.game_type == GameType::Mouse {
                    return;
                }

                // TODO: the idea is to start the timer, however reset button which is a
                // sub element also has on_key that sets game to Ready.
                // There is no stop propagation, so another method should be used.
                if state.game_state == GameState::Ready {
                    state.game_state = GameState::Running;
                }

                match event.key {
                    Key::Enter => match el.focus_mode.get() {
                        FocusMode::DropBoxes => {
                            if let Some(drop_idx) = el.focused_drop_box.get() {
                                let frames = el.elements.cot::<Frame<State>>();
                                let drop_box_count = frames.len().saturating_sub(1);

                                if drop_idx < drop_box_count {
                                    let shapes =
                                        frames[drop_idx].elements.cot::<ColoredShape<State>>();
                                    if !shapes.is_empty() {
                                        el.focus_mode.set(FocusMode::InsideDropBox);
                                        el.focused_shape_in_drop_box.set(Some(0));
                                        el.apply_focus();
                                    }
                                }
                            }
                        }
                        FocusMode::InsideDropBox => {
                            el.focus_mode.set(FocusMode::DropBoxes);
                            el.focused_shape_in_drop_box.set(None);
                            el.apply_focus();
                        }
                        _ => {}
                    },
                    Key::Char(' ') => {
                        match el.focus_mode.get() {
                            FocusMode::ShapeBox => {
                                if el.focused_drop_box.get().is_none() {
                                    el.focused_drop_box.set(Some(0));
                                }
                                el.focus_mode.set(FocusMode::DropBoxes);
                            }
                            FocusMode::DropBoxes | FocusMode::InsideDropBox => {
                                if el.focused_shape.get().is_none() {
                                    el.focused_shape.set(Some(0));
                                }
                                el.focus_mode.set(FocusMode::ShapeBox);
                                el.focused_shape_in_drop_box.set(None);
                            }
                        }
                        el.apply_focus();
                    }
                    Key::Left => el.shift_focus(-1),
                    Key::Right => el.shift_focus(1),
                    Key::Up => el.shift_focus_vertical(-1),
                    Key::Down => el.shift_focus_vertical(1),
                    // Ctrl+x: pick up the focused shape (like mouse drag)
                    Key::Char('x') if event.modifiers.contains(&KeyMod::Ctrl) => {
                        if el.elements.cot::<ColoredShape<State>>().first().is_none() {
                            let frames = el.elements.cot::<Frame<State>>();

                            let mut picked_shape = None;
                            match el.focus_mode.get() {
                                FocusMode::ShapeBox => {
                                    if let Some(shape_box) = frames.last() {
                                        picked_shape = shape_box
                                            .elements
                                            .sot_w::<ColoredShape<State>, _>(|s| {
                                                s.status.focused.get()
                                            })
                                            .or_else(|| {
                                                shape_box.elements.sot::<ColoredShape<State>>()
                                            })
                                            .map(|(_, shape)| shape);

                                        shape_box
                                            .elements_snap_left()
                                            .elements_snap_top()
                                            .elements_flow(FLOW_CONFIG_DEFAULT);
                                    }
                                }
                                FocusMode::DropBoxes => {
                                    let drop_box_count = frames.len().saturating_sub(1);
                                    if let Some(idx) = el.focused_drop_box.get() {
                                        if idx < drop_box_count {
                                            picked_shape = frames[idx]
                                                .elements
                                                .sot::<ColoredShape<State>>()
                                                .map(|(_, shape)| shape);

                                            frames[idx]
                                                .elements_snap_left()
                                                .elements_snap_top()
                                                .elements_flow(FLOW_CONFIG_DEFAULT);
                                        }
                                    }
                                }
                                FocusMode::InsideDropBox => {
                                    let drop_box_count = frames.len().saturating_sub(1);
                                    if let Some(drop_idx) = el.focused_drop_box.get() {
                                        if drop_idx < drop_box_count {
                                            picked_shape = frames[drop_idx]
                                                .elements
                                                .sot_w::<ColoredShape<State>, _>(|s| {
                                                    s.status.focused.get()
                                                })
                                                .or_else(|| {
                                                    frames[drop_idx]
                                                        .elements
                                                        .sot::<ColoredShape<State>>()
                                                })
                                                .map(|(_, shape)| shape);

                                            frames[drop_idx]
                                                .elements_snap_left()
                                                .elements_snap_top()
                                                .elements_flow(FLOW_CONFIG_DEFAULT);

                                            el.focused_shape_in_drop_box.set(None);
                                        }
                                    }
                                }
                            }

                            if let Some(shape) = picked_shape {
                                if let Some(shape_box) = frames.last() {
                                    shape
                                        .x(shape_box.get_x() + shape_box.get_width() as isize + 2)
                                        .y(shape_box.get_y() + 2)
                                        .focused(false);
                                }
                                el.elements.inner.borrow_mut().push(shape);

                                if el.focus_mode.get() == FocusMode::ShapeBox
                                    && el.focused_drop_box.get().is_none()
                                {
                                    el.focused_drop_box.set(Some(0));
                                }
                                el.focus_mode.set(FocusMode::DropBoxes);
                                el.apply_focus();
                            }
                        }
                    }

                    // Ctrl+v: drop the floating shape (like mouse up)
                    Key::Char('v') if event.modifiers.contains(&KeyMod::Ctrl) => {
                        if let Some((_, shape)) = el.elements.sot::<ColoredShape<State>>() {
                            let frames = el.elements.cot::<Frame<State>>();
                            let mut placed = false;

                            for frame in &frames {
                                if frame.get_focused()
                                    && Self::is_match(frame, &shape)
                                    && frame.elements.len() < 4
                                {
                                    frame.elements.inner.borrow_mut().push(shape.clone());
                                    frame
                                        .elements_snap_left()
                                        .elements_snap_top()
                                        .elements_flow(FLOW_CONFIG_DEFAULT);
                                    placed = true;
                                    break;
                                }
                            }

                            if !placed {
                                shape.dragged(false);

                                if let Some(shape_box) = frames.last() {
                                    shape_box.elements.inner.borrow_mut().push(shape);
                                    shape_box
                                        .elements_snap_left()
                                        .elements_snap_top()
                                        .elements_flow(FLOW_CONFIG_DEFAULT);
                                } else {
                                    el.elements.inner.borrow_mut().push(shape);
                                }
                            }
                        }
                        el.focus_mode.set(FocusMode::ShapeBox);
                        el.apply_focus();
                        el.update_completion(state);
                    }
                    // Esc cancels a carried shape like releasing mouse in empty space.
                    Key::Escape => {
                        if let Some((_, shape)) = el.elements.sot::<ColoredShape<State>>() {
                            let frames = el.elements.cot::<Frame<State>>();

                            shape.dragged(false);

                            if let Some(shape_box) = frames.last() {
                                shape_box.elements.inner.borrow_mut().push(shape);
                                shape_box
                                    .elements_snap_left()
                                    .elements_snap_top()
                                    .elements_flow(FLOW_CONFIG_DEFAULT);
                            } else {
                                el.elements.inner.borrow_mut().push(shape);
                            }
                        }
                        el.focus_mode.set(FocusMode::ShapeBox);
                        el.apply_focus();
                    }

                    _ => {}
                }
            })
            // Mouse input handles drag/drop, while keeping focus logic consistent for Mix mode.
            .internal_on_mouse(|el, state: &mut State, event| {
                // Disable mouse drag/drop if game_type is Key
                if state.game_type == GameType::Key {
                    return;
                }

                if event.mouse == Mouse::Move || event.mouse == Mouse::Down {
                    el.clear_focus();
                }
                if event.mouse == Mouse::Drag {
                    let frames = el.elements.cot::<Frame<State>>();
                    let mut dragged_opt = None;

                    for frame in &frames {
                        if let Some((_, dragged)) = frame
                            .elements
                            .sot_w::<ColoredShape<State>, _>(|el| el.status.dragged.get())
                        {
                            frame
                                .elements_snap_left()
                                .elements_snap_top()
                                .elements_flow(FLOW_CONFIG_DEFAULT);

                            dragged_opt = Some(dragged);
                            break;
                        }
                    }

                    if let Some(dragged) = dragged_opt {
                        dragged.focused(false);
                        el.elements.inner.borrow_mut().push(dragged);
                    }

                    // For mix mode - switch focus to shape_box when drag is performed
                    el.focus_mode.set(FocusMode::ShapeBox);
                    el.apply_focus();
                }

                if event.mouse == Mouse::Up {
                    if let Some((_, shape)) = el.elements.sot::<ColoredShape<State>>() {
                        let frames = el.elements.cot::<Frame<State>>();
                        let mut placed = false;

                        for frame in &frames {
                            if frame.status.entered.get()
                                && Self::is_match(frame, &shape)
                                && frame.elements.len() < 4
                            {
                                frame.elements.inner.borrow_mut().push(shape.clone());
                                frame
                                    .elements_snap_left()
                                    .elements_snap_top()
                                    .elements_flow(FLOW_CONFIG_DEFAULT);
                                placed = true;
                                break;
                            }
                        }

                        if !placed {
                            shape.dragged(false);

                            if let Some(shape_box) = frames.last() {
                                shape_box.elements.inner.borrow_mut().push(shape);
                                shape_box
                                    .elements_snap_left()
                                    .elements_snap_top()
                                    .elements_flow(FLOW_CONFIG_DEFAULT);
                            } else {
                                el.elements.inner.borrow_mut().push(shape);
                            }
                        }
                    }
                    el.update_completion(state);
                }
            })
            // If the pointer leaves a drop box during drag, cancel the drag and
            // return the shape back to the shape box so it doesn't get lost.
            .internal_on_change(|el, _state, event| {
                if event.changes.contains(&Change::Entered(false)) {
                    if let Some((_, dropped)) = el
                        .elements
                        .sot_w::<ColoredShape<State>, _>(|s| s.status.dragged.get())
                    {
                        // Set the element status.
                        dropped.dragged(false);

                        let frames = el.elements.cot::<Frame<State>>();
                        if let Some(shape_box) = frames.last() {
                            shape_box.elements.inner.borrow_mut().push(dropped);
                            shape_box
                                .elements_snap_left()
                                .elements_snap_top()
                                .elements_flow(FLOW_CONFIG_DEFAULT);
                        } else {
                            el.elements.inner.borrow_mut().push(dropped);
                        }
                        el.draw();
                    }
                }
            })
            // Rebuild the game layout when the splash screen selects a mode (Ready state).
            .internal_on_state(|el, state, _ev| {
                if state.game_state == GameState::Ready {
                    el.setup_game(state.game_type);
                }
            });

        el
    }

    // focus engine

    fn is_match(frame: &Frame<State>, shape: &ColoredShape<State>) -> bool {
        let matches_shape = SYMBOLS
            .iter()
            .position(|&c| c == shape.options.shape)
            .and_then(|idx| EMPTY_SYMBOLS.get(idx))
            == Some(&frame.get_fill().unwrap_or_default());

        let matches_color = frame.get_color() == Some(shape.options.color.into());

        matches_shape || matches_color
    }

    fn update_completion(&self, state: &mut State) {
        let frames = self.elements.cot::<Frame<State>>();
        if let Some(shape_box) = frames.last() {
            if shape_box.elements.len() == 0 && state.game_state == GameState::Running {
                state.game_state = GameState::Completed;
                if state.stats.last_game_time > 0
                    && (state.stats.fastest_game == 0
                        || state.stats.last_game_time < state.stats.fastest_game)
                {
                    state.stats.fastest_game = state.stats.last_game_time;
                }
            }
        }
    }

    /// Clears focus from all frames and shapes.
    fn clear_focus(&self) {
        let frames = self.elements.cot::<Frame<State>>();
        for frame in frames {
            frame.focused(false);
            for shape in frame.elements.cot::<ColoredShape<State>>() {
                shape.focused(false);
            }
        }
    }

    /// Applies focus based on the current focus mode and stored indices.
    fn apply_focus(&self) {
        let frames = self.elements.cot::<Frame<State>>();
        if frames.is_empty() {
            return;
        }

        self.clear_focus();

        match self.focus_mode.get() {
            FocusMode::DropBoxes => {
                let drop_box_count = frames.len().saturating_sub(1);
                if drop_box_count == 0 {
                    return;
                }

                if let Some(idx) = self.focused_drop_box.get() {
                    let idx = idx % drop_box_count;
                    self.focused_drop_box.set(Some(idx));
                    frames[idx].focused(true);
                }
            }

            FocusMode::ShapeBox => {
                let Some(shape_box) = frames.last() else {
                    return;
                };
                let shapes = shape_box.elements.cot::<ColoredShape<State>>();
                if shapes.is_empty() {
                    return;
                }

                if let Some(idx) = self.focused_shape.get() {
                    let idx = idx % shapes.len();
                    self.focused_shape.set(Some(idx));

                    shape_box.focused(true);
                    shapes[idx].focused(true);
                }
            }

            FocusMode::InsideDropBox => {
                if let Some(drop_idx) = self.focused_drop_box.get() {
                    let drop_box_count = frames.len().saturating_sub(1);
                    if drop_idx < drop_box_count {
                        frames[drop_idx].focused(true);

                        let shapes = frames[drop_idx].elements.cot::<ColoredShape<State>>();
                        if !shapes.is_empty() {
                            let shape_idx =
                                self.focused_shape_in_drop_box.get().unwrap_or(0) % shapes.len();
                            shapes[shape_idx].focused(true);
                            self.focused_shape_in_drop_box.set(Some(shape_idx));
                        }
                    }
                }
            }
        }
    }

    /// Shifts focus left/right across shapes or drop boxes.
    fn shift_focus(&self, delta: isize) {
        match self.focus_mode.get() {
            FocusMode::ShapeBox => {
                let frames = self.elements.cot::<Frame<State>>();
                if let Some(shape_box) = frames.last() {
                    let len = shape_box.elements.len() as isize;
                    if len > 0 {
                        let idx = match self.focused_shape.get() {
                            Some(current) => (current as isize + delta + len) % len,
                            None => 0, // start at first shape if none
                        };
                        self.focused_shape.set(Some(idx as usize));
                    }
                }
            }
            FocusMode::DropBoxes => {
                let frames = self.elements.cot::<Frame<State>>();
                let drop_len = frames.len().saturating_sub(1) as isize;
                if drop_len > 0 {
                    let idx = match self.focused_drop_box.get() {
                        Some(current) => (current as isize + delta + drop_len) % drop_len,
                        None => 0, // start at first drop box if none
                    };
                    self.focused_drop_box.set(Some(idx as usize));
                }
            }
            FocusMode::InsideDropBox => {
                if let Some(drop_idx) = self.focused_drop_box.get() {
                    let frames = self.elements.cot::<Frame<State>>();
                    let drop_box_count = frames.len().saturating_sub(1);

                    if drop_idx < drop_box_count {
                        let len = frames[drop_idx].elements.len() as isize;
                        if len > 0 {
                            let idx = match self.focused_shape_in_drop_box.get() {
                                Some(current) => (current as isize + delta + len) % len,
                                None => 0,
                            };
                            self.focused_shape_in_drop_box.set(Some(idx as usize));
                        }
                    }
                }
            }
        }
        self.apply_focus();
    }

    /// Shifts focus up/down across rows in the shape box with wrap-around.
    fn shift_focus_vertical(&self, delta: isize) {
        match self.focus_mode.get() {
            FocusMode::ShapeBox => {
                let frames = self.elements.cot::<Frame<State>>();
                let Some(shape_box) = frames.last() else {
                    return;
                };

                let shapes = shape_box.elements.cot::<ColoredShape<State>>();
                if shapes.is_empty() {
                    return;
                }

                let current_idx = self.focused_shape.get().unwrap_or(0) % shapes.len();
                let current = &shapes[current_idx];
                let current_x = current.get_x();
                let current_y = current.get_y();

                let mut candidate_y: Option<isize> = None;
                for shape in &shapes {
                    let y = shape.get_y();
                    if delta < 0 && y < current_y {
                        candidate_y = Some(candidate_y.map_or(y, |best| best.max(y)));
                    } else if delta > 0 && y > current_y {
                        candidate_y = Some(candidate_y.map_or(y, |best| best.min(y)));
                    }
                }

                let target_y = match candidate_y {
                    Some(y) => y,
                    None => {
                        let mut extreme: Option<isize> = None;
                        for shape in &shapes {
                            let y = shape.get_y();
                            if delta < 0 {
                                // wrap to bottom row
                                extreme = Some(extreme.map_or(y, |best| best.max(y)));
                            } else {
                                // wrap to top row
                                extreme = Some(extreme.map_or(y, |best| best.min(y)));
                            }
                        }
                        let Some(y) = extreme else {
                            return;
                        };
                        y
                    }
                };

                let mut best_idx: Option<usize> = None;
                let mut best_dx: isize = isize::MAX;
                for (idx, shape) in shapes.iter().enumerate() {
                    if shape.get_y() == target_y {
                        let dx = (shape.get_x() - current_x).abs();
                        if dx < best_dx {
                            best_idx = Some(idx);
                            best_dx = dx;
                        }
                    }
                }

                if let Some(idx) = best_idx {
                    self.focused_shape.set(Some(idx));
                    self.apply_focus();
                }
            }
            FocusMode::DropBoxes => {
                let frames = self.elements.cot::<Frame<State>>();
                let drop_count = frames.len().saturating_sub(1);
                if drop_count == 0 {
                    return;
                }
                let drop_frames = &frames[..drop_count];
                let current_idx = self.focused_drop_box.get().unwrap_or(0) % drop_count;
                let current = &drop_frames[current_idx];
                let current_x = current.get_x();
                let current_y = current.get_y();

                let mut candidate_y: Option<isize> = None;
                for frame in drop_frames {
                    let y = frame.get_y();
                    if delta < 0 && y < current_y {
                        candidate_y = Some(candidate_y.map_or(y, |best| best.max(y)));
                    } else if delta > 0 && y > current_y {
                        candidate_y = Some(candidate_y.map_or(y, |best| best.min(y)));
                    }
                }

                let target_y = match candidate_y {
                    Some(y) => y,
                    None => {
                        let mut extreme: Option<isize> = None;
                        for frame in drop_frames {
                            let y = frame.get_y();
                            if delta < 0 {
                                extreme = Some(extreme.map_or(y, |best| best.max(y)));
                            } else {
                                extreme = Some(extreme.map_or(y, |best| best.min(y)));
                            }
                        }
                        let Some(y) = extreme else {
                            return;
                        };
                        y
                    }
                };

                let mut best_idx: Option<usize> = None;
                let mut best_dx: isize = isize::MAX;
                for (idx, frame) in drop_frames.iter().enumerate() {
                    if frame.get_y() == target_y {
                        let dx = (frame.get_x() - current_x).abs();
                        if dx < best_dx {
                            best_idx = Some(idx);
                            best_dx = dx;
                        }
                    }
                }

                if let Some(idx) = best_idx {
                    self.focused_drop_box.set(Some(idx));
                    self.apply_focus();
                }
            }
            FocusMode::InsideDropBox => {
                if let Some(drop_idx) = self.focused_drop_box.get() {
                    let frames = self.elements.cot::<Frame<State>>();
                    let drop_box_count = frames.len().saturating_sub(1);

                    if drop_idx < drop_box_count {
                        let shapes = frames[drop_idx].elements.cot::<ColoredShape<State>>();
                        if shapes.is_empty() {
                            return;
                        }

                        let current_idx =
                            self.focused_shape_in_drop_box.get().unwrap_or(0) % shapes.len();
                        let current = &shapes[current_idx];
                        let current_x = current.get_x();
                        let current_y = current.get_y();

                        let mut candidate_y: Option<isize> = None;
                        for shape in &shapes {
                            let y = shape.get_y();
                            if delta < 0 && y < current_y {
                                candidate_y = Some(candidate_y.map_or(y, |best| best.max(y)));
                            } else if delta > 0 && y > current_y {
                                candidate_y = Some(candidate_y.map_or(y, |best| best.min(y)));
                            }
                        }

                        let target_y = match candidate_y {
                            Some(y) => y,
                            None => {
                                let mut extreme: Option<isize> = None;
                                for shape in &shapes {
                                    let y = shape.get_y();
                                    if delta < 0 {
                                        extreme = Some(extreme.map_or(y, |best| best.max(y)));
                                    } else {
                                        extreme = Some(extreme.map_or(y, |best| best.min(y)));
                                    }
                                }
                                let Some(y) = extreme else {
                                    return;
                                };
                                y
                            }
                        };

                        let mut best_idx: Option<usize> = None;
                        let mut best_dx: isize = isize::MAX;
                        for (idx, shape) in shapes.iter().enumerate() {
                            if shape.get_y() == target_y {
                                let dx = (shape.get_x() - current_x).abs();
                                if dx < best_dx {
                                    best_idx = Some(idx);
                                    best_dx = dx;
                                }
                            }
                        }

                        if let Some(idx) = best_idx {
                            self.focused_shape_in_drop_box.set(Some(idx));
                            self.apply_focus();
                        }
                    }
                }
            }
        }
    }

    // game setup
    /// (Re)builds the board for the selected game type and seeds random shapes.
    pub fn setup_game(&self, game_type: GameType) {
        self.elements.empty();

        let offset_x = 0;
        let offset_y = 0;

        // drop boxes
        let frames = ui::drop_boxes::build();
        if game_type == GameType::Mouse {
            for frame in &frames {
                frame.frame_style.entered.kind.set(Some(FrameKind::Double));
            }
        }
        for frame in frames {
            frame.x(frame.get_x().saturating_add(offset_x));
            frame.y(frame.get_y().saturating_add(offset_y));
            self.add(frame);
        }

        // shape box
        let shape_box = ui::shape_box::build();
        if game_type == GameType::Mouse {
            shape_box
                .frame_style
                .entered
                .kind
                .set(Some(FrameKind::Double));
        }
        shape_box.x(shape_box.get_x().saturating_add(offset_x));
        shape_box.y(shape_box.get_y().saturating_add(offset_y));

        for shape in ui::shapes::build() {
            if game_type == GameType::Key {
                shape.draggable(false);
                shape.decoration.style.entered.decor.underline.set(None);
            }
            shape_box.add(shape);
        }

        shape_box
            .elements_snap_left()
            .elements_snap_top()
            .elements_flow(FLOW_CONFIG_DEFAULT);

        self.add(shape_box);

        // initial focus
        self.focus_mode.set(FocusMode::ShapeBox);
        self.focused_shape.set(None);
        self.focused_drop_box.set(None);
        self.focused_shape_in_drop_box.set(None);
        self.apply_focus();
    }
}

impl Default for ShapeSorting {
    /// Default constructor for the shape sorting element.
    fn default() -> Self {
        Self::new()
    }
}
