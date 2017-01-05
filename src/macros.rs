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

//! Utility macros for code generation

#![macro_use]

/// Generates a binary operator implementation for the permutations of by-ref and by-val
macro_rules! impl_operator {
    // When it is an unary operator
    (<$S:ident: $Constraint:ident> $Op:ident for $Lhs:ty {
        fn $op:ident($x:ident) -> $Output:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self) -> $Output {
                let $x = self; $body
            }
        }

        impl<'a, $S: $Constraint> $Op for &'a $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self) -> $Output {
                let $x = self; $body
            }
        }
    };
    // When the right operand is a scalar
    (<$S:ident: $Constraint:ident> $Op:ident<$Rhs:ident> for $Lhs:ty {
        fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }

        impl<'a, $S: $Constraint> $Op<$Rhs> for &'a $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }
    };
    // When the right operand is a compound type
    (<$S:ident: $Constraint:ident> $Op:ident<$Rhs:ty> for $Lhs:ty {
        fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }

        impl<'a, $S: $Constraint> $Op<&'a $Rhs> for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: &'a $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }

        impl<'a, $S: $Constraint> $Op<$Rhs> for &'a $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }

        impl<'a, 'b, $S: $Constraint> $Op<&'a $Rhs> for &'b $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: &'a $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }
    };
    // When the left operand is a scalar
    ($Op:ident<$Rhs:ident<$S:ident>> for $Lhs:ty {
        fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr }
    }) => {
        impl $Op<$Rhs<$S>> for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: $Rhs<$S>) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }

        impl<'a> $Op<&'a $Rhs<$S>> for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: &'a $Rhs<$S>) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }
    };
}

macro_rules! impl_assignment_operator {
    (<$S:ident: $Constraint:ident> $Op:ident<$Rhs:ty> for $Lhs:ty {
        fn $op:ident(&mut $lhs:ident, $rhs:ident) $body:block
    }) => {
        impl<$S: $Constraint + $Op<$S>> $Op<$Rhs> for $Lhs {
            #[inline]
            fn $op(&mut $lhs, $rhs: $Rhs) $body
        }
    };
}

macro_rules! fold_array {
    (&$method:ident, { $x:expr })                            => { *$x };
    (&$method:ident, { $x:expr, $y:expr })                   => { $x.$method(&$y) };
    (&$method:ident, { $x:expr, $y:expr, $z:expr })          => { $x.$method(&$y).$method(&$z) };
    (&$method:ident, { $x:expr, $y:expr, $z:expr, $w:expr }) => { $x.$method(&$y).$method(&$z).$method(&$w) };
    ($method:ident, { $x:expr })                             => { $x };
    ($method:ident, { $x:expr, $y:expr })                    => { $x.$method($y) };
    ($method:ident, { $x:expr, $y:expr, $z:expr })           => { $x.$method($y).$method($z) };
    ($method:ident, { $x:expr, $y:expr, $z:expr, $w:expr })  => { $x.$method($y).$method($z).$method($w) };
}

/// Generate array conversion implementations for a compound array type
macro_rules! impl_fixed_array_conversions {
    ($ArrayN:ident <$S:ident> { $($field:ident : $index:expr),+ }, $n:expr) => {
        impl<$S> Into<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn into(self) -> [$S; $n] {
                match self { $ArrayN { $($field),+ } => [$($field),+] }
            }
        }

        impl<$S> AsRef<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn as_ref(&self) -> &[$S; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$S; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S: Clone> From<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn from(v: [$S; $n]) -> $ArrayN<$S> {
                // We need to use a clone here because we can't pattern match on arrays yet
                $ArrayN { $($field: v[$index].clone()),+ }
            }
        }

        impl<'a, $S> From<&'a [$S; $n]> for &'a $ArrayN<$S> {
            #[inline]
            fn from(v: &'a [$S; $n]) -> &'a $ArrayN<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut [$S; $n]> for &'a mut $ArrayN<$S> {
            #[inline]
            fn from(v: &'a mut [$S; $n]) -> &'a mut $ArrayN<$S> {
                unsafe { mem::transmute(v) }
            }
        }
    }
}

/// Generate homogeneous tuple conversion implementations for a compound array type
macro_rules! impl_tuple_conversions {
    ($ArrayN:ident <$S:ident> { $($field:ident),+ }, $Tuple:ty) => {
        impl<$S> Into<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn into(self) -> $Tuple {
                match self { $ArrayN { $($field),+ } => ($($field),+,) }
            }
        }

        impl<$S> AsRef<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn as_ref(&self) -> &$Tuple {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut $Tuple {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> From<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn from(v: $Tuple) -> $ArrayN<$S> {
                match v { ($($field),+,) => $ArrayN { $($field: $field),+ } }
            }
        }

        impl<'a, $S> From<&'a $Tuple> for &'a $ArrayN<$S> {
            #[inline]
            fn from(v: &'a $Tuple) -> &'a $ArrayN<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut $Tuple> for &'a mut $ArrayN<$S> {
            #[inline]
            fn from(v: &'a mut $Tuple) -> &'a mut $ArrayN<$S> {
                unsafe { mem::transmute(v) }
            }
        }
    }
}

/// Generates index operators for a compound type
macro_rules! impl_index_operators {
    ($VectorN:ident<$S:ident>, $n:expr, $Output:ty, $I:ty) => {
        impl<$S> Index<$I> for $VectorN<$S> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&'a self, i: $I) -> &'a $Output {
                let v: &[$S; $n] = self.as_ref(); &v[i]
            }
        }

        impl<$S> IndexMut<$I> for $VectorN<$S> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: $I) -> &'a mut $Output {
                let v: &mut [$S; $n] = self.as_mut(); &mut v[i]
            }
        }
    }
}

macro_rules! impl_vector_scalar_ops {
    ($VectorN:ident<$S:ident> { $($field:ident),+ }) => {
        impl_operator!(Mul<$VectorN<$S>> for $S {
            fn mul(scalar, vector) -> $VectorN<$S> { $VectorN::new($(scalar * vector.$field),+) }
        });
        impl_operator!(Div<$VectorN<$S>> for $S {
            fn div(scalar, vector) -> $VectorN<$S> { $VectorN::new($(scalar / vector.$field),+) }
        });
        impl_operator!(Rem<$VectorN<$S>> for $S {
            fn rem(scalar, vector) -> $VectorN<$S> { $VectorN::new($(scalar % vector.$field),+) }
        });
    };
}

// Utility macro for generating associated functions for the vectors
macro_rules! impl_vector {
    ($VectorN:ident { $($field:ident),+ }, $n:expr, $constructor:ident) => {
        impl<S> $VectorN<S> {
            /// Construct a new vector, using the provided values.
            #[inline]
            pub fn new($($field: S),+) -> $VectorN<S> {
                $VectorN { $($field: $field),+ }
            }
        }

        /// The short constructor.
        #[inline]
        pub fn $constructor<S>($($field: S),+) -> $VectorN<S> {
            $VectorN::new($($field),+)
        }

        impl<S: NumCast + Copy> $VectorN<S> {
            /// Component-wise casting to another type
            #[inline]
            pub fn cast<T: NumCast>(&self) -> $VectorN<T> {
                $VectorN { $($field: NumCast::from(self.$field).unwrap()),+ }
            }
        }

        impl<S: BaseFloat> MetricSpace for $VectorN<S> {
            type Metric = S;

            #[inline]
            fn distance2(self, other: Self) -> S {
                (other - self).magnitude2()
            }
        }

        impl<S: Copy> Array for $VectorN<S> {
            type Element = S;

            #[inline]
            fn from_value(scalar: S) -> $VectorN<S> {
                $VectorN { $($field: scalar),+ }
            }

            #[inline]
            fn sum(self) -> S where S: Add<Output = S> {
                fold_array!(add, { $(self.$field),+ })
            }

            #[inline]
            fn product(self) -> S where S: Mul<Output = S> {
                fold_array!(mul, { $(self.$field),+ })
            }

            #[inline]
            fn min(self) -> S where S: PartialOrd {
                fold_array!(partial_min, { $(self.$field),+ })
            }

            #[inline]
            fn max(self) -> S where S: PartialOrd {
                fold_array!(partial_max, { $(self.$field),+ })
            }

            #[inline]
            fn pluck_min(self, other: Self) -> Self where S: PartialOrd {
                $VectorN { $($field: self.$field.partial_min(other.$field)),+ }
            }

            #[inline]
            fn pluck_max(self, other: Self) -> Self where S: PartialOrd {
                $VectorN { $($field: self.$field.partial_max(other.$field)),+ }
            }
        }

        impl<S: BaseNum> Zero for $VectorN<S> {
            #[inline]
            fn zero() -> $VectorN<S> {
                $VectorN::from_value(S::zero())
            }

            #[inline]
            fn is_zero(&self) -> bool {
                *self == $VectorN::zero()
            }
        }

        impl<S: BaseNum> VectorSpace for $VectorN<S> {
            type Scalar = S;
        }

        impl<S: Neg<Output = S> + BaseNum> Neg for $VectorN<S> {
            type Output = $VectorN<S>;
            #[inline]
            fn neg(self) -> $VectorN<S> { $VectorN::new($(-self.$field),+) }
        }

        impl<'a, S: Neg<Output = S> + BaseNum> Neg for &'a $VectorN<S> {
            type Output = $VectorN<S>;
            #[inline]
            fn neg(self) -> $VectorN<S> { let x = self; $VectorN::new($(-x.$field),+) }
        }

        impl<S: BaseFloat> ApproxEq for $VectorN<S> {
            type Epsilon = S::Epsilon;

            #[inline]
            fn default_epsilon() -> S::Epsilon {
                S::default_epsilon()
            }

            #[inline]
            fn default_max_relative() -> S::Epsilon {
                S::default_max_relative()
            }

            #[inline]
            fn default_max_ulps() -> u32 {
                S::default_max_ulps()
            }

            #[inline]
            fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
                $(S::relative_eq(&self.$field, &other.$field, epsilon, max_relative))&&+
            }

            #[inline]
            fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
                $(S::ulps_eq(&self.$field, &other.$field, epsilon, max_ulps))&&+
            }
        }

        impl<S: BaseFloat + Rand> Rand for $VectorN<S> {
            #[inline]
            fn rand<R: Rng>(rng: &mut R) -> $VectorN<S> {
                $VectorN { $($field: rng.gen()),+ }
            }
        }

        // Doesn't work because the complex condition breaks the macro parsing!
        // impl_operator!(<S: Neg<Output = S>> Neg for $VectorN<S> {
        //     fn neg(lhs) -> $VectorN<S> { $VectorN::new($(-lhs.$field),+) }
        // });

        impl_operator!(<S: BaseNum> Add<$VectorN<S> > for $VectorN<S> {
            fn add(lhs, rhs) -> $VectorN<S> { $VectorN::new($(lhs.$field + rhs.$field),+) }
        });
        impl_assignment_operator!(<S: BaseNum> AddAssign<$VectorN<S> > for $VectorN<S> {
            fn add_assign(&mut self, other) { $(self.$field += other.$field);+ }
        });

        impl_operator!(<S: BaseNum> Sub<$VectorN<S> > for $VectorN<S> {
            fn sub(lhs, rhs) -> $VectorN<S> { $VectorN::new($(lhs.$field - rhs.$field),+) }
        });
        impl_assignment_operator!(<S: BaseNum> SubAssign<$VectorN<S> > for $VectorN<S> {
            fn sub_assign(&mut self, other) { $(self.$field -= other.$field);+ }
        });

        impl_operator!(<S: BaseNum> Mul<S> for $VectorN<S> {
            fn mul(vector, scalar) -> $VectorN<S> { $VectorN::new($(vector.$field * scalar),+) }
        });
        impl_assignment_operator!(<S: BaseNum> MulAssign<S> for $VectorN<S> {
            fn mul_assign(&mut self, scalar) { $(self.$field *= scalar);+ }
        });

        impl_operator!(<S: BaseNum> Div<S> for $VectorN<S> {
            fn div(vector, scalar) -> $VectorN<S> { $VectorN::new($(vector.$field / scalar),+) }
        });
        impl_assignment_operator!(<S: BaseNum> DivAssign<S> for $VectorN<S> {
            fn div_assign(&mut self, scalar) { $(self.$field /= scalar);+ }
        });

        impl_operator!(<S: BaseNum> Rem<S> for $VectorN<S> {
            fn rem(vector, scalar) -> $VectorN<S> { $VectorN::new($(vector.$field % scalar),+) }
        });
        impl_assignment_operator!(<S: BaseNum> RemAssign<S> for $VectorN<S> {
            fn rem_assign(&mut self, scalar) { $(self.$field %= scalar);+ }
        });

        impl<S: BaseNum> ElementWise for $VectorN<S> {
            #[inline] fn add_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field + rhs.$field),+) }
            #[inline] fn sub_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field - rhs.$field),+) }
            #[inline] fn mul_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field * rhs.$field),+) }
            #[inline] fn div_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field / rhs.$field),+) }
            #[inline] fn rem_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field % rhs.$field),+) }

            #[inline] fn add_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field += rhs.$field);+ }
            #[inline] fn sub_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field -= rhs.$field);+ }
            #[inline] fn mul_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field *= rhs.$field);+ }
            #[inline] fn div_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field /= rhs.$field);+ }
            #[inline] fn rem_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field %= rhs.$field);+ }
        }

        impl<S: BaseNum> ElementWise<S> for $VectorN<S> {
            #[inline] fn add_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field + rhs),+) }
            #[inline] fn sub_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field - rhs),+) }
            #[inline] fn mul_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field * rhs),+) }
            #[inline] fn div_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field / rhs),+) }
            #[inline] fn rem_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field % rhs),+) }

            #[inline] fn add_assign_element_wise(&mut self, rhs: S) { $(self.$field += rhs);+ }
            #[inline] fn sub_assign_element_wise(&mut self, rhs: S) { $(self.$field -= rhs);+ }
            #[inline] fn mul_assign_element_wise(&mut self, rhs: S) { $(self.$field *= rhs);+ }
            #[inline] fn div_assign_element_wise(&mut self, rhs: S) { $(self.$field /= rhs);+ }
            #[inline] fn rem_assign_element_wise(&mut self, rhs: S) { $(self.$field %= rhs);+ }
        }

        impl_vector_scalar_ops!($VectorN<usize> { $($field),+ });
        impl_vector_scalar_ops!($VectorN<u8> { $($field),+ });
        impl_vector_scalar_ops!($VectorN<u16> { $($field),+ });
        impl_vector_scalar_ops!($VectorN<u32> { $($field),+ });
        impl_vector_scalar_ops!($VectorN<u64> { $($field),+ });
        impl_vector_scalar_ops!($VectorN<isize> { $($field),+ });
        impl_vector_scalar_ops!($VectorN<i8> { $($field),+ });
        impl_vector_scalar_ops!($VectorN<i16> { $($field),+ });
        impl_vector_scalar_ops!($VectorN<i32> { $($field),+ });
        impl_vector_scalar_ops!($VectorN<i64> { $($field),+ });
        impl_vector_scalar_ops!($VectorN<f32> { $($field),+ });
        impl_vector_scalar_ops!($VectorN<f64> { $($field),+ });

        impl_index_operators!($VectorN<S>, $n, S, usize);
        impl_index_operators!($VectorN<S>, $n, [S], Range<usize>);
        impl_index_operators!($VectorN<S>, $n, [S], RangeTo<usize>);
        impl_index_operators!($VectorN<S>, $n, [S], RangeFrom<usize>);
        impl_index_operators!($VectorN<S>, $n, [S], RangeFull);
        // #![feature(inclusive_range,inclusive_range_syntax)]
        // impl_index_operators!($VectorN<S>, $n, [S], RangeInclusive<usize>);
        // impl_index_operators!($VectorN<S>, $n, [S], RangeToInclusive<usize>);
    }
}

