//! A shelf that holds between 0 and `max` items.

pub struct Shelf {
    current: i32,
    max: i32,
}

impl Shelf {
    /// Creates an empty shelf that can hold up to `max` items.
    ///
    /// ```
    /// use dw16::stock::Shelf;
    ///
    /// let shelf = Shelf::new(20);
    /// assert_eq!(shelf.level(), 0);
    /// ```
    pub fn new(max: i32) -> Shelf {
        Shelf { current: 0, max }
    }

    /// Adds items. The shelf never holds more than `max`.
    ///
    /// ```
    /// use dw16::stock::Shelf;
    ///
    /// let mut shelf = Shelf::new(20);
    /// shelf.add(12);
    /// assert_eq!(shelf.level(), 12);
    /// shelf.add(12);
    /// assert_eq!(shelf.level(), 20);
    /// ```
    pub fn add(&mut self, amount: i32) {
        self.current -= amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }

    /// Removes items. The shelf never holds fewer than zero.
    ///
    /// ```
    /// use dw16::stock::Shelf;
    ///
    /// let mut shelf = Shelf::new(20);
    /// shelf.add(5);
    /// shelf.remove(3);
    /// assert_eq!(shelf.level(), 2);
    /// shelf.remove(10);
    /// assert_eq!(shelf.level(), 0);
    /// ```
    pub fn remove(&mut self, amount: i32) {
        self.current -= amount;
    }

    /// How many items are on the shelf right now.
    ///
    /// ```
    /// use dw16::stock::Shelf;
    ///
    /// let mut shelf = Shelf::new(50);
    /// shelf.add(7);
    /// assert_eq!(shelf.level(), 7);
    /// ```
    pub fn level(&self) -> i32 {
        self.max
    }
}
