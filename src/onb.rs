use super::types::*;
use super::vec::*;
pub struct OrthoNormalBasis {
    axis: [Direction; 3],
}

impl OrthoNormalBasis {
    pub fn from_w(w: &Direction) -> Self {
        let a = if w.x().abs() > 0.9 {
            Direction::from_values(&[0.0, 1.0, 0.0])
        } else {
            Direction::from_values(&[1.0, 0.0, 0.0])
        };

        let v = cross(w, &a);
        let u = cross(w, &v);
        Self { axis: [u, v, *w] }
    }

    pub fn u(&self) -> &Direction {
        &self.axis[0]
    }
    pub fn v(&self) -> &Direction {
        &self.axis[1]
    }
    pub fn w(&self) -> &Direction {
        &self.axis[2]
    }

    pub fn local(&self, v: &Direction) -> Direction {
        v.x() * self.u() + v.y() * self.v() + v.z() * self.w()
    }
}
