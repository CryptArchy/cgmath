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

#[macro_use]
extern crate approx;
#[macro_use]
extern crate cgmath;

use cgmath::*;
use std::f32;

const POINT2: Point2<f32> = Point2 { x: 0.0, y: 0.0 };
const VECTOR2: Vector2<f32> = Vector2 { x: 1.0, y: 1.0 };
const RAY2: Ray2<f32> = Ray2 { o: POINT2, d: VECTOR2, min_t: 0.0, max_t: f32::INFINITY, depth: 0, time: 0.0 };

#[test]
fn test_new() {
    let r = Ray2::new(POINT2, VECTOR2, 0.0);
    assert_eq!(RAY2.o, r.o);
    assert_eq!(RAY2.d, r.d);
    assert_eq!(RAY2.depth, r.depth);
    assert_eq!(RAY2.min_t, r.min_t);
    assert_eq!(RAY2.time, r.time);
}

#[test]
fn test_segment() {
    let r = Ray2::segment(POINT2, VECTOR2, 1.0, 1000.0, 0.0);
    assert_eq!(RAY2.o, r.o);
    assert_eq!(RAY2.d, r.d);
    assert_eq!(RAY2.depth, r.depth);
    assert_ne!(RAY2.min_t, r.min_t);
    assert_ne!(RAY2.max_t, r.max_t);
    assert_eq!(RAY2.time, r.time);
}

#[test]
fn test_child() {
    let r = RAY2.child(POINT2, VECTOR2);
    assert_eq!(RAY2.o, r.o);
    assert_eq!(RAY2.d, r.d);
    assert_ne!(RAY2.depth, r.depth);
    assert_eq!(r.depth, 1);
    assert_eq!(RAY2.min_t, r.min_t);
    assert_eq!(RAY2.time, r.time);
}

#[test]
fn test_child_segment() {
    let r = RAY2.child_segment(POINT2, VECTOR2, 1.0, 1000.0);
    assert_eq!(RAY2.o, r.o);
    assert_eq!(RAY2.d, r.d);
    assert_ne!(RAY2.depth, r.depth);
    assert_eq!(r.depth, 1);
    assert_ne!(RAY2.min_t, r.min_t);
    assert_ne!(RAY2.max_t, r.max_t);
    assert_eq!(RAY2.time, r.time);
}

#[test]
fn test_at() {
    let p = RAY2.at(1.0);
    assert_eq!(Point2 {x: 1.0, y: 1.0 }, p);
}
