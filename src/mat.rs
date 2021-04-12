use super::vec::*;

pub struct Matrix<const ROWS: usize, const COLUMS: usize> {
    rows: [Vector<COLUMS>; ROWS],
}