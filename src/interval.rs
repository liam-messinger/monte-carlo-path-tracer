#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    // Constsants
    pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
    pub const UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };

    // Constructors
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    // Create the interval tightly enclosing the two input intervals.
    pub fn merge(a: &Interval, b: &Interval) -> Self {
        Self {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
    }

    pub const fn empty() -> Self {
        Self::EMPTY
    }

    pub const fn universe() -> Self {
        Self::UNIVERSE
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }

    pub fn surrounds(&self, value: f64) -> bool {
        value > self.min && value < self.max
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

// Default implementation for Interval
impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}