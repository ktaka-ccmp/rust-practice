fn find_error_nums(n: Vec<i32>) -> Vec<i32> {

    let mut frequency = vec![0; n.len() + 1];
    let mut duplicate = 0;
    let mut missing = 0;

    for i in n.iter() {
        frequency[*i as usize] += 1;
    }

    for i in 1..frequency.len() {
        if frequency[i] == 0 {
            missing = i as i32;
        }
        if frequency[i] == 2 {
            duplicate = i as i32;
        }
    }

    return vec![duplicate, missing];
}

fn main() {
    let test_cases = vec![
        vec![1, 2, 2, 4, 5],
        vec![1, 2, 4, 4, 5],
        vec![1, 1],
        vec![3, 2, 2],
        vec![3,2,3,4,6,5],
        vec![1, 3, 4, 5, 5],
        vec![1, 2, 2, 3, 4],
        vec![1, 2, 3, 4, 5, 5],
        vec![1, 2, 3, 4, 5],
    ];
    
    for nums in test_cases {
        println!("{:?}", find_error_nums(nums));
    }
}
