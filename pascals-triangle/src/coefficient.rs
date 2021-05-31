pub fn rows(&self) -> Vec<Vec<u32>> {
    let mut triangle_rows: Vec<Vec<u32>> = Vec::new();
    let coefficient = |n: u32, k: u32| {
        (1..=k)
            .map(|i| (n as f32 + 1.0 - i as f32) / i as f32)
            .product::<f32>() as u32
    };

    for n in 0..self.rows {
        let row: Vec<u32> = (0..=n).map(|k| coefficient(n, k)).collect();
        triangle_rows.push(row);
    }
    triangle_rows
}
