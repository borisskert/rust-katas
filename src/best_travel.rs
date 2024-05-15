/// https://www.codewars.com/kata/55e7280b40e1c4a06d0000aa/train/rust
pub fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    return go(t, k, ls, 0, 0);
}

fn go(max: i32, count: i32, list: &Vec<i32>, index: usize, sum: i32) -> i32 {
    if count == 0 && sum <= max {
        return sum;
    }
    
    if sum > max || index >= list.len() {
        return -1;
    }
    
    let a = go(max, count - 1, list, index + 1, sum + list[index]);
    let b = go(max, count, list, index + 1, sum);
    
    if a == -1 {
        return b;
    }
    
    if b == -1 {
        return a;
    }
    
    return if a > b { a } else { b };
}
