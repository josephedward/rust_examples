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
    //clone the array into something mutable
    let mut arr = arr.to_vec();

    //rotate the array
    arr.rotate_left(d as usize);

    //return the array
    arr
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn most_words_found(sentences: Vec<String>) -> i32 {
    let mut count = 0;
    for x in sentences.into_iter().map(|x| String::from(x).split(' ').count()) {
        if x > count {
            count = x;
        }
    }
    count as i32
}

pub fn is_palindrome(x: i32) -> bool {
    let mut x = x;
    for i in 0..=x.to_string().len() / 2 {
        if
            x.to_string().chars().nth(i).unwrap() !=
            x
                .to_string()
                .chars()
                .nth(x.to_string().len() - 1 - i)
                .unwrap()
        {
            return false;
        }
    }
    true
}

fn solution(num: i32) -> i32 {
    let mut sum = 0;
    for x in 1..num {
        if x % 3 == 0 {
            sum = sum + x;
            continue;
        }
        if x % 5 == 0 {
            sum = sum + x;
            continue;
        }
    }
    sum
}

fn solution(num: i32) -> i32 {
    (1..num).filter(|x| (x % 3 == 0 || x % 5 == 0)).sum()
}

fn solution(num: i32) -> i32 {
    let mut sum = 0;
    for i in 1..num {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

match io::stdin().read_line(&mut user_input) {
    Ok(_) => (),
    Err(_e) => {
        println!("error: {}", _e);
        continue;
    }
}

if let Err(e) = io::stdin().read_line(&mut user_input) {
    eprintln!("error: {}", e);
    continue;
}
let num: i64 = match user_input.trim().parse::<i64>() {
    Ok(n) => n,
    Err(_e) => {
        trim_newline(&mut user_input); // remove trailing newline
        println!("'{}' is not a valid number, full error: {}", user_input, _e);
        continue;
    }
};

fn zeros(n: u64) -> u64 {
    let mut n = n.clone();
    let mut sum = 0;
    //      A trailing zero is always produced by prime factors 2 and 5.
    //      if we count 5s in prime factors, we are done
    while n >= 5 {
        n = n / 5;
        sum += n;
    }
    sum
}

fn zeros(n: u64) -> u64 {
    //     println!("N -> {}", n);
    if n == 0 {
        0
    } else {
        n / 5 + zeros(n / 5)
    }
}


//declare empty vec of i32
let mut vec: Vec<i32> = Vec::new();


pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    
    let mut i = 0;
    while i < nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            nums.remove(i);
        } else {
            i += 1;
        }
    }
    nums.len() as i32
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    return nums.len() as i32
    }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        
        nums.len() as i32
 }


 //split string of spaced integers into an array of integeres
    let mut nums: Vec<i32> = nums.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    //sort the array
    nums.sort();

    //remove duplicates
    nums.dedup();

    //return the length of the array
    nums.len() as i32


    fn main() {
        let a = arr2(&[[1, 2, 3],
                       [4, 5, 6]]);
    
        let b = arr2(&[[6, 3],
                       [5, 2],
                       [4, 1]]);
    
        println!("{}", a.dot(&b));

        a.iter()
        .zip(b.iter())
        .map(|(a, b)| a * b)
        .sum()


        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            for x in 0..nums.len(){
            for y in 0..nums.len(){
                if nums[x] as i32 +nums[y] as i32==target{
                    println!("{:?}", (x,y));
                    if x!=y{
                    return [x as i32,y as i32].to_vec();
                    }
                }
            }
        }
        
        
        [0 as i32,0 as i32].to_vec()
    
        }


    

        fn simpleArraySum(ar: &[i32]) -> i32 {
            let sum: i32 = ar.iter().sum();
            sum
        }

        fn insertNodeAtPosition(llist: *const SinglyLinkedListNode, data: i32, position: i32) -> *const SinglyLinkedListNode {
            let mut head = llist;
            let mut count = 0;
            let mut prev = llist;
            while !head.is_null() {
                if count == position {
                    let new_node = Box::new(SinglyLinkedListNode::new(data));
                    new_node.next = head;
                    unsafe {
                        (*prev).next = Box::into_raw(new_node);
                    }
                    break;
                }
                count += 1;
                prev = head;
                unsafe {
                    head = (*head).next;
                }
            }
            llist
 
        }


        fn insertNodeAtPosition(llist: *const SinglyLinkedListNode, data: i32, position: i32) -> *const SinglyLinkedListNode {
            if position == 0 {
                let new_node = Box::new(SinglyLinkedListNode::new(data));
                unsafe {
                    (*(*new_node)).next = llist as *mut SinglyLinkedListNode;
                }
                return Box::into_raw(new_node) as *const SinglyLinkedListNode;
                } else {
        let new_node = Box::new(SinglyLinkedListNode::new(data));
                let mut head = llist;
                let mut count = 0;
                let mut prev = llist as *mut SinglyLinkedListNode;
                while !head.is_null() {
                    if count == position {
                        unsafe {
                            (*(*new_node)).next = head as *mut SinglyLinkedListNode;
                            (*prev).next = Box::into_raw(new_node) as *mut SinglyLinkedListNode;
                        }
                        break;
                    }
                    count += 1;
                    prev = head as *mut SinglyLinkedListNode;
                    unsafe {
                        head = (*head).next;
                    }
                }}
                llist
        }

        //do the same thing but do not insert the zero 

        fn insertNodeAtPosition(llist: *const SinglyLinkedListNode, data: i32, position: i32) -> *const SinglyLinkedListNode {
            if position == 0 {
                let new_node = Box::new(SinglyLinkedListNode::new(data));
                unsafe {
                    (*(*new_node)).next = llist as *mut SinglyLinkedListNode;
                }
                return Box::into_raw(new_node) as *const SinglyLinkedListNode;
                } else {
        let new_node = Box::new(SinglyLinkedListNode::new(data));
                let mut head = llist;
                let mut count = 0;
                let mut prev = llist as *mut SinglyLinkedListNode;
                while !head.is_null() {
                    if count == position {
                        unsafe {
                            (*(*new_node)).next = head as *mut SinglyLinkedListNode;
                            (*prev).next = Box::into_raw(new_node) as *mut SinglyLinkedListNode;
                        }
                        break;
                    }
                    count += 1;
                    prev = head as *mut SinglyLinkedListNode;
                    unsafe {
                        head = (*head).next;
                    }
                }}
                llist
        }
        use std::env;
        use std::fs::File;
        use std::io::{self, BufRead, Write};
        use std::ptr;
        
        struct SinglyLinkedListNode {
            data: i32,
            next: *mut SinglyLinkedListNode,
        }
        
        struct SinglyLinkedList {
            head: *mut SinglyLinkedListNode,
            tail: *mut SinglyLinkedListNode,
        }
        
        impl SinglyLinkedListNode {
            pub fn new(data: i32) -> *mut Self {
                Box::into_raw(Box::new(SinglyLinkedListNode {
                    data,
                    next: ptr::null_mut(),
                }))
            }
        }
        
        impl Drop for SinglyLinkedListNode {
            fn drop(&mut self) {
                self.next = ptr::null_mut();
            }
        }
        
        impl SinglyLinkedList {
            pub fn new() -> Self {
                SinglyLinkedList { head: ptr::null_mut(), tail: ptr::null_mut() }
            }
        
            pub fn insert_node(&mut self, data: i32) {
                unsafe {
                    let node = SinglyLinkedListNode::new(data);
        
                    if self.head.is_null() {
                        self.head = node;
                    } else {
                        (*self.tail).next = node;
                    }
        
                    self.tail = node;
                }
            }
        }
        
        impl Drop for SinglyLinkedList {
            fn drop(&mut self) {
                while !self.head.is_null() {
                    unsafe {
                        if !self.head.is_null() {
                            let head = Box::from_raw(self.head);
                            self.head = head.next;
                        }
                    }
                }
        
                self.tail = ptr::null_mut();
            }
        }
        
        fn print_singly_linked_list(head: *const SinglyLinkedListNode, sep: &str, fptr: &mut File) {
            let mut node = head;
        
            while !node.is_null() {
                unsafe {
                    write!(fptr, "{}", (*node).data).ok();
        
                    node = (*node).next;
                }
        
                if !node.is_null() {
                    write!(fptr, "{}", sep).ok();
                }
            }
        }
        
        /*
         * Complete the 'insertNodeAtPosition' function below.
         *
         * The function is expected to return an INTEGER_SINGLY_LINKED_LIST.
         * The function accepts following parameters:
         *  1. INTEGER_SINGLY_LINKED_LIST llist
         *  2. INTEGER data
         *  3. INTEGER position
         */
        
        /*
         * For your reference:
         *
         * SinglyLinkedListNode {
         *     data: i32,
         *     next: *mut SinglyLinkedListNode,
         * };
         *
         */
        
        //    fn insertNodeAtPosition(llist: *const SinglyLinkedListNode, data: i32, position: i32) -> *const SinglyLinkedListNode {
        //             if position == 0 {
        //                 let new_node = Box::new(SinglyLinkedListNode::new(data));
        //                 unsafe {
        //                     (*(*new_node)).next = llist as *mut SinglyLinkedListNode;
        //                 }
        //                 return Box::into_raw(new_node) as *const SinglyLinkedListNode;
        //             } else {
        //                 let mut node = llist as *mut SinglyLinkedListNode;
        //                 let mut new_node = Box::new(SinglyLinkedListNode::new(data));
                        
        //                 for i in 0..position+1 {
        //                     println!("i {:?}", i);
        //                     println!("position {:?}", position);
        //                     unsafe {
        //                         if i == position {
        //                             println!("new_node {:?}", new_node);
        //                             println!("node {:?}", node);  
        //                             (*node).data = (*(*new_node)).data;      
        //                             (*(*new_node)).next = node;  
        //                             // node = new_node as *mut SinglyLinkedListNode;                      
        //                         } else {
        //                             node = (*node).next;

        //                         }
                                
        //                         // node = (*node).next;     
        //                         println!("(*(*new_node)).data {:?}", (*(*new_node)).data);
        //                         println!("(*node).data {:?}", (*node).data);
        //                         println!("(*node).next {:?}", (*node).next);
        
        //                     }
        //                 }              
        //             }
        //                 llist
        //         }
        
fn insertNodeAtPosition(llist: *const SinglyLinkedListNode, data: i32, position: i32) -> *const SinglyLinkedListNode {
            if position == 0 {
                let new_node = Box::new(SinglyLinkedListNode::new(data));
                unsafe {
                    (*(*new_node)).next = llist as *mut SinglyLinkedListNode;
                }
                return Box::into_raw(new_node) as *const SinglyLinkedListNode;
                } else {
                    let mut head = llist;
                    let mut count = 0;
                    let mut prev = llist as *mut SinglyLinkedListNode;
                    while !head.is_null() {
                        if count == position {
                            let new_node = Box::new(SinglyLinkedListNode::new(data));
                            unsafe {
                                (*(*new_node)).next = head as *mut SinglyLinkedListNode;
                                (*prev).next = Box::into_raw(new_node) as *mut SinglyLinkedListNode;
                            }
                            break;
                        }
                        count += 1;
                        prev = head as *mut SinglyLinkedListNode;
                        unsafe {
                            head = (*head).next;
                        }
                    }}
                    llist
                }


//

        fn main() {
            let stdin = io::stdin();
            let mut stdin_iterator = stdin.lock().lines();
        
            let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
        
            let llist_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
            let mut llist = SinglyLinkedList::new();
        
            for _ in 0..llist_count {
                let llist_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        
                llist.insert_node(llist_item);
            }
        
            let data = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        
            let position = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        
            let llist_head = insertNodeAtPosition(llist.head, data, position);
        
            print_singly_linked_list(llist_head, " ", &mut fptr);
            writeln!(&mut fptr).ok();
        }
        


        let mut vec = start
        
        let mut vec = vec.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();



fn ips_between(start: &str, end: &str) -> u32 {
    let mut vec = start.split(".").collect::<Vec<&str>>();
    let mut vec2 = end.split(".").collect::<Vec<&str>>();
    let mut vec = vec.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut vec2 = vec2.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut count = 0;
    for i in 0..vec.len() {
        if vec[i] != vec2[i] {
            
            // count += (vec2[i] - vec[i]) * 256.pow((vec.len() - i - 1) as u32);
            // can't call method `pow` on ambiguous numeric type `{integer}`
            count += (vec2[i] - vec[i]) * 256.pow((vec.len() - i - 1) as u32);
        }
    }
    count
}



fn ips_between(start: &str, end: &str) -> u32 {
    
    println!("{:?}", start);
    println!("{:?}", end);
    let start_vec=ip_vec(start);
    let end_vec=ip_vec(end);
    let mut result= 0;
    
    if start_vec[2] != end_vec[2] && start_vec[3] != end_vec[3] {
        result = diff(start_vec[2], end_vec[2]);
        result = result*256;
        let offset= diff(start_vec[3], end_vec[3]);
        result = result - offset;
        println!("result : {:?}", result);     
    } else if start_vec[2] != end_vec[2]  {
            result= diff(start_vec[2], end_vec[2]);
            result = result*256;
            println!("result : {:?}", result);     
    }else if start_vec[3] != end_vec[3] {
            result= diff(start_vec[3], end_vec[3]);
            println!("result : {:?}", result);        
    }


    result as u32
}

fn ip_vec(ip: &str) -> Vec<i32>{
    let vec = ip.split(".").collect::<Vec<&str>>();
    let vec = vec.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    vec
}

fn diff(num1: i32, num2: i32) -> i32{
    let mut res = num1 - num2;
    res = res.abs();
    println!("diff: {:?}", res);
    res
}


//     
//     result = result.abs();
    
    
//     for i in 0..start_vec.len(){
//         if i != 3{
//             let count = 3-i;
//             result += diff(start_vec[i], end_vec[i])*256_i32.pow(count as u32);
//         } else {
//             result -= diff(start_vec[i], end_vec[i])
//         }
    
// //         result = result.abs();
//         println!("result : {:?}", result);     
// //         if start_vec[i] != end_vec[i]{
// //             let count = 4 - i - 1;
// //             println!("count : {:?}", count);    
// //             result += (start_vec[i] - end_vec[i]) * (256 as i32).pow(count.try_into().unwrap());
// //         } 
//     }

//     if start_vec[2] != end_vec[2] && start_vec[3] != end_vec[3] {
//         result = diff(start_vec[2], end_vec[2]);
//         result = result*256;
//         let offset= diff(start_vec[3], end_vec[3]);
//         result = result - offset;
//         println!("result : {:?}", result);     
//     } else if start_vec[2] != end_vec[2]  {
//             result= diff(start_vec[2], end_vec[2]);
//             result = result*256;
//             println!("result : {:?}", result);     
//     }else if start_vec[3] != end_vec[3] {
//             result= diff(start_vec[3], end_vec[3]);
//             println!("result : {:?}", result);        
//     }


let num1 = diff(start_vec[0],end_vec[0]).overflowing_mul(16777216);
result = num1+num2+num3-num4;  // cannot add `i32` to `(i32, bool)`
//add i32 to (i32, bool)





fn ips_between(start: &str, end: &str) -> u32 {
    
    println!("{:?}", start);
    println!("{:?}", end);
    let start_vec=ip_vec(start);
    let end_vec=ip_vec(end);
    let mut result= 0;
    
    let num1 = diff(start_vec[0],end_vec[0]).wrapping_mul(16777216);
    println!(" num1 {:?}", num1);
    let num2 = diff(start_vec[1],end_vec[1]) * 65536;
    println!("num2 {:?}", num2);
    let num3 = diff(start_vec[2],end_vec[2]) * 256;
    println!("num3 {:?}", num3);
    let num4 = diff(start_vec[3],end_vec[3]);
    println!("num4 {:?}", num4);

    if start_vec[3]>end_vec[3]{
       result = num1+num2+num3-num4; 
        return result as u32
    }
    if start_vec[3]<end_vec[3]{
       result = num1+num2+num3+num4;
        return result as u32
    }
    result = num1+num2+num3+num4;
    println!("result : {:?}", result);        

    result as u32
}

fn ip_vec(ip: &str) -> Vec<i32>{
    let vec = ip.split(".").collect::<Vec<&str>>();
    let vec = vec.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    vec
}

fn diff(num1: i32, num2: i32) -> i32{
    let mut res = num1 - num2;
    res = res.abs();
    println!("diff: {:?}", res);
    res
}


use std::net::Ipv4Addr;

fn ips_between(start: &str, end: &str) -> u32 {
    let start: u32 = start.parse::<Ipv4Addr>().unwrap().into();
    let end: u32 = end.parse::<Ipv4Addr>().unwrap().into();
    end - start
}



let found = false; 
let mut count = 1; 
let mut smallest = n.clone();

while !found{
    if smallest%2==0 {
        return smallest;
    }
    count+=1;
    smallest=smallest*count;
}

0


fn move_zeros(xs: &[u8]) -> Vec<u8> {
    let mut ys = Vec::with_capacity(xs.len());
    ys.extend(xs.iter().filter(|&&x| x != 0));
    ys.resize(xs.len(), 0);
    ys
}

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let ans: Vec<u8> = [0].to_vec();
    let mut num = arr.to_vec();
    let count = num.iter().filter(|&n| *n == 0).count();
    num.retain(|value| *value != 0);
    for x in 0..count{
        num.push(0)
    }
    num
}


fn increment_string(s: &str) -> String {
    let test: String = "test".to_string();
    println!("{:?}", s);
    let cvec: Vec<char> = s.chars().collect();
    println!("{:?}", cvec);

    let index_element = cvec
        .iter()
        .position(|&x|x.is_numeric())
        .unwrap();
    
    println!("Index of b is {}", index_element);
    
        
    let test2 = &cvec[index_element..cvec.len()];
    
    let joinVector = test2.iter().collect::<String>();
    let mut my_u32: u32 = joinVector.parse::<u32>().unwrap();    
    
    my_u32+=1;
    println!("vec: {:?}, string: {:?}", test2, my_u32);
//     println!("{:?}", test2);
    return test
}

fn increment_string(s: &str) -> String {
    let mut ans: String = "".to_string();
    println!("{:?}", s);
    let cvec: Vec<char> = s.chars().collect();
    println!("{:?}", cvec);

    let index_element = cvec
        .iter()
        .position(|&x|x.is_numeric())
        .unwrap_or(0);
    
    if index_element == 0 {
        return s.to_owned()+"1";
    }
    
    println!("Index of b is {}", index_element);
    let alphas = &cvec[0..index_element];    
    let nums = &cvec[index_element..cvec.len()];
    
    let alphaVector = alphas.iter().collect::<String>();
    let numVector = nums.iter().collect::<String>();
    
    let mut my_u32: u32 = numVector.parse::<u32>().unwrap();    
    my_u32+=1;
    
    ans = alphaVector.to_owned()+&format!("{:0width$}", my_u32, width = numVector.len()).to_string();
    
    return ans
}


fn increment_string(s: &str) -> String {
    let mut ans: String = "".to_string();
//     println!("{:?}", s);
    let cvec: Vec<char> = s.chars().collect();
//     println!("{:?}", cvec);

    let index_element = cvec
        .iter()
        .position(|&x|x.is_numeric())
        .unwrap_or(0);
    println!("Index of number is {}", index_element);
    
    let alphas = &cvec[0..index_element];    
    let nums = &cvec[index_element..cvec.len()];
    
    let alphaVector = alphas.iter().collect::<String>();
    let numVector = nums.iter().collect::<String>();
    
    if alphaVector.len() == 0 {
        return s.to_owned()+"1";
    }
    
    if alphaVector.len() == 0 && numVector.len() !=0 {
        
    }
    
    if numVector.len() !=0 {
    let mut my_u32: u32 = numVector.parse::<u32>().unwrap_or(0);    
    my_u32+=1;
    ans = alphaVector.to_owned()+&format!("{:0width$}", my_u32, width = numVector.len()).to_string();
    }
    
    return ans
}



//find the product of a vec of i32s
fn product(vec: Vec<i32>) -> i32 {
    let mut result = 1;
    for i in vec {
        result *= i;
    }
    result
}


//find prime factors of a number
fn prime_factors(n: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    let mut n = n;
    let mut i = 2;
    while i <= n {
        if n % i == 0 {
            factors.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    factors
}


/*
 * Complete the 'connectingTowns' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY routes
 */

 fn connectingTowns(n: i32, routes: &[i32]) -> i32 {
    let mut result = 1;
    for i in routes {
        result *= i;
    }
    result % 1234567
}

fn aVeryBigSum(ar: &[i64]) -> i64 {
    let mut sum = 0;
    for i in ar {
        sum += i;
    }
    sum
}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        //count occurences of each number
        // let mut count = HashMap::new();
        for i in nums {
            if nums.iter().filter(|&x| *x == i).count() == 1 {
                return i;
            }
        }    
    }
}
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
 
        let mut list1 = list1;
        let mut list2 = list2;
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = &mut head;
        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                // current.as_mut().unwrap().next = list1; throws
                
                // current = &mut current.as_mut().unwrap().next; use l1
                
                list1 = list1.unwrap().next;
            } else {
                // current.as_mut().unwrap().next = list2;
                current = &mut current.as_mut().unwrap().next;
                list2 = list2.unwrap().next;
            }
            current = &mut current.as_mut().unwrap().next;
        }
        if list1.is_some() {
            current.as_mut().unwrap().next = list1;
        } else {
            current.as_mut().unwrap().next = list2;
        }
        head.unwrap().next
    }
}


fn dna_strand(dna: &str) -> String {
    // Translate the DNA strand
    //   let match = {
    //       "A":"T"
    //       "T":"A"
    //       "C":"G"
    //       "G":"C"
    //   } 
    //   println!("{:?}", dna);
    //   dna.to_string()
      let mut result = String::new();
        for c in dna.chars() {
            match c {
                'A' => result.push('T'),
                'T' => result.push('A'),
                'C' => result.push('G'),
                'G' => result.push('C'),
                _ => (),
            }
        }
  }