pub fn wrap(mut item: f32, min_val: f32, max_val: f32) -> f32 {
    let width = max_val - min_val;

    if item < min_val {
        item += width;

        while item < min_val {
            item += width
        }
    } else if item > max_val {
        item -= width;

        while item > max_val {
            item -= width
        }
    }

    item
}