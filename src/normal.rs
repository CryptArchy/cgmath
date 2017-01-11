// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rand::{Rand, Rng};
use num_traits::NumCast;
use std::fmt;
use std::mem;
use std::ops::*;

use structure::*;

use angle::Rad;
use approx::ApproxEq;
use num::{BaseNum, BaseFloat, PartialOrd};

// Differences with Normal3 and Vector3
// Normal cannot be added to a Point
// Cross Product can be: V*V, V*N, N*V. (NOT N*N) Always returns V
// Dot Product can be: V*V, V*N, N*V, N*N
// AbsDot is same
// Faceforward can be: NfN, NfV, VfN, VfV

// Misc Operations
// Componentized Min, Max, Floor, Ceiling, arithmatic
// Which dimension has the Min or Max value
// New vector from Min/Max components of two vectors
// New vector by permuting components of vector
// Dot, AbsDot, Abs, FaceForward

/// A 1-dimensional surface-normal vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature = "eders", derive(Serialize, Deserialize))]
pub struct Normal1<S> {
    /// The x component of the vector.
    pub x: S,
}

/// A 2-dimensional surface-normal vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature = "eders", derive(Serialize, Deserialize))]
pub struct Normal2<S> {
    /// The x component of the vector.
    pub x: S,
    /// The y component of the vector.
    pub y: S,
}

/// A 3-dimensional surface-normal vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature = "eders", derive(Serialize, Deserialize))]
pub struct Normal3<S> {
    /// The x component of the vector.
    pub x: S,
    /// The y component of the vector.
    pub y: S,
    /// The z component of the vector.
    pub z: S,
}

/// A 4-dimensional surface-normal vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature = "eders", derive(Serialize, Deserialize))]
pub struct Normal4<S> {
    /// The x component of the vector.
    pub x: S,
    /// The y component of the vector.
    pub y: S,
    /// The z component of the vector.
    pub z: S,
    /// The w component of the vector.
    pub w: S,
}

impl_vector!(Normal1 { x }, 1, norm1);
impl_vector!(Normal2 { x, y }, 2, norm2);
impl_vector!(Normal3 { x, y, z }, 3, norm3);
impl_vector!(Normal4 { x, y, z, w }, 4, norm4);

impl_fixed_array_conversions!(Normal1<S> { x: 0 }, 1);
impl_fixed_array_conversions!(Normal2<S> { x: 0, y: 1 }, 2);
impl_fixed_array_conversions!(Normal3<S> { x: 0, y: 1, z: 2 }, 3);
impl_fixed_array_conversions!(Normal4<S> { x: 0, y: 1, z: 2, w: 3 }, 4);

impl_tuple_conversions!(Normal1<S> { x }, (S,));
impl_tuple_conversions!(Normal2<S> { x, y }, (S, S));
impl_tuple_conversions!(Normal3<S> { x, y, z }, (S, S, S));
impl_tuple_conversions!(Normal4<S> { x, y, z, w }, (S, S, S, S));

impl<S: BaseNum> Normal1<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Normal1<S> {
        Normal1::new(S::one())
    }
}

impl<S: BaseNum> Normal2<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Normal2<S> {
        Normal2::new(S::one(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Normal2<S> {
        Normal2::new(S::zero(), S::one())
    }

    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(self, other: Normal2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }

    /// Create a `Normal3`, using the `x` and `y` values from this vector, and the
    /// provided `z`.
    #[inline]
    pub fn extend(self, z: S)-> Normal3<S> {
        Normal3::new(self.x, self.y, z)
    }
}

impl<S: BaseNum> Normal3<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Normal3<S> {
        Normal3::new(S::one(), S::zero(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Normal3<S> {
        Normal3::new(S::zero(), S::one(), S::zero())
    }

    /// A unit vector in the `w` direction.
    #[inline]
    pub fn unit_z() -> Normal3<S> {
        Normal3::new(S::zero(), S::zero(), S::one())
    }

    /// Returns the cross product of the vector and `other`.
    #[inline]
    #[must_use]
    pub fn cross(self, other: Normal3<S>) -> Normal3<S> {
        Normal3::new((self.y * other.z) - (self.z * other.y),
                     (self.z * other.x) - (self.x * other.z),
                     (self.x * other.y) - (self.y * other.x))
    }

    /// Create a `Normal4`, using the `x`, `y` and `z` values from this vector, and the
    /// provided `w`.
    #[inline]
    pub fn extend(self, w: S)-> Normal4<S> {
        Normal4::new(self.x, self.y, self.z, w)
    }

    /// Create a `Normal2`, dropping the `z` value.
    #[inline]
    pub fn truncate(self)-> Normal2<S> {
        Normal2::new(self.x, self.y)
    }
}

impl<S: BaseNum> Normal4<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Normal4<S> {
        Normal4::new(S::one(), S::zero(), S::zero(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Normal4<S> {
        Normal4::new(S::zero(), S::one(), S::zero(), S::zero())
    }

    /// A unit vector in the `z` direction.
    #[inline]
    pub fn unit_z() -> Normal4<S> {
        Normal4::new(S::zero(), S::zero(), S::one(), S::zero())
    }

    /// A unit vector in the `w` direction.
    #[inline]
    pub fn unit_w() -> Normal4<S> {
        Normal4::new(S::zero(), S::zero(), S::zero(), S::one())
    }

    /// Create a `Normal3`, dropping the `w` value.
    #[inline]
    pub fn truncate(self)-> Normal3<S> {
        Normal3::new(self.x, self.y, self.z)
    }

    /// Create a `Normal3`, dropping the nth element
    #[inline]
    pub fn truncate_n(&self, n: isize)-> Normal3<S> {
        match n {
            0 => Normal3::new(self.y, self.z, self.w),
            1 => Normal3::new(self.x, self.z, self.w),
            2 => Normal3::new(self.x, self.y, self.w),
            3 => Normal3::new(self.x, self.y, self.z),
            _ => panic!("{:?} is out of range", n)
        }
    }
}

impl<S:BaseFloat> DotProduct for Normal1<S> {
    type Output = S;
    #[inline]
    fn dot(self, other: Self) -> Self::Output {
        Normal1::mul_element_wise(self, other).sum()
    }
}
impl<S: BaseFloat> InnerSpace for Normal1<S> {}

impl<S:BaseFloat> DotProduct for Normal2<S> {
    type Output = S;
    #[inline]
    fn dot(self, other: Self) -> Self::Output {
        Normal2::mul_element_wise(self, other).sum()
    }
}
impl<S: BaseFloat> InnerSpace for Normal2<S> {
    #[inline]
    fn angle(self, other: Normal2<S>) -> Rad<S> {
        Rad::atan2(Self::perp_dot(self, other), Self::dot(self, other))
    }
}

impl<S: BaseFloat> DotProduct for Normal3<S> {
    type Output = S;
    #[inline]
    fn dot(self, other: Self) -> Self::Output {
        Normal3::mul_element_wise(self, other).sum()
    }
}
impl<S: BaseFloat> FaceForward for Normal3<S> { }
impl<S: BaseFloat> InnerSpace for Normal3<S> {
    #[inline]
    fn angle(self, other: Normal3<S>) -> Rad<S> {
        Rad::atan2(self.cross(other).magnitude(), Self::dot(self, other))
    }
}

impl<S: BaseFloat> DotProduct for Normal4<S> {
    type Output = S;
    #[inline]
    fn dot(self, other: Self) -> Self::Output {
        Normal4::mul_element_wise(self, other).sum()
    }
}
impl<S: BaseFloat> InnerSpace for Normal4<S> { }

impl<S: fmt::Debug> fmt::Debug for Normal1<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Normal1 "));
        <[S; 1] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Normal2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Normal2 "));
        <[S; 2] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Normal3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Normal3 "));
        <[S; 3] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Normal4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Normal4 "));
        <[S; 4] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}


#[cfg(test)]
mod tests {
    mod normal2 {
        use normal::*;

        const NORMAL2: Normal2<i32> = Normal2 { x: 1, y: 2 };

        #[test]
        fn test_index() {
            assert_eq!(NORMAL2[0], NORMAL2.x);
            assert_eq!(NORMAL2[1], NORMAL2.y);
        }

        #[test]
        fn test_index_mut() {
            let mut v = NORMAL2;
            *&mut v[0] = 0;
            assert_eq!(v, [0, 2].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            NORMAL2[2];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&NORMAL2[..0], &[]);
            assert_eq!(&NORMAL2[..1], &[1]);
            assert_eq!(NORMAL2[..0].len(), 0);
            assert_eq!(NORMAL2[..1].len(), 1);
            assert_eq!(&NORMAL2[2..], &[]);
            assert_eq!(&NORMAL2[1..], &[2]);
            assert_eq!(NORMAL2[2..].len(), 0);
            assert_eq!(NORMAL2[1..].len(), 1);
            assert_eq!(&NORMAL2[..], &[1, 2]);
            assert_eq!(NORMAL2[..].len(), 2);
        }

        #[test]
        fn test_into() {
            let v = NORMAL2;
            {
                let v: [i32; 2] = v.into();
                assert_eq!(v, [1, 2]);
            }
            {
                let v: (i32, i32) = v.into();
                assert_eq!(v, (1, 2));
            }
        }

        #[test]
        fn test_as_ref() {
            let v = NORMAL2;
            {
                let v: &[i32; 2] = v.as_ref();
                assert_eq!(v, &[1, 2]);
            }
            {
                let v: &(i32, i32) = v.as_ref();
                assert_eq!(v, &(1, 2));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut v = NORMAL2;
            {
                let v: &mut [i32; 2] = v.as_mut();
                assert_eq!(v, &mut [1, 2]);
            }
            {
                let v: &mut (i32, i32) = v.as_mut();
                assert_eq!(v, &mut (1, 2));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Normal2::from([1, 2]), NORMAL2);
            {
                let v = &[1, 2];
                let v: &Normal2<_> = From::from(v);
                assert_eq!(v, &NORMAL2);
            }
            {
                let v = &mut [1, 2];
                let v: &mut Normal2<_> = From::from(v);
                assert_eq!(v, &NORMAL2);
            }
            assert_eq!(Normal2::from((1, 2)), NORMAL2);
            {
                let v = &(1, 2);
                let v: &Normal2<_> = From::from(v);
                assert_eq!(v, &NORMAL2);
            }
            {
                let v = &mut (1, 2);
                let v: &mut Normal2<_> = From::from(v);
                assert_eq!(v, &NORMAL2);
            }
        }
    }

    mod normal3 {
        use normal::*;

        const NORMAL3: Normal3<i32> = Normal3 { x: 1, y: 2, z: 3 };

        #[test]
        fn test_index() {
            assert_eq!(NORMAL3[0], NORMAL3.x);
            assert_eq!(NORMAL3[1], NORMAL3.y);
            assert_eq!(NORMAL3[2], NORMAL3.z);
        }

        #[test]
        fn test_index_mut() {
            let mut v = NORMAL3;
            *&mut v[1] = 0;
            assert_eq!(v, [1, 0, 3].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            NORMAL3[3];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&NORMAL3[..1], &[1]);
            assert_eq!(&NORMAL3[..2], &[1, 2]);
            assert_eq!(NORMAL3[..1].len(), 1);
            assert_eq!(NORMAL3[..2].len(), 2);
            assert_eq!(&NORMAL3[2..], &[3]);
            assert_eq!(&NORMAL3[1..], &[2, 3]);
            assert_eq!(NORMAL3[2..].len(), 1);
            assert_eq!(NORMAL3[1..].len(), 2);
            assert_eq!(&NORMAL3[..], &[1, 2, 3]);
            assert_eq!(NORMAL3[..].len(), 3);
        }

        #[test]
        fn test_into() {
            let v = NORMAL3;
            {
                let v: [i32; 3] = v.into();
                assert_eq!(v, [1, 2, 3]);
            }
            {
                let v: (i32, i32, i32) = v.into();
                assert_eq!(v, (1, 2, 3));
            }
        }

        #[test]
        fn test_as_ref() {
            let v = NORMAL3;
            {
                let v: &[i32; 3] = v.as_ref();
                assert_eq!(v, &[1, 2, 3]);
            }
            {
                let v: &(i32, i32, i32) = v.as_ref();
                assert_eq!(v, &(1, 2, 3));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut v = NORMAL3;
            {
                let v: &mut [i32; 3] = v.as_mut();
                assert_eq!(v, &mut [1, 2, 3]);
            }
            {
                let v: &mut (i32, i32, i32) = v.as_mut();
                assert_eq!(v, &mut (1, 2, 3));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Normal3::from([1, 2, 3]), NORMAL3);
            {
                let v = &[1, 2, 3];
                let v: &Normal3<_> = From::from(v);
                assert_eq!(v, &NORMAL3);
            }
            {
                let v = &mut [1, 2, 3];
                let v: &mut Normal3<_> = From::from(v);
                assert_eq!(v, &NORMAL3);
            }
            assert_eq!(Normal3::from((1, 2, 3)), NORMAL3);
            {
                let v = &(1, 2, 3);
                let v: &Normal3<_> = From::from(v);
                assert_eq!(v, &NORMAL3);
            }
            {
                let v = &mut (1, 2, 3);
                let v: &mut Normal3<_> = From::from(v);
                assert_eq!(v, &NORMAL3);
            }
        }
    }

    mod normal4 {
        use normal::*;

        const NORMAL4: Normal4<i32> = Normal4 { x: 1, y: 2, z: 3, w: 4 };

        #[test]
        fn test_index() {
            assert_eq!(NORMAL4[0], NORMAL4.x);
            assert_eq!(NORMAL4[1], NORMAL4.y);
            assert_eq!(NORMAL4[2], NORMAL4.z);
            assert_eq!(NORMAL4[3], NORMAL4.w);
        }

        #[test]
        fn test_index_mut() {
            let mut v = NORMAL4;
            *&mut v[2] = 0;
            assert_eq!(v, [1, 2, 0, 4].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            NORMAL4[4];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&NORMAL4[..2], &[1, 2]);
            assert_eq!(&NORMAL4[..3], &[1, 2, 3]);
            assert_eq!(NORMAL4[..2].len(), 2);
            assert_eq!(NORMAL4[..3].len(), 3);
            assert_eq!(&NORMAL4[2..], &[3, 4]);
            assert_eq!(&NORMAL4[1..], &[2, 3, 4]);
            assert_eq!(NORMAL4[2..].len(), 2);
            assert_eq!(NORMAL4[1..].len(), 3);
            assert_eq!(&NORMAL4[..], &[1, 2, 3, 4]);
            assert_eq!(NORMAL4[..].len(), 4);
        }

        #[test]
        fn test_into() {
            let v = NORMAL4;
            {
                let v: [i32; 4] = v.into();
                assert_eq!(v, [1, 2, 3, 4]);
            }
            {
                let v: (i32, i32, i32, i32) = v.into();
                assert_eq!(v, (1, 2, 3, 4));
            }
        }

        #[test]
        fn test_as_ref() {
            let v = NORMAL4;
            {
                let v: &[i32; 4] = v.as_ref();
                assert_eq!(v, &[1, 2, 3, 4]);
            }
            {
                let v: &(i32, i32, i32, i32) = v.as_ref();
                assert_eq!(v, &(1, 2, 3, 4));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut v = NORMAL4;
            {
                let v: &mut[i32; 4] = v.as_mut();
                assert_eq!(v, &mut [1, 2, 3, 4]);
            }
            {
                let v: &mut(i32, i32, i32, i32) = v.as_mut();
                assert_eq!(v, &mut (1, 2, 3, 4));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Normal4::from([1, 2, 3, 4]), NORMAL4);
            {
                let v = &[1, 2, 3, 4];
                let v: &Normal4<_> = From::from(v);
                assert_eq!(v, &NORMAL4);
            }
            {
                let v = &mut [1, 2, 3, 4];
                let v: &mut Normal4<_> = From::from(v);
                assert_eq!(v, &NORMAL4);
            }
            assert_eq!(Normal4::from((1, 2, 3, 4)), NORMAL4);
            {
                let v = &(1, 2, 3, 4);
                let v: &Normal4<_> = From::from(v);
                assert_eq!(v, &NORMAL4);
            }
            {
                let v = &mut (1, 2, 3, 4);
                let v: &mut Normal4<_> = From::from(v);
                assert_eq!(v, &NORMAL4);
            }
        }
    }
}
