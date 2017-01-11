use structure::*;
use angle::Rad;
use matrix::{Matrix4};
use num::BaseFloat;
use num_traits::{cast};
use point::{Point3};
use normal::{Normal3};
use ray::{Ray3};
use vector::{Vector3};

pub trait Transform<T = Self> {
    type Output;
    fn transform(&self, t: &T) -> Self::Output;
    // fn transform_mut(&mut self, t: &T) {
    //     *self = self.transform(t)
    // }
}

pub trait Transformation : Sized {
    type Scalar: BaseFloat;
    type Angle: Angle<Unitless = Self::Scalar>;
    type Vector: VectorSpace<Scalar = Self::Scalar>;
    type Point: EuclideanSpace<Scalar = Self::Scalar, Diff = Self::Vector>;

    fn identity() -> Self;
    fn translate(vector: Self::Vector) -> Self;
    fn scale(vector: Self::Vector) -> Self;
    fn uniform_scale(scalar: Self::Scalar) -> Self;
    fn rotate_x(angle: Self::Angle) -> Self;
    fn rotate_y(angle: Self::Angle) -> Self;
    fn rotate_z(angle: Self::Angle) -> Self;
    fn rotate(axis: Self::Vector, angle: Self::Angle) -> Self;
    fn look_at(eye: Self::Point, center: Self::Point, up: Self::Vector) -> Self;
    fn perspective(fovy: Self::Angle, near: Self::Scalar, far: Self::Scalar) -> Self;

    fn inverse(&self) -> Self;
    fn compose(&self, rhs: &Self) -> Self;
    fn compose_mut(&mut self, rhs: &Self) {
        *self = self.compose(rhs);
    }
    fn apply<V: Transform<Self>>(&self, v: V) -> V::Output {
        v.transform(self)
    }
}

impl<S> Transformation for Matrix4<S> where S: BaseFloat {
    type Scalar = S;
    type Angle = Rad<S>;
    type Vector = Vector3<S>;
    type Point = Point3<S>;

    fn identity() -> Self {
        Self::one()
    }
    fn translate(vector: Self::Vector) -> Self {
        Self::from_translation(vector)
    }
    fn scale(vector: Self::Vector) -> Self {
        Self::from_nonuniform_scale(vector.x, vector.y, vector.z)
    }
    fn uniform_scale(scalar: Self::Scalar) -> Self {
        Self::from_scale(scalar)
    }
    fn rotate_x(angle: Self::Angle) -> Self {
        Self::from_angle_x(angle)
    }
    fn rotate_y(angle: Self::Angle) -> Self {
        Self::from_angle_y(angle)
    }
    fn rotate_z(angle: Self::Angle) -> Self {
        Self::from_angle_z(angle)
    }
    fn rotate(axis: Self::Vector, angle: Self::Angle) -> Self {
        Self::from_axis_angle(axis, angle)
    }
    fn look_at(eye: Self::Point, center: Self::Point, up: Self::Vector) -> Self {
        Self::look_at(eye, center, up)
    }
    fn perspective(fovy: Self::Angle, near: Self::Scalar, far: Self::Scalar) -> Self {
        let proj = Matrix4::new(S::one(), S::zero(), S::zero(), S::zero(),
                                S::zero(), S::one(), S::zero(), S::zero(),
                                S::zero(), S::zero(), far / (far - near), -far * near / (far - near),
                                S::zero(), S::zero(), S::one(), S::zero());
        let two: S = cast(2).unwrap();
        let inv_tan = (fovy / two).cot();
        Self::from_nonuniform_scale(inv_tan, inv_tan, S::one()) * proj
    }

    fn inverse(&self) -> Self {
        self.invert().unwrap()
    }
    fn compose(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl<S> Transform<Matrix4<S>> for Point3<S> where S: BaseFloat {
    type Output = Point3<S>;

    fn transform(&self, t: &Matrix4<S>) -> Self::Output {
        let mut res = Point3::from_value(S::zero());
        for i in 0..3 {
            res[i] = t[i][0] * self.x
                   + t[i][1] * self.y
                   + t[i][2] * self.z
                   + t[i][3];
        }
        let w = t[3][0] * self.x
              + t[3][1] * self.y
              + t[3][2] * self.z
              + t[3][3];

        if w != S::one() {
            res / w
        } else {
            res
        }
    }
}

impl<S> Transform<Matrix4<S>> for Vector3<S> where S: BaseFloat {
    type Output = Vector3<S>;

    fn transform(&self, t: &Matrix4<S>) -> Self::Output {
        let mut res = Vector3::from_value(S::zero());
        for i in 0..3 {
            res[i] = t[i][0] * self.x
                   + t[i][1] * self.y
                   + t[i][2] * self.z;
        }
        res
    }
}

impl<S> Transform<Matrix4<S>> for Normal3<S> where S: BaseFloat {
    type Output = Normal3<S>;

    fn transform(&self, t: &Matrix4<S>) -> Self::Output {
        let mut res = Normal3::from_value(S::zero());
        let inv = t.invert().unwrap();
        for i in 0..3 {
            res[i] = inv[0][i] * self.x
                   + inv[1][i] * self.y
                   + inv[2][i] * self.z;
        }
        res
    }
}

impl<S> Transform<Matrix4<S>> for Ray3<S> where S: BaseFloat {
    type Output = Ray3<S>;

    fn transform(&self, t: &Matrix4<S>) -> Self::Output {
        let mut res = *self;
        res.o = self.o.transform(t);
        res.d = self.d.transform(t);
        res
    }
}
