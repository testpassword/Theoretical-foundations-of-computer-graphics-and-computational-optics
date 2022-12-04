pub fn create_grid(x_size: usize, y_size: usize) -> Vec<(usize, usize)> {
    (0..x_size)
        .flat_map(|x| (0..y_size).map(move |y| (x as usize, y as usize)))
        .collect::<Vec<(usize, usize)>>()
}
