/// A structure representing a pair of integers for basic arithmetic operations.
#[derive(Debug)]
pub struct TwoDigit(pub i32, pub i32);

/// A structure representing three integers for basic arithmetic operations.
pub struct ThreeDigit(pub i32, pub i32, pub i32);

impl TwoDigit {
    /// Performs addition on the two numbers and returns the result.
    pub fn add(&self) -> i32 {
        self.0 + self.1
    }

    /// Performs subtraction on the two numbers and returns the result.
    pub fn sub(&self) -> i32 {
        self.0 - self.1
    }

    /// Performs multiplication on the two numbers and returns the result.
    pub fn mul(&self) -> i32 {
        self.0 * self.1
    }

    /// Performs division on the two numbers and returns the result.
    pub fn div(&self) -> i32 {
        self.0 / self.1
    }
}
impl ThreeDigit {
    /// Performs addition on the three numbers and returns the result.
    pub fn add(&self) -> i32 {
        self.0 + self.1 + self.2
    }

    /// Performs subtraction on the three numbers and returns the result.
    pub fn sub(&self) -> i32 {
        self.0 - self.1 - self.2
    }

    /// Performs multiplication on the three numbers and returns the result.
    pub fn mul(&self) -> i32 {
        self.0 * self.1 * self.2
    }

    /// Performs division on the three numbers and returns the result.
    pub fn div(&self) -> i32 {
        self.0 / self.1 / self.2
    }
}
