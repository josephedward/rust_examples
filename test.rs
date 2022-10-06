
impl Solution {

    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums.clone();
        result.extend(nums);
        result
    }
    }
}



fn reverseArray(a: &[i32]) -> Vec<i32> {

    
    let mut b = Vec::new();
    for i in (0..a.len()).rev() {
        b.push(a[i]);
    }
    return b;

}


fn hourglassSum(arr: &[Vec<i32>]) -> i32 {

    let mut max = -100;
    for i in 0..4 {
        for j in 0..4 {
            let sum = arr[i][j] + arr[i][j + 1] + arr[i][j + 2] + arr[i + 1][j + 1] + arr[i + 2][j] + arr[i + 2][j + 1] + arr[i + 2][j + 2];
            if sum > max {
                max = sum;
            }
        }
    }
    return max;
}

fn spin_words(words: &str) -> String {
    let mut result = String::new();
    for word in words.split_whitespace() {
        if word.len() >= 5 {
            result.push_str(&word.chars().rev().collect::<String>());
        } else {
            result.push_str(word);
        }
        result.push(' ');
    }
    result.pop();
    result
}


//returns the single even or odd number in an array of numbers with one even or odd number and the rest even or odd numbers
fn find_outlier(values: &[i32]) -> i32 {
    
    let mut even = 0;
    let mut odd = 0;
    let mut even_num = 0;
    let mut odd_num = 0; 
    for i in 0..values.len() {
        if values[i] % 2 == 0 {
            even += 1;
            even_num = values[i];
        } else {
            odd += 1;
            odd_num = values[i];
        }

        if even > 1 && odd == 1 {
            return odd_num;
        } else if odd > 1 && even == 1 {
            return even_num;
        }



    }

    return 0;
}
 

fn tower_builder(n_floors: usize) -> Vec<String> {
    let mut tower = Vec::new();
    for i in 1..n_floors + 1 {
        let mut floor = String::new();
        for _ in 0..n_floors - i {
            floor.push(' ');
        }
        for _ in 0..i * 2 - 1 {
            floor.push('*');
        }
        for _ in 0..n_floors - i {
            floor.push(' ');
        }
        tower.push(floor);
    }
    tower
}



fn tower_builder(n_floors: usize) -> Vec<String> {
    let mut tower = Vec::new();
    let double = n_floors*2;
    for i in 0..double+1{
        if i%2==1{
            let whiteSpace: &str = &" ".repeat((double-(i+1))/2);
            let stars: &str = &"*".repeat(i);
            let wholeStr = whiteSpace.to_owned()+&stars.to_owned()+&whiteSpace.to_owned();
            tower.push(wholeStr);
        }                   
    }
    tower
}
