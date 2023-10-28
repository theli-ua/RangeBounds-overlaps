use std::ops::{
    Bound::{self, Excluded, Included, Unbounded},
    Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

pub trait RangeBounds<T: ?Sized> {
    /// Start index bound.
    ///
    /// Returns the start value as a `Bound`.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() {
    /// use std::ops::Bound::*;
    /// use std::ops::RangeBounds;
    ///
    /// assert_eq!((..10).start_bound(), Unbounded);
    /// assert_eq!((3..10).start_bound(), Included(&3));
    /// # }
    /// ```
    // #[stable(feature = "collections_range", since = "1.28.0")]
    fn start_bound(&self) -> Bound<&T>;

    /// End index bound.
    ///
    /// Returns the end value as a `Bound`.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() {
    /// use std::ops::Bound::*;
    /// use std::ops::RangeBounds;
    ///
    /// assert_eq!((3..).end_bound(), Unbounded);
    /// assert_eq!((3..10).end_bound(), Excluded(&10));
    /// # }
    /// ```
    // #[stable(feature = "collections_range", since = "1.28.0")]
    fn end_bound(&self) -> Bound<&T>;

    /// Returns `true` if `item` is contained in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!( (3..5).contains(&4));
    /// assert!(!(3..5).contains(&2));
    ///
    /// assert!( (0.0..1.0).contains(&0.5));
    /// assert!(!(0.0..1.0).contains(&f32::NAN));
    /// assert!(!(0.0..f32::NAN).contains(&0.5));
    /// assert!(!(f32::NAN..1.0).contains(&0.5));
    /// ```
    // #[stable(feature = "range_contains", since = "1.35.0")]
    fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        (match self.start_bound() {
            Included(start) => start <= item,
            Excluded(start) => start < item,
            Unbounded => true,
        }) && (match self.end_bound() {
            Included(end) => item <= end,
            Excluded(end) => item < end,
            Unbounded => true,
        })
    }

    /// Returns `true` if there exists an element present in both ranges.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!( (3..5).overlaps(&(1..4));
    /// assert!(!(3..5).overlaps(&(2..6));
    /// ```
    ///
    fn overlaps<O, E>(&self, other: &O) -> bool
    where
        T: PartialOrd<E>,
        E: ?Sized + PartialOrd<T>,
        O: RangeBounds<E>,
    {
        match (
            self.start_bound(),
            self.end_bound(),
            other.start_bound(),
            other.end_bound(),
        ) {
            (Unbounded, Unbounded, _, _) => true,
            (_, _, Unbounded, Unbounded) => true,
            (Unbounded, _, Unbounded, _) => true,
            (_, Unbounded, _, Unbounded) => true,
            (Included(s), Included(e), _, _) => other.contains(s) || other.contains(e),
            (_, _, Included(s), Included(e)) => self.contains(s) || self.contains(e),
            (Included(s), _, Included(o), _) => self.contains(o) || other.contains(s),
            (_, Included(s), _, Included(o)) => self.contains(o) || other.contains(s),
            (Included(_), Excluded(_), Excluded(_), Included(_)) => todo!(),
            (Included(_), Excluded(_), Excluded(_), Excluded(_)) => todo!(),
            (Included(_), Excluded(_), Excluded(_), Unbounded) => todo!(),
            (Included(_), Excluded(_), Unbounded, Included(_)) => todo!(),
            (Included(_), Excluded(_), Unbounded, Excluded(_)) => todo!(),
            (Included(_), Unbounded, Excluded(_), Included(_)) => todo!(),
            (Included(_), Unbounded, Excluded(_), Excluded(_)) => todo!(),
            (Included(_), Unbounded, Unbounded, Included(_)) => todo!(),
            (Included(_), Unbounded, Unbounded, Excluded(_)) => todo!(),
            (Excluded(_), Included(_), Included(_), Excluded(_)) => todo!(),
            (Excluded(_), Included(_), Included(_), Unbounded) => todo!(),
            (Excluded(_), Included(_), Excluded(_), Excluded(_)) => todo!(),
            (Excluded(_), Included(_), Excluded(_), Unbounded) => todo!(),
            (Excluded(_), Included(_), Unbounded, Excluded(_)) => todo!(),
            (Excluded(_), Excluded(_), Included(_), Excluded(_)) => todo!(),
            (Excluded(_), Excluded(_), Included(_), Unbounded) => todo!(),
            (Excluded(_), Excluded(_), Excluded(_), Included(_)) => todo!(),
            (Excluded(_), Excluded(_), Excluded(_), Excluded(_)) => todo!(),
            (Excluded(_), Excluded(_), Excluded(_), Unbounded) => todo!(),
            (Excluded(_), Excluded(_), Unbounded, Included(_)) => todo!(),
            (Excluded(_), Excluded(_), Unbounded, Excluded(_)) => todo!(),
            (Excluded(_), Unbounded, Included(_), Excluded(_)) => todo!(),
            (Excluded(_), Unbounded, Excluded(_), Included(_)) => todo!(),
            (Excluded(_), Unbounded, Excluded(_), Excluded(_)) => todo!(),
            (Excluded(_), Unbounded, Unbounded, Included(_)) => todo!(),
            (Excluded(_), Unbounded, Unbounded, Excluded(_)) => todo!(),
            (Unbounded, Included(_), Included(_), Excluded(_)) => todo!(),
            (Unbounded, Included(_), Included(_), Unbounded) => todo!(),
            (Unbounded, Included(_), Excluded(_), Excluded(_)) => todo!(),
            (Unbounded, Included(_), Excluded(_), Unbounded) => todo!(),
            (Unbounded, Excluded(_), Included(_), Excluded(_)) => todo!(),
            (Unbounded, Excluded(_), Included(_), Unbounded) => todo!(),
            (Unbounded, Excluded(_), Excluded(_), Included(_)) => todo!(),
            (Unbounded, Excluded(_), Excluded(_), Excluded(_)) => todo!(),
            (Unbounded, Excluded(_), Excluded(_), Unbounded) => todo!(),
        }
    }
}

impl<T: ?Sized> RangeBounds<T> for RangeFull {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

impl<T> RangeBounds<T> for RangeFrom<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

impl<T> RangeBounds<T> for RangeTo<T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}

impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}

impl<T> RangeBounds<T> for RangeInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        std::ops::RangeBounds::start_bound(self)
    }
    fn end_bound(&self) -> Bound<&T> {
        std::ops::RangeBounds::end_bound(self)
    }
}

impl<T> RangeBounds<T> for RangeToInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(&self.end)
    }
}

impl<T> RangeBounds<T> for (Bound<T>, Bound<T>) {
    fn start_bound(&self) -> Bound<&T> {
        match *self {
            (Included(ref start), _) => Included(start),
            (Excluded(ref start), _) => Excluded(start),
            (Unbounded, _) => Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&T> {
        match *self {
            (_, Included(ref end)) => Included(end),
            (_, Excluded(ref end)) => Excluded(end),
            (_, Unbounded) => Unbounded,
        }
    }
}

impl<'a, T: ?Sized + 'a> RangeBounds<T> for (Bound<&'a T>, Bound<&'a T>) {
    fn start_bound(&self) -> Bound<&T> {
        self.0
    }

    fn end_bound(&self) -> Bound<&T> {
        self.1
    }
}

impl<T> RangeBounds<T> for RangeFrom<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

impl<T> RangeBounds<T> for RangeTo<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(self.end)
    }
}

impl<T> RangeBounds<T> for Range<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(self.end)
    }
}

impl<T> RangeBounds<T> for RangeInclusive<&T> {
    fn start_bound(&self) -> Bound<&T> {
        std::ops::RangeBounds::start_bound(self)
    }
    fn end_bound(&self) -> Bound<&T> {
        std::ops::RangeBounds::end_bound(self)
    }
}

impl<T> RangeBounds<T> for RangeToInclusive<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(self.end)
    }
}

fn check<A, B, K>(a: A, b: B, expected: bool)
where
    A: RangeBounds<K>,
    B: RangeBounds<K>,
    K: PartialEq<K> + ?Sized + PartialOrd<K>,
{
    assert_eq!(expected, a.overlaps(&b));
    assert_eq!(expected, b.overlaps(&a));
}

fn main() {
    check(
        (Bound::Excluded(0), Bound::Excluded(3)),
        (Bound::Excluded(1), Bound::Excluded(3)),
        true,
    );
    check(
        (Bound::Excluded(0), Bound::Excluded(3)),
        (Bound::Excluded(3), Bound::Excluded(4)),
        false,
    );

    check(
        (Bound::Excluded(0), Bound::Included(3)),
        (Bound::Excluded(3), Bound::Excluded(5)),
        false,
    );
}
