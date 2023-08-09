pub fn convert_range(value: f32, old_range: [f32; 2], new_range: [f32; 2]) -> f32 {
    ((value - old_range[0]) * (new_range[1] - new_range[0])) / (old_range[1] - old_range[0])
        + new_range[0]
}
