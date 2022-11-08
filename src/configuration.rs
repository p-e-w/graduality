// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use csscolorparser::Color;

enum Adjustment {
    Fit,
    Stretch,
    Zoom,
}

enum Interpolation {
    Linear,
    Spline,
}

struct Position {
    x: f32,
    y: f32,
}

struct Step {
    position: Position,
    color: Color,
    duration: f32,
    easing: Box<dyn Fn(f32) -> f32>,
}

struct Point {
    delay: f32,
    interpolation: Interpolation,
    steps: Vec<Step>,
}

struct Configuration {
    adjustment: Adjustment,
    speed: f32,
    points: Vec<Point>,
}
