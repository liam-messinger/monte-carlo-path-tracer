use std::ops::Add;

/// A one-dimensional interval [min, max].
#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    // Constants
    pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
    pub const UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };

    /// Constructor for Interval.
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Create the interval tightly enclosing the two input intervals.
    pub fn merge(a: &Interval, b: &Interval) -> Self {
        Self {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
    }

    /// Create an empty interval.
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    /// Create a universe interval.
    pub const fn universe() -> Self {
        Self::UNIVERSE
    }

    /// Get the size of the interval.
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Check if the interval contains a given value.
    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }

    /// Check if the interval strictly surrounds a given value.
    pub fn surrounds(&self, value: f64) -> bool {
        value > self.min && value < self.max
    }

    /// Expand the interval by delta (total increase in size).
    pub fn expand(&mut self, delta: f64) {
        let padding = delta / 2.0;
        self.min -= padding;
        self.max += padding;
    }

    /// Pad the interval to ensure it has at least the given minimum size.
    pub fn pad_to_minimum(&mut self, delta: f64) {
        if self.size() < delta { 
            self.expand(delta);
        }
    }
}

// Default implementation for Interval
impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Add<f64> for Interval {
    // interval + f64
    type Output = Interval;
    fn add(self, displacement: f64) -> Interval {
        Interval {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }
}

impl Add<f64> for &Interval {
    // &interval + f64
    type Output = Interval;
    fn add(self, displacement: f64) -> Interval {
        Interval {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }
}

impl Add<Interval> for f64 {
    // f64 + interval
    type Output = Interval;
    fn add(self, inval: Interval) -> Interval {
        inval + self
    }
}