use std::{ops::{Index, IndexMut}, slice::SliceIndex};

pub struct VMatrix {
    m: [f32; 16],
}

impl VMatrix {
    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32,
    ) -> Self {
        VMatrix {
            m: [
                m00, m01, m02, m03,
                m10, m11, m12, m13,
                m20, m21, m22, m23,
                m30, m31, m32, m33,
            ],
        }
    }

    pub fn base(&self) -> &f32 {
        &self.m[0]
    }

    pub fn base_mut(&mut self) -> &mut f32 {
        &mut self.m[0]
    }
}

impl Index<usize> for VMatrix {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.m[i]
    }
}

impl IndexMut<usize> for VMatrix {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.m[i]
    }
}