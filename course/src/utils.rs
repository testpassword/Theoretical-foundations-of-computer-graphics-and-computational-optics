use crate::vec3::Vec3;

pub fn create_grid(x_size: u32, y_size: u32) -> Vec<(u32, u32)> {
    (0..x_size)
        .flat_map(|x| (0..y_size).map(move |y| (x, y)))
        .collect::<Vec<(u32, u32)>>()
}

pub fn to0_255_color_format(color: Vec3) -> [u8; 3] {
    let convert = |c: f64| 255.min((c * 255.0).round() as u8);
    [convert(color.x), convert(color.y), convert(color.z)]
}


/*pub fn transpose_matrix(data: Vec<usize>, n: usize, m: usize) -> Vec<usize> {
    let transposed = vec![];
    for i in 0..m {
        for j in 0..n {
            let index1 = i * n + j;
            let index2 = j * m + i;

        }
    }
    transposed
}
*/
