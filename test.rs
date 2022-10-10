pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut result = nums.clone();
    result.extend(nums);
    result
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
            let sum =
                arr[i][j] +
                arr[i][j + 1] +
                arr[i][j + 2] +
                arr[i + 1][j + 1] +
                arr[i + 2][j] +
                arr[i + 2][j + 1] +
                arr[i + 2][j + 2];
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
    let double = n_floors * 2;
    for i in 0..double + 1 {
        if i % 2 == 1 {
            let whiteSpace: &str = &" ".repeat((double - (i + 1)) / 2);
            let stars: &str = &"*".repeat(i);
            let wholeStr = whiteSpace.to_owned() + &stars.to_owned() + &whiteSpace.to_owned();
            tower.push(wholeStr);
        }
    }
    tower
}

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .map(|&x| nums[x as usize])
            .collect()
    }
}

fn is_valid_ip(ip: &str) -> bool {
    let mut octets = ip.split('.');
    if octets.clone().count() != 4 {
        return false;
    }
    for octet in octets {
        if octet.len() == 0 || octet.len() > 3 {
            return false;
        }
        if octet.len() > 1 && octet.starts_with('0') {
            return false;
        }
        if let Err(_) = octet.parse::<u8>() {
            return false;
        }
    }
    true
}

fn is_valid_ip(ip: &str) -> bool {
    let mut vec = ip.split(".").collect::<Vec<&str>>();
    if vec.len() != 4 {
        return false;
    }

    for x in vec {
        if x.len() == 0 || x.len() > 3 {
            return false;
        }

        if x.len() > 1 && x.starts_with('0') {
            return false;
        }

        if let Err(_) = x.parse::<u8>() {
            return false;
        }
    }

    true
}

fn is_valid_ip(ip: &str) -> bool {
    let segments: Vec<&str> = ip.split(".").collect();
    segments.iter().all(|s| (!s.starts_with("0") || s.len() == 1) && s.parse::<u8>().is_ok()) &&
        segments.len() == 4
}

fn is_valid_ip(s: &str) -> bool {
    let mut parts = s.split('.');
    (0..4).all(|_| matches!(parts.next(), Some(p) if is_valid_ip_part(p))) && parts.next().is_none()
}

fn is_valid_ip_part(s: &str) -> bool {
    matches!(
        s.as_bytes(),
        [b'0'..=b'9'] |
            [b'1'..=b'9', b'0'..=b'9'] |
            [b'1', b'0'..=b'9', b'0'..=b'9'] |
            [b'2', b'0'..=b'4', b'0'..=b'9'] |
            [b'2', b'5', b'0'..=b'5']
    )
}

fn rotateLeft(d: i32, arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    //clone the array into something mutable
    let mut arr = arr.to_vec();

    //rotate the array
    arr.rotate_left(d as usize);

    //return the array
    arr
}