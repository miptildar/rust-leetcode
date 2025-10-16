
#[test]
fn runner() {
    let mut nums = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    Solution::rotate(&mut nums);
    println!("{:?}", nums);
}

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; matrix.len()]; matrix.len()];

        let n: i32 = matrix.len() as i32;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        loop {
            println!("processing {:?}", (i, j));
            if j >= n {
                j = 0;
                i += 1;
            }

            if i >= n {
                break;
            }

            if (visited[i as usize][j as usize]) {
                j += 1;
                continue;
            }

            let rotate1 = rotate_value(matrix, i, j, None);
            visited[rotate1.0 as usize][rotate1.1 as usize] = true;

            let rotate2 = rotate_value(matrix, rotate1.0, rotate1.1, Some(rotate1.2));
            visited[rotate2.0 as usize][rotate2.1 as usize] = true;

            let rotate3 = rotate_value(matrix, rotate2.0, rotate2.1, Some(rotate2.2));
            visited[rotate2.0 as usize][rotate2.1 as usize] = true;

            let rotate4 = rotate_value(matrix, rotate3.0, rotate3.1, Some(rotate3.2));
            visited[rotate3.0 as usize][rotate3.1 as usize] = true;
            j += 1;
        }
    }

    fn find_rotated_position(i1: i32, j1: i32, n: i32) -> (i32, i32) {
        (j1, n - i1 - 1)
    }

    fn rotate_value(matrix: &mut Vec<Vec<i32>>, i1: i32, j1: i32, removed: Option<i32>) -> (i32, i32, i32) {
        let new_position = find_rotated_position(i1, j1, matrix.len() as i32);
        let temp: i32 = matrix[new_position.0 as usize][new_position.1 as usize];
        if (removed.is_some()) {
            matrix[new_position.0 as usize][new_position.1 as usize] = removed.unwrap();
        } else {
            matrix[new_position.0 as usize][new_position.1 as usize] = matrix[i1 as usize][j1 as usize];
        }

        (new_position.0, new_position.1, temp)
    }
}

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; matrix.len()]; matrix.len()];

    let n: i32 = matrix.len() as i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    loop {
        println!("processing {:?}", (i, j));
        if j >= n {
            j = 0;
            i += 1;
        }

        if i >= n {
            break;
        }

        if (visited[i as usize][j as usize]) {
            j += 1;
            continue;
        }

        let rotate1 = rotate_value(matrix, i, j, None);
        visited[rotate1.0 as usize][rotate1.1 as usize] = true;

        let rotate2 = rotate_value(matrix, rotate1.0, rotate1.1, Some(rotate1.2));
        visited[rotate2.0 as usize][rotate2.1 as usize] = true;

        let rotate3 = rotate_value(matrix, rotate2.0, rotate2.1, Some(rotate2.2));
        visited[rotate2.0 as usize][rotate2.1 as usize] = true;

        let rotate4 = rotate_value(matrix, rotate3.0, rotate3.1, Some(rotate3.2));
        visited[rotate3.0 as usize][rotate3.1 as usize] = true;
        j += 1;
    }
}

fn find_rotated_position(i1: i32, j1: i32, n: i32) -> (i32, i32) {
    (j1, n - i1 - 1)
}

fn rotate_value(matrix: &mut Vec<Vec<i32>>, i1: i32, j1: i32, removed: Option<i32>) -> (i32, i32, i32) {
    let new_position = find_rotated_position(i1, j1, matrix.len() as i32);
    let temp: i32 = matrix[new_position.0 as usize][new_position.1 as usize];
    if (removed.is_some()) {
        matrix[new_position.0 as usize][new_position.1 as usize] = removed.unwrap();
    } else {
        matrix[new_position.0 as usize][new_position.1 as usize] = matrix[i1 as usize][j1 as usize];
    }

    (new_position.0, new_position.1, temp)
}

