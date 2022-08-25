use std::ops::Deref;

use crate::{types::Color, vec::normalize};

use super::types::{Direction, Position};

pub trait Light {
    fn sample(&self, position: &Position) -> Direction;
    fn color(&self) -> Color;
}

pub struct DirectionalLight {
    position: Position,
    intenstity: f32,
    color: Color,
}

impl DirectionalLight {
    pub fn new(position: Position) -> Self {
        Self {
            position: normalize(&position),
            color: Color::from_values([1., 1., 1.]),
            intenstity: 10.0,
        }
    }
}

impl Light for DirectionalLight {
    fn sample(&self, _: &Position) -> Direction {
        self.position
    }

    fn color(&self) -> Color {
        self.color * self.intenstity
    }
}

pub struct SphericalLight {
    radius: f32,
    intensity: f32,
}

pub struct Lights {
    data: Vec<Box<dyn Light>>,
}

impl Lights {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add<T>(&mut self, light: T)
    where
        T: Light + 'static,
    {
        self.data.push(Box::new(light));
    }

    pub fn data(&self) -> &Vec<Box<dyn Light>> {
        &self.data
    }
}

unsafe impl Send for Lights {}
unsafe impl Sync for Lights {}
