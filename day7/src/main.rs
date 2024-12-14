use std::fs::read_to_string;
use std::path::Path;
use num::{BigInt,ToPrimitive};

fn main() {
    let path = Path::new("src/data.txt");
    let lines: Vec<String> = read_to_string(path).unwrap().lines().map(String::from).collect();

    let map: Vec<(BigInt, Vec<BigInt>)> = lines
        .into_iter()
        .map(|line| {
            let number_and_array: Vec<&str> = line.split(":").collect();
            let number = number_and_array[0].parse::<BigInt>().unwrap();
            let array = number_and_array[1]
                .split_whitespace()
                .map(|x| x.parse::<BigInt>().unwrap())
                .collect();
            (number, array)
        })
        .collect();

    let ans: BigInt = map
        .iter()
        .filter(|(tar, nums)| solve(tar.clone(), nums))
        .map(|(tar, _)| tar)
        .sum();

    if let Some(ans_as_i64) = ans.to_i64() {
        println!("{}", ans_as_i64);
    } else {
        println!("skibidi bit bro: {}", ans);
    }
}

fn solve(tar: BigInt, nums: &[BigInt]) -> bool {
    fn aux(nums: &[BigInt], curr_idx: usize, curr_val: BigInt, tar: &BigInt) -> bool {
        if curr_idx == nums.len() {
            return curr_val == *tar;
        }

        let num = &nums[curr_idx];

        // part 1 conditions
        if aux(nums, curr_idx + 1, &curr_val + num, tar) {
            return true;
        }

        if aux(nums, curr_idx + 1, &curr_val * num, tar) {
            return true;
        }

        // part 2 condition
        let num_dig = num.to_string().len() as u32;
        let power_of_ten = BigInt::from(10).pow(num_dig);
        let conc_val = &curr_val * &power_of_ten + num;
        if aux(nums, curr_idx + 1, conc_val, tar) {
            return true;
        }


        false
    }

    aux(nums, 1, nums[0].clone(), &tar)
}


