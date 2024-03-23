use std::ops::{Add, Index};

#[derive(Debug)]
pub struct Matrix<T: Add + Default, const S: usize> {
    fields: [[T; S]; S],
}

impl<T, const S: usize> Index<usize> for Matrix<T, S>
where
    T: Add<Output = T> + Copy + Default,
{
    type Output = [T; S];

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}

impl<T, const S: usize> Matrix<T, S>
where 
    T: Add<Output = T> + Copy + Default + std::fmt::Display,
{
    pub fn new() -> Self {
        Self { fields: [[T::default(); S]; S] }
    }

    pub fn from_arr(arr: [[T; S]; S]) -> Self {
        Self { fields: arr }
    }

    pub fn det(&self) -> T {
        T::default()
    }

    
}

pub fn determinant(sub: Vec<Vec<i32>>) -> i32 {
    assert_eq!(sub.len(), sub[0].len(), "Expected quadratic matrix");

    if sub.len() <= 3 {
        let mut det = 0;
        let len = sub.len();

        for i in 0..len {
            let mut d = [1i32; 2];

            for j in 0..len {
                d[0] *= sub[j][(i + j) % len];
                d[1] *= sub[len - j - 1][(i + j) % len];
            }

            det += d[0] - d[1];
        }

        return det;
    }

    let mut ans = 0;

    for i in 0..sub.len() {
        let mut clone = Vec::new();

        for j in 0..sub.len() {
            if i != j {
                clone.push(sub[j][1..].to_vec());
            }
        }

        ans += (-1i32).pow(i as u32) * sub[i][0] * determinant(clone);
    }

    ans
}
