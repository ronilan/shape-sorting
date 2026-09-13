use incredible::UnderlineKind;
use rand::{rng, seq::SliceRandom};

use crate::core::State;
use crate::core::constants::{COLORS, SYMBOLS};
use crate::elements::{ColoredShape, ColoredShapeOptions};

const SHAPE_COUNT: usize = 64;

/// Builds the randomized set of draggable shapes for the game.
pub(crate) fn build() -> Vec<ColoredShape<State>> {
    let mut rng = rng();

    let mut shapes_a = Vec::new();
    for &symbol in &SYMBOLS {
        let mut cols = COLORS.to_vec();
        cols.shuffle(&mut rng);
        for &color in &cols[0..4] {
            shapes_a.push((color, symbol));
        }
    }

    let mut shapes_b = Vec::new();
    for &color in &COLORS {
        let mut syms = SYMBOLS.to_vec();
        syms.shuffle(&mut rng);
        for &symbol in &syms[0..4] {
            shapes_b.push((color, symbol));
        }
    }

    let mut combos = Vec::new();
    combos.extend(shapes_a);
    combos.extend(shapes_b);
    combos.shuffle(&mut rng);

    let mut shapes = Vec::new();
    for idx in 0..SHAPE_COUNT {
        let (color, symbol) = combos[idx];

        let shape = ColoredShape::new(ColoredShapeOptions {
            color,
            shape: symbol,
            draggable: true,
        });
        shape
            .decoration
            .style
            .focused
            .decor
            .underline
            .set(Some(UnderlineKind::Single));
        shape
            .decoration
            .style
            .entered
            .decor
            .underline
            .set(Some(UnderlineKind::Single));

        shapes.push(shape);
    }

    shapes
}
