fn main() {
    fn get_max_cals() -> i32 {
        let input = std::fs::read_to_string("src/input.txt").unwrap();
        let lines = input.lines();
        let mut max = 0;
        let mut curr_cal = 0;
        for line in lines {
            if line == "" {
                if curr_cal > max {
                    max = curr_cal
                }
                curr_cal = 0;
            }
            else {
                curr_cal += line.parse::<i32>().unwrap();
            }
        }
        return max;
    }

    fn get_top_three_total_cals() -> i32 {
        let input = std::fs::read_to_string("src/input.txt").unwrap();
        let lines = input.lines();
        let mut v: Vec<i32> = Vec::new();
        let mut curr_cal = 0;
        for line in lines {
            if line == "" {
                v.push(curr_cal);
                curr_cal = 0;
            }
            else {
                curr_cal += line.parse::<i32>().unwrap();
            }
        }
        v.sort();
        let slice = v.as_slice()[v.len()-3..].to_vec();
        let mut result: i32 = 0;
        for n in slice {
            result += n;
        }
        return result;
    }


    let max_cals: String = get_max_cals().to_string();
    println!("{max_cals}");
    let top_three: String = get_top_three_total_cals().to_string();
    println!("{top_three}");
}
