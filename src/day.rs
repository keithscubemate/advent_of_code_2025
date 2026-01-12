//! Common trait definition for Advent of Code puzzle solutions.
//!
//! This module defines the [`Day`] trait that all daily puzzle implementations
//! must implement, providing a consistent interface for solving both parts of
//! each day's challenge.

/// Trait implemented by each day's puzzle solution.
///
/// Each day in Advent of Code consists of two parts (A and B) that operate
/// on the same input but typically with different logic or complexity.
pub trait Day {
    /// Solves part A of the puzzle.
    ///
    /// # Arguments
    /// * `lines` - The puzzle input as a slice of strings, one per line.
    ///
    /// # Returns
    /// The solution as a string representation.
    fn part_a(lines: &[String]) -> String;

    /// Solves part B of the puzzle.
    ///
    /// # Arguments
    /// * `lines` - The puzzle input as a slice of strings, one per line.
    ///
    /// # Returns
    /// The solution as a string representation.
    fn part_b(lines: &[String]) -> String;
}
