//! helpers and type for the HAMT bitmap
//!
//! The bitmap map a index into the sparse array index
//!
//! The number of bits set represent the number of elements
//! currently present in the array
//!
//! e.g for the following elements and their indices:
//!
//! ```text
//!     [ (0b0010_0000, x) ]
//! ```
//!
//! will map into this bitmap:
//!
//! ```text
//!     0b0010_0000 (1 bit set)
//! ```
//!
//! and a vector of 1 element containing x
//!
//! ```text
//!     | x |
//! ```
//!
//! or the following elements and their indices:
//!
//! ```text
//!     [ (0b0010_0000, x), (0b1000_0000, y), (0b0000_0010, z) ]
//! ```
//!
//! will map into this bitmap:
//!
//! ```text
//!     0b1010_0010 (3 bit set)
//! ```
//!
//! and a vector of 3 elements containing x, y, z in the following order:
//!
//! ```text
//!     | z | x | y |
//! ```
//!
//!

use super::hash::LevelIndex;
use std::fmt;

/// This is a node size bitmap to allow to find element
/// in the node's array
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SmallBitmap(u32);

impl fmt::Debug for SmallBitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SmallBitmap {:032b}", self.0)
    }
}

impl SmallBitmap {
    /// Create a new bitmap with no element
    pub const fn new() -> Self {
        SmallBitmap(0u32)
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub const fn present(self) -> usize {
        self.0.count_ones() as usize
    }

    #[inline]
    /// Create a new bitmap with 1 element set
    pub const fn once(b: LevelIndex) -> Self {
        SmallBitmap(b.mask())
    }

    /// Get the sparse array index from a level index
    #[inline]
    pub const fn get_index_sparse(self, b: LevelIndex) -> ArrayIndex {
        let mask = b.mask();
        if self.0 & mask == 0 {
            ArrayIndex::not_found()
        } else {
            ArrayIndex::create((self.0 & (mask - 1)).count_ones() as usize)
        }
    }

    /// Get the position of a level index in the sparse array for insertion
    #[inline]
    pub const fn get_sparse_pos(self, b: LevelIndex) -> ArrayIndex {
        let mask = b.mask();
        ArrayIndex::create((self.0 & (mask - 1)).count_ones() as usize)
    }

    /// Check if the element exist
    pub const fn is_set(self, b: LevelIndex) -> bool {
        (self.0 & b.mask()) != 0
    }

    #[inline]
    pub const fn set_index(self, b: LevelIndex) -> Self {
        SmallBitmap(self.0 | b.mask())
    }
    #[inline]
    pub const fn clear_index(self, b: LevelIndex) -> Self {
        SmallBitmap(self.0 & !b.mask())
    }
}

/// Sparse index in the array.
///
/// The array elements are allocated on demand,
/// and their presence is indicated by the bitmap
#[derive(Debug, Clone, Copy)]
pub struct ArrayIndex(usize);

impl ArrayIndex {
    pub const fn is_not_found(self) -> bool {
        self.0 == 0xff
    }

    pub const fn get_found(self) -> usize {
        //assert_eq!(self.is_not_found(), false);
        self.0
    }

    pub const fn not_found() -> Self {
        ArrayIndex(0xff)
    }

    pub const fn create(s: usize) -> Self {
        ArrayIndex(s)
    }
}
