//! TODO: document this

use carton_common::prelude::{Size, Pos, Color};

/// TODO: document this
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum CoordinateMode {
    Origin,
    Previous,
}

/// TODO: document this
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum PolygonShape {
    Complex,
    NonConvex,
    Convex,
}

/// TODO: document this
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DrawCall<'a> {
    Filled,
    NonFilled,
    Color { color: Color },
    CoordinateMode { mode: CoordinateMode },

    Point { position: Pos },
    Points { positions: &'a [Pos] },

    Line(Pos, Pos),
    Lines { lines: &'a [(Pos, Pos)] },

    Segment(Pos, Pos),
    Segments { segments: &'a [(Pos, Pos)] },

    Rectangle { position: Pos, size: Size },

    Arc { position: Pos, size: Size, arc1: i16, arc2: i16 },

    Polygon { shape: PolygonShape, points: &'a [Pos] },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
