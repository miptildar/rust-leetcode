#[test]
fn runner() {
    let mut nums = vec![1, 1, 2];
    let i = remove_duplicates(&mut nums);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut left_index: usize = 0;
    let mut current_index: usize = 1;

    let mut left_value: i32 = nums[0];
    let mut current_value: i32;

    let mut result = 0;

    loop {
        if (current_index >= nums.len()) {
            break;
        }

        current_value = nums[current_index];
        if (left_value == current_value) {
            current_index += 1;
            continue;
        }

        result += 1;
        nums[result] = current_value;
        left_value = current_value;
        current_index += 1;
    }

    nums.truncate(result + 1);

    (result + 1) as i32
}
