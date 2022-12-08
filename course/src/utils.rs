use crate::vec3::Vec3;

pub fn create_grid(x_size: usize, y_size: usize) -> Vec<(usize, usize)> {
    (0..x_size)
        .flat_map(|x| (0..y_size).map(move |y| (x as usize, y as usize)))
        .collect::<Vec<(usize, usize)>>()
}

pub fn to0_255_color_format(color: Vec3) -> [u8; 3] {
    let convert = |c: f64| -> u8 { 255.min((c * 255.0).round() as u8) };
    [convert(color.x), convert(color.y), convert(color.z)]
}
