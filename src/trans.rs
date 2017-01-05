use structure::*;

use approx::ApproxEq;
use matrix::{Matrix2, Matrix3, Matrix4};
use num::{BaseFloat, BaseNum};
use point::{Point2, Point3};
use vector::{Vector2, Vector3};

pub trait Transform<T = Self> {
    type Output;
    fn transform(&self, t: &T) -> Self::Output;
    // fn transform_mut(&mut self, t: &T) {
    //     *self = self.transform(t)
    // }
}

pub trait Transformation : Sized {
    fn inverse(&self) -> Self;
    fn compose(&self, rhs: &Self) -> Self;
    fn compose_mut(&mut self, rhs: &Self) {
        *self = self.compose(rhs);
    }
    fn apply<V: Transform<Self>>(&self, v: V) -> V::Output {
        v.transform(self)
    }
}

impl<S> Transform<Matrix4<S>> for Vector3<S> where S: BaseNum {
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

impl<S> Transformation for Matrix4<S> where S: BaseNum {
    fn inverse(&self) -> Self {
        unimplemented!();
    }
    fn compose(&self, rhs: &Self) -> Self {
        unimplemented!();
    }
}