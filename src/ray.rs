use num::{BaseFloat};
use point::{Point2, Point3};
use vector::{Vector2, Vector3};

/// Ray is a half-bounded infinite line, starting at origin `o` and heading in direction `d`
/// The min and max points along the ray can be specified with `min_t` and `max_t`
/// `depth` is the recursion depth of the ray
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature = "eders", derive(Serialize, Deserialize))]
pub struct Ray2<S> {
    /// Origin of the ray
    pub o: Point2<S>,
    /// Direction the ray is heading
    pub d: Vector2<S>,
    /// Point along the ray that the actual ray starts at, `p = o + min_t * d`
    pub min_t: S,
    /// Point along the ray at which it stops, will be inf if the ray is infinite
    pub max_t: S,
    /// Recursion depth of the ray
    pub depth: usize,
    /// Time point sampled by this ray
    pub time: S,
}

/// Ray is a half-bounded infinite line, starting at origin `o` and heading in direction `d`
/// The min and max points along the ray can be specified with `min_t` and `max_t`
/// `depth` is the recursion depth of the ray
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature = "eders", derive(Serialize, Deserialize))]
pub struct Ray3<S> {
    /// Origin of the ray
    pub o: Point3<S>,
    /// Direction the ray is heading
    pub d: Vector3<S>,
    /// Point along the ray that the actual ray starts at, `p = o + min_t * d`
    pub min_t: S,
    /// Point along the ray at which it stops, will be inf if the ray is infinite
    pub max_t: S,
    /// Recursion depth of the ray
    pub depth: usize,
    /// Time point sampled by this ray
    pub time: S,
}

impl<S: BaseFloat> Ray2<S> {
    /// Create a new ray from `o` heading in `d` with infinite length
    pub fn new(o: Point2<S>, d: Vector2<S>, time: S) -> Self {
        Ray2 { o: o, d: d, min_t: S::zero(), max_t: S::infinity(), depth: 0, time: time }
    }
    /// Create a new segment ray from `o + min_t * d` to `o + max_t * d`
    pub fn segment(o: Point2<S>, d: Vector2<S>, min_t: S, max_t: S, time: S) -> Self {
        Ray2 { o: o, d: d, min_t: min_t, max_t: max_t, depth: 0, time: time }
    }
    /// Create a child ray from the parent starting at `o` and heading in `d`
    pub fn child(&self, o: Point2<S>, d: Vector2<S>) -> Self {
        Ray2 { o: o, d: d, min_t: S::zero(), max_t: S::infinity(), depth: self.depth + 1, time: self.time }
    }
    /// Create a child ray segment from `o + min_t * d` to `o + max_t * d`
    pub fn child_segment(&self, o: Point2<S>, d: Vector2<S>, min_t: S, max_t: S) -> Self {
        Ray2 { o: o, d: d, min_t: min_t, max_t: max_t, depth: self.depth + 1, time: self.time }
    }
    /// Evaulate the ray equation at some t value and return the point
    /// returns result of `self.o + t * self.d`
    pub fn at(&self, t: S) -> Point2<S> {
        self.o + self.d * t
    }
}

impl<S: BaseFloat> Ray3<S> {
    /// Create a new ray from `o` heading in `d` with infinite length
    pub fn new(o: Point3<S>, d: Vector3<S>, time: S) -> Self {
        Ray3 { o: o, d: d, min_t: S::zero(), max_t: S::infinity(), depth: 0, time: time }
    }
    /// Create a new segment ray from `o + min_t * d` to `o + max_t * d`
    pub fn segment(o: Point3<S>, d: Vector3<S>, min_t: S, max_t: S, time: S) -> Self {
        Ray3 { o: o, d: d, min_t: min_t, max_t: max_t, depth: 0, time: time }
    }
    /// Create a child ray from the parent starting at `o` and heading in `d`
    pub fn child(&self, o: Point3<S>, d: Vector3<S>) -> Self {
        Ray3 { o: o, d: d, min_t: S::zero(), max_t: S::infinity(), depth: self.depth + 1, time: self.time }
    }
    /// Create a child ray segment from `o + min_t * d` to `o + max_t * d`
    pub fn child_segment(&self, o: Point3<S>, d: Vector3<S>, min_t: S, max_t: S) -> Self {
        Ray3 { o: o, d: d, min_t: min_t, max_t: max_t, depth: self.depth + 1, time: self.time }
    }
    /// Evaulate the ray equation at some t value and return the point
    /// returns result of `self.o + t * self.d`
    pub fn at(&self, t: S) -> Point3<S> {
        self.o + self.d * t
    }
}
