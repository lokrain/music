//! notation/music-engraving/src/geometry.rs
//! Layout geometry primitives.

/// Position in engraving units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutPosition {
    pub x: f32,
    pub y: f32,
}

impl LayoutPosition {
    /// Create a new position.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Translate the position by `(dx, dy)`.
    #[must_use]
    pub fn translate(self, dx: f32, dy: f32) -> Self {
        Self { x: self.x + dx, y: self.y + dy }
    }
}

/// Bounding box for a glyph or group (origin at top-left).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl LayoutBox {
    /// Construct a bounding box ensuring non-negative dimensions.
    ///
    /// # Panics
    ///
    /// Panics if `width` or `height` is negative or non-finite.
    #[must_use]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        assert!(width.is_finite() && width >= 0.0, "width must be non-negative and finite");
        assert!(height.is_finite() && height >= 0.0, "height must be non-negative and finite");
        Self { x, y, width, height }
    }

    /// Minimum x coordinate.
    #[must_use]
    pub fn min_x(&self) -> f32 {
        self.x
    }

    /// Minimum y coordinate.
    #[must_use]
    pub fn min_y(&self) -> f32 {
        self.y
    }

    /// Maximum x coordinate.
    #[must_use]
    pub fn max_x(&self) -> f32 {
        self.x + self.width
    }

    /// Maximum y coordinate.
    #[must_use]
    pub fn max_y(&self) -> f32 {
        self.y + self.height
    }

    /// Translate the box by the given delta.
    #[must_use]
    pub fn translate(self, dx: f32, dy: f32) -> Self {
        Self::new(self.x + dx, self.y + dy, self.width, self.height)
    }

    /// Check if the box contains a layout position.
    #[must_use]
    pub fn contains(&self, position: LayoutPosition) -> bool {
        position.x >= self.min_x()
            && position.x <= self.max_x()
            && position.y >= self.min_y()
            && position.y <= self.max_y()
    }

    /// Compute intersection box if any overlap exists.
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let min_x = self.min_x().max(other.min_x());
        let min_y = self.min_y().max(other.min_y());
        let max_x = self.max_x().min(other.max_x());
        let max_y = self.max_y().min(other.max_y());
        if max_x <= min_x || max_y <= min_y {
            return None;
        }
        Some(Self::new(min_x, min_y, max_x - min_x, max_y - min_y))
    }

    /// Whether two boxes intersect.
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }

    /// Union of two boxes.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let min_x = self.min_x().min(other.min_x());
        let min_y = self.min_y().min(other.min_y());
        let max_x = self.max_x().max(other.max_x());
        let max_y = self.max_y().max(other.max_y());
        Self::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }
}

/// System index in a page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemIndex(pub u16);

/// Page index in a score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageIndex(pub u16);

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: f32, y: f32, w: f32, h: f32) -> LayoutBox {
        LayoutBox::new(x, y, w, h)
    }

    #[test]
    fn intersection_and_union() {
        let a = rect(0.0, 0.0, 10.0, 5.0);
        let b = rect(5.0, 2.0, 10.0, 5.0);
        let intersection = a.intersection(&b).expect("boxes overlap");
        assert_eq!(intersection, rect(5.0, 2.0, 5.0, 3.0));
        assert!(a.intersects(&b));

        let union = a.union(&b);
        assert_eq!(union, rect(0.0, 0.0, 15.0, 7.0));
    }

    #[test]
    fn contains_points() {
        let box_ = rect(1.0, 1.0, 2.0, 2.0);
        assert!(box_.contains(LayoutPosition::new(2.0, 2.0)));
        assert!(!box_.contains(LayoutPosition::new(0.5, 2.0)));
    }

    #[test]
    fn translation_preserves_size() {
        let box_ = rect(0.0, 0.0, 4.0, 1.0).translate(10.0, -2.0);
        assert_eq!(box_.min_x(), 10.0);
        assert_eq!(box_.max_y(), -1.0);
    }
}
