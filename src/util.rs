pub fn convert_range(value: f32, oldRange: [f32; 2], newRange: [f32; 2]) -> f32 {
    ((value - oldRange[0]) * (newRange[1] - newRange[0])) / (oldRange[1] - oldRange[0])
        + newRange[0]
}
