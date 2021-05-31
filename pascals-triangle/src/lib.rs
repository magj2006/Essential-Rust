pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = vec![];

        for n in 1..=row_count {
            match n {
                1 => rows.push(vec![1]),
                2 => rows.push(vec![1, 1]),
                n if n > 2 => {
                    let m = (n - 1) as usize;
                    let v = &rows[m - 1]; //prev line

                    let mut row = vec![1]; //current line
                                           /*
                                           for i in 0..m - 1 {
                                               row.push(v[i] + v[i + 1])
                                           }
                                           */
                    v.windows(2)
                        .map::<u32, _>(|x| x.iter().sum())
                        .for_each(|x| row.push(x));
                    row.push(1);

                    rows.push(row);
                }
                _ => (),
            }
        }

        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
