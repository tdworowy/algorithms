const INF: i32 = i32::MAX;

#[derive(Debug)]
struct YoungTableau {
    rows: usize,
    cols: usize,
    data: Vec<Vec<i32>>,
    len: usize,
}

impl YoungTableau {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![vec![INF; cols]; rows],
            len: rows * cols,
        }
    }

    fn insert(&mut self, key: i32) -> Result<(), &'static str> {
        if self.data[self.rows - 1][self.cols - 1] != INF {
            return Err("tableau is full");
        }

        let mut i = self.rows - 1;
        let mut j = self.cols - 1;
        self.data[i][j] = key;

        while i > 0 || j > 0 {
            let mut ni = i;
            let mut nj = j;

            if i > 0 && self.data[i - 1][j] > self.data[ni][nj] {
                ni = i - 1;
                nj = j;
            }

            if j > 0 && self.data[i][j - 1] > self.data[ni][nj] {
                ni = i;
                nj = j - 1;
            }

            if ni == i && nj == j {
                break;
            }

            self.data[i][j] = self.data[ni][nj];
            self.data[ni][nj] = key;

            i = ni;
            j = nj;
        }

        Ok(())
    }

    fn from_array(array: &[i32], rows: usize, cols: usize) -> Result<Self, &'static str> {
        if array.len() > rows * cols {
            return Err("array does not fit in tableau");
        }

        let mut tableau = Self::new(rows, cols);
        for &x in array {
            tableau.insert(x)?;
        }

        Ok(tableau)
    }
    fn extract_min(&mut self) -> Option<i32> {
        if self.data[0][0] == INF {
            return None;
        }
        let min = self.data[0][0];
        self.data[0][0] = INF;

        let mut i = 0;
        let mut j = 0;

        loop {
            let mut ni = i;
            let mut nj = j;
            if i + 1 < self.rows && self.data[i + 1][j] < self.data[ni][nj] {
                ni = i + 1;
                nj = j;
            }
            if j + 1 < self.cols && self.data[i][j + 1] < self.data[ni][nj] {
                ni = i;
                nj = j + 1;
            }

            if ni == i && nj == j {
                break;
            }
            self.data[i][j] = self.data[ni][nj];
            self.data[ni][nj] = INF;

            i = ni;
            j = nj;
        }

        Some(min)
    }
    fn is_empty(&self) -> bool {
        self.data[0][0] == INF
    }
    fn sort(&mut self) -> Vec<i32> {
        let mut result = Vec::with_capacity(self.len);
        while let Some(x) = self.extract_min() {
            result.push(x);
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_array() {
        let array = [9, 16, 3, 2, 4, 8, 5, 16, 12];
        let tableau = YoungTableau::from_array(&array, 3, 3).unwrap();
        assert_eq!(tableau.data[0], [2, 4, 8]);
        assert_eq!(tableau.data[1], [3, 9, 12]);
        assert_eq!(tableau.data[2], [5, 16, 16]);
    }
    #[test]
    fn test_sort() {
        let array = [9, 16, 3, 2, 4, 8, 5, 16, 12];
        let mut tableau = YoungTableau::from_array(&array, 3, 3).unwrap();
        let result = tableau.sort();
        assert_eq!(result, [2, 3, 4, 5, 8, 9, 12, 16, 16]);
    }
}

fn main() {
    let array = [9, 16, 3, 2, 4, 8, 5, 16, 12];
    let mut tableau = YoungTableau::from_array(&array, 3, 3).unwrap();
    for row in tableau.data.clone() {
        println!("{:?}", row);
    }
    println!("{:?}", tableau.sort());
}
