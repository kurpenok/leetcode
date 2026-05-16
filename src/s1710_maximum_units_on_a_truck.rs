pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    let mut box_types = box_types;
    box_types.sort_by(|a, b| b[1].cmp(&a[1]));

    let mut remaining = truck_size;
    let mut total_units = 0;

    for box_type in box_types {
        let num_boxes = box_type[0];
        let units_per_box = box_type[1];

        let take = remaining.min(num_boxes);
        total_units += take * units_per_box;
        remaining -= take;

        if remaining == 0 {
            break;
        }
    }

    total_units
}
