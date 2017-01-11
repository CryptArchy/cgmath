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

/// A 1-dimensional vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature = "eders", derive(Serialize, Deserialize))]
pub struct Vector1<S> {
    /// The x component of the vector.
    pub x: S,
}

/// A 2-dimensional vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature = "eders", derive(Serialize, Deserialize))]
pub struct Vector2<S> {
    /// The x component of the vector.
    pub x: S,
    /// The y component of the vector.
    pub y: S,
}

/// A 3-dimensional vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature = "eders", derive(Serialize, Deserialize))]
pub struct Vector3<S> {
    /// The x component of the vector.
    pub x: S,
    /// The y component of the vector.
    pub y: S,
    /// The z component of the vector.
    pub z: S,
}

/// A 4-dimensional vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature = "eders", derive(Serialize, Deserialize))]
pub struct Vector4<S> {
    /// The x component of the vector.
    pub x: S,
    /// The y component of the vector.
    pub y: S,
    /// The z component of the vector.
    pub z: S,
    /// The w component of the vector.
    pub w: S,
}

impl_vector!(Vector1 { x }, 1, vec1);
impl_vector!(Vector2 { x, y }, 2, vec2);
impl_vector!(Vector3 { x, y, z }, 3, vec3);
impl_vector!(Vector4 { x, y, z, w }, 4, vec4);

impl_fixed_array_conversions!(Vector1<S> { x: 0 }, 1);
impl_fixed_array_conversions!(Vector2<S> { x: 0, y: 1 }, 2);
impl_fixed_array_conversions!(Vector3<S> { x: 0, y: 1, z: 2 }, 3);
impl_fixed_array_conversions!(Vector4<S> { x: 0, y: 1, z: 2, w: 3 }, 4);

impl_tuple_conversions!(Vector1<S> { x }, (S,));
impl_tuple_conversions!(Vector2<S> { x, y }, (S, S));
impl_tuple_conversions!(Vector3<S> { x, y, z }, (S, S, S));
impl_tuple_conversions!(Vector4<S> { x, y, z, w }, (S, S, S, S));

impl<S: BaseNum> Vector1<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Vector1<S> {
        Vector1::new(S::one())
    }
}

impl<S: BaseNum> Vector2<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Vector2<S> {
        Vector2::new(S::one(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Vector2<S> {
        Vector2::new(S::zero(), S::one())
    }

    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(self, other: Vector2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }

    /// Create a `Vector3`, using the `x` and `y` values from this vector, and the
    /// provided `z`.
    #[inline]
    pub fn extend(self, z: S)-> Vector3<S> {
        Vector3::new(self.x, self.y, z)
    }
}

impl<S: BaseNum> Vector3<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Vector3<S> {
        Vector3::new(S::one(), S::zero(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Vector3<S> {
        Vector3::new(S::zero(), S::one(), S::zero())
    }

    /// A unit vector in the `w` direction.
    #[inline]
    pub fn unit_z() -> Vector3<S> {
        Vector3::new(S::zero(), S::zero(), S::one())
    }

    /// Create a `Vector4`, using the `x`, `y` and `z` values from this vector, and the
    /// provided `w`.
    #[inline]
    pub fn extend(self, w: S)-> Vector4<S> {
        Vector4::new(self.x, self.y, self.z, w)
    }

    /// Create a `Vector2`, dropping the `z` value.
    #[inline]
    pub fn truncate(self)-> Vector2<S> {
        Vector2::new(self.x, self.y)
    }
}

impl<S: BaseNum> Vector4<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Vector4<S> {
        Vector4::new(S::one(), S::zero(), S::zero(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Vector4<S> {
        Vector4::new(S::zero(), S::one(), S::zero(), S::zero())
    }

    /// A unit vector in the `z` direction.
    #[inline]
    pub fn unit_z() -> Vector4<S> {
        Vector4::new(S::zero(), S::zero(), S::one(), S::zero())
    }

    /// A unit vector in the `w` direction.
    #[inline]
    pub fn unit_w() -> Vector4<S> {
        Vector4::new(S::zero(), S::zero(), S::zero(), S::one())
    }

    /// Create a `Vector3`, dropping the `w` value.
    #[inline]
    pub fn truncate(self)-> Vector3<S> {
        Vector3::new(self.x, self.y, self.z)
    }

    /// Create a `Vector3`, dropping the nth element
    #[inline]
    pub fn truncate_n(&self, n: isize)-> Vector3<S> {
        match n {
            0 => Vector3::new(self.y, self.z, self.w),
            1 => Vector3::new(self.x, self.z, self.w),
            2 => Vector3::new(self.x, self.y, self.w),
            3 => Vector3::new(self.x, self.y, self.z),
            _ => panic!("{:?} is out of range", n)
        }
    }
}

impl<S:BaseFloat> DotProduct for Vector1<S> {
    type Output = S;
    #[inline]
    fn dot(self, other: Self) -> Self::Output {
        Vector1::mul_element_wise(self, other).sum()
    }
}
impl<S: BaseFloat> InnerSpace for Vector1<S> {}

impl<S:BaseFloat> DotProduct for Vector2<S> {
    type Output = S;
    #[inline]
    fn dot(self, other: Self) -> Self::Output {
        Vector2::mul_element_wise(self, other).sum()
    }
}
impl<S: BaseFloat> InnerSpace for Vector2<S> {
    #[inline]
    fn angle(self, other: Vector2<S>) -> Rad<S> {
        Rad::atan2(Self::perp_dot(self, other), Self::dot(self, other))
    }
}

impl<S: BaseFloat> DotProduct for Vector3<S> {
    type Output = S;
    #[inline]
    fn dot(self, other: Self) -> Self::Output {
        Vector3::mul_element_wise(self, other).sum()
    }
}
impl<S: BaseNum> CrossProduct for Vector3<S> {
    type Output = Vector3<S>;
    #[inline]
    #[must_use]
    fn cross(self, other: Self) -> Self {
        Vector3::new((self.y * other.z) - (self.z * other.y),
                     (self.z * other.x) - (self.x * other.z),
                     (self.x * other.y) - (self.y * other.x))
    }
}
impl<S: BaseFloat> FaceForward for Vector3<S> { }
impl<S: BaseFloat> InnerSpace for Vector3<S> {
    #[inline]
    fn angle(self, other: Vector3<S>) -> Rad<S> {
        Rad::atan2(self.cross(other).magnitude(), Self::dot(self, other))
    }
}

impl<S: BaseFloat> DotProduct for Vector4<S> {
    type Output = S;
    #[inline]
    fn dot(self, other: Self) -> Self::Output {
        Vector4::mul_element_wise(self, other).sum()
    }
}
impl<S: BaseFloat> InnerSpace for Vector4<S> { }

impl<S: fmt::Debug> fmt::Debug for Vector1<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Vector1 "));
        <[S; 1] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Vector2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Vector2 "));
        <[S; 2] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Vector3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Vector3 "));
        <[S; 3] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Vector4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Vector4 "));
        <[S; 4] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

#[cfg(test)]
mod tests {
    mod vector2 {
        use vector::*;

        const VECTOR2: Vector2<i32> = Vector2 { x: 1, y: 2 };

        #[test]
        fn test_index() {
            assert_eq!(VECTOR2[0], VECTOR2.x);
            assert_eq!(VECTOR2[1], VECTOR2.y);
        }

        #[test]
        fn test_index_mut() {
            let mut v = VECTOR2;
            *&mut v[0] = 0;
            assert_eq!(v, [0, 2].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            VECTOR2[2];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&VECTOR2[..0], &[]);
            assert_eq!(&VECTOR2[..1], &[1]);
            assert_eq!(VECTOR2[..0].len(), 0);
            assert_eq!(VECTOR2[..1].len(), 1);
            assert_eq!(&VECTOR2[2..], &[]);
            assert_eq!(&VECTOR2[1..], &[2]);
            assert_eq!(VECTOR2[2..].len(), 0);
            assert_eq!(VECTOR2[1..].len(), 1);
            assert_eq!(&VECTOR2[..], &[1, 2]);
            assert_eq!(VECTOR2[..].len(), 2);
        }

        #[test]
        fn test_into() {
            let v = VECTOR2;
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
            let v = VECTOR2;
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
            let mut v = VECTOR2;
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
            assert_eq!(Vector2::from([1, 2]), VECTOR2);
            {
                let v = &[1, 2];
                let v: &Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
            {
                let v = &mut [1, 2];
                let v: &mut Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
            assert_eq!(Vector2::from((1, 2)), VECTOR2);
            {
                let v = &(1, 2);
                let v: &Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
            {
                let v = &mut (1, 2);
                let v: &mut Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
        }
    }

    mod vector3 {
        use vector::*;

        const VECTOR3: Vector3<i32> = Vector3 { x: 1, y: 2, z: 3 };

        #[test]
        fn test_index() {
            assert_eq!(VECTOR3[0], VECTOR3.x);
            assert_eq!(VECTOR3[1], VECTOR3.y);
            assert_eq!(VECTOR3[2], VECTOR3.z);
        }

        #[test]
        fn test_index_mut() {
            let mut v = VECTOR3;
            *&mut v[1] = 0;
            assert_eq!(v, [1, 0, 3].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            VECTOR3[3];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&VECTOR3[..1], &[1]);
            assert_eq!(&VECTOR3[..2], &[1, 2]);
            assert_eq!(VECTOR3[..1].len(), 1);
            assert_eq!(VECTOR3[..2].len(), 2);
            assert_eq!(&VECTOR3[2..], &[3]);
            assert_eq!(&VECTOR3[1..], &[2, 3]);
            assert_eq!(VECTOR3[2..].len(), 1);
            assert_eq!(VECTOR3[1..].len(), 2);
            assert_eq!(&VECTOR3[..], &[1, 2, 3]);
            assert_eq!(VECTOR3[..].len(), 3);
        }

        #[test]
        fn test_into() {
            let v = VECTOR3;
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
            let v = VECTOR3;
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
            let mut v = VECTOR3;
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
            assert_eq!(Vector3::from([1, 2, 3]), VECTOR3);
            {
                let v = &[1, 2, 3];
                let v: &Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
            {
                let v = &mut [1, 2, 3];
                let v: &mut Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
            assert_eq!(Vector3::from((1, 2, 3)), VECTOR3);
            {
                let v = &(1, 2, 3);
                let v: &Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
            {
                let v = &mut (1, 2, 3);
                let v: &mut Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
        }
    }

    mod vector4 {
        use vector::*;

        const VECTOR4: Vector4<i32> = Vector4 { x: 1, y: 2, z: 3, w: 4 };

        #[test]
        fn test_index() {
            assert_eq!(VECTOR4[0], VECTOR4.x);
            assert_eq!(VECTOR4[1], VECTOR4.y);
            assert_eq!(VECTOR4[2], VECTOR4.z);
            assert_eq!(VECTOR4[3], VECTOR4.w);
        }

        #[test]
        fn test_index_mut() {
            let mut v = VECTOR4;
            *&mut v[2] = 0;
            assert_eq!(v, [1, 2, 0, 4].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            VECTOR4[4];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&VECTOR4[..2], &[1, 2]);
            assert_eq!(&VECTOR4[..3], &[1, 2, 3]);
            assert_eq!(VECTOR4[..2].len(), 2);
            assert_eq!(VECTOR4[..3].len(), 3);
            assert_eq!(&VECTOR4[2..], &[3, 4]);
            assert_eq!(&VECTOR4[1..], &[2, 3, 4]);
            assert_eq!(VECTOR4[2..].len(), 2);
            assert_eq!(VECTOR4[1..].len(), 3);
            assert_eq!(&VECTOR4[..], &[1, 2, 3, 4]);
            assert_eq!(VECTOR4[..].len(), 4);
        }

        #[test]
        fn test_into() {
            let v = VECTOR4;
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
            let v = VECTOR4;
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
            let mut v = VECTOR4;
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
            assert_eq!(Vector4::from([1, 2, 3, 4]), VECTOR4);
            {
                let v = &[1, 2, 3, 4];
                let v: &Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
            {
                let v = &mut [1, 2, 3, 4];
                let v: &mut Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
            assert_eq!(Vector4::from((1, 2, 3, 4)), VECTOR4);
            {
                let v = &(1, 2, 3, 4);
                let v: &Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
            {
                let v = &mut (1, 2, 3, 4);
                let v: &mut Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
        }
    }
}
