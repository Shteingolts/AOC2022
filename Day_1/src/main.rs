fn find_max(input: &Vec<Vec<i32>>) -> i32 {
    let mut max: i32 = 0;
    for v in input {
       let sum: i32 = v.iter().sum();
        if sum > max { max = sum} else {}
    }
    max
}

fn main() {
    // Process input from input.txt file
    let input: String = std::fs::read_to_string("data/input.txt")
                                .expect("Unable to read file");
    
    let input: Vec<&str> = input.split("\r\n\r\n")
                                .collect();
    
    let mut input_fin: Vec<Vec<i32>> = Vec::new();
    for s in input {
        let v: Vec<i32> = s.split("\r\n")
                                  .map(|x| x.parse().unwrap())
                                  .collect();
        input_fin.push(v);
    }

    // Find first answer
    let answer1 = find_max(&input_fin);
    println!("1st answer: {answer1}");

    // Find 2nd answer
    let mut sums: Vec<i32> = Vec::new();
    for v in input_fin {
        sums.push(v.iter().sum())
    }
    sums.sort();
    sums.reverse();

    let answer2 = sums[0] + sums[1] + sums[2];
    println!("2nd answer: {answer2}")
}
