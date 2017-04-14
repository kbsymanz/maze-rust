
#[test]
pub fn get_neighbors_test_corners() {
    // Upper left
    let upper_left = ::utils::get_neighbors(0, 0, 10, 10);
    let upper_left_ok = upper_left.contains(&(0, 1)) && upper_left.contains(&(1, 0));
    assert!(upper_left.len() == 2);
    assert!(upper_left_ok);
    // Upper right
    let upper_right = ::utils::get_neighbors(9, 0, 10, 10);
    let upper_right_ok = upper_right.contains(&(8, 0)) && upper_right.contains(&(9, 1));
    assert!(upper_right.len() == 2);
    assert!(upper_right_ok);
    // Lower right
    let lower_right = ::utils::get_neighbors(9, 9, 10, 10);
    let lower_right_ok = lower_right.contains(&(8, 9)) && lower_right.contains(&(9, 8));
    assert!(lower_right.len() == 2);
    assert!(lower_right_ok);
    // Lower left
    let lower_left = ::utils::get_neighbors(0, 9, 10, 10);
    let lower_left_ok = lower_left.contains(&(0, 8)) && lower_left.contains(&(1, 9));
    assert!(lower_left.len() == 2);
    assert!(lower_left_ok);
}

#[test]
pub fn get_neighbors_sides() {
    // Left
    let left = ::utils::get_neighbors(0, 3, 10, 10);
    let left_ok = left.contains(&(0, 2)) && left.contains(&(1, 3)) && left.contains(&(0, 4));
    assert!(left.len() == 3);
    assert!(left_ok);
    // Right
    let right = ::utils::get_neighbors(9, 3, 10, 10);
    let right_ok = right.contains(&(9, 2)) && right.contains(&(8, 3)) && right.contains(&(9, 4));
    assert!(right.len() == 3);
    assert!(right_ok);
    // Top
    let top = ::utils::get_neighbors(5, 0, 10, 10);
    let top_ok = top.contains(&(4, 0)) && top.contains(&(5, 1)) && top.contains(&(6, 0));
    assert!(top.len() == 3);
    assert!(top_ok);
    // Bottom
    let bottom = ::utils::get_neighbors(5, 9, 10, 10);
    let bottom_ok = bottom.contains(&(4, 9)) && bottom.contains(&(5, 8)) &&
                    bottom.contains(&(6, 9));
    assert!(bottom.len() == 3);
    assert!(bottom_ok);
}

#[test]
pub fn get_neighbors_middle() {
    let middle1 = ::utils::get_neighbors(5, 5, 10, 10);
    let middle2 = ::utils::get_neighbors(1, 5, 10, 10);
    let middle3 = ::utils::get_neighbors(8, 8, 10, 10);
    assert!(middle1.len() == 4);
    assert!(middle2.len() == 4);
    assert!(middle3.len() == 4);

    let middle1_ok = middle1.contains(&(5, 4)) && middle1.contains(&(6, 5)) &&
                     middle1.contains(&(5, 6)) && middle1.contains(&(4, 5));
    assert!(middle1_ok);

    let middle2_ok = middle2.contains(&(1, 4)) && middle2.contains(&(2, 5)) &&
                     middle2.contains(&(1, 6)) && middle2.contains(&(0, 5));
    assert!(middle2_ok);

    let middle3_ok = middle3.contains(&(8, 7)) && middle3.contains(&(9, 8)) &&
                     middle3.contains(&(8, 9)) && middle3.contains(&(7, 8));
    assert!(middle3_ok);
}

#[test]
pub fn get_neighbors_invalid() {
    let too_right = ::utils::get_neighbors(11, 9, 10, 10);
    assert!(too_right.len() == 0);

    let too_bottom = ::utils::get_neighbors(8, 11, 10, 10);
    assert!(too_bottom.len() == 0);
}
