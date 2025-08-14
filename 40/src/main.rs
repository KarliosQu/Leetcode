use std::{backtrace, path};

fn main() {
    println!("Hello, world!");
}

struct Solution {

}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        Self::backstack(candidates, target, &mut result, &mut path);
        result  
    }

    pub fn backstack(candidates: Vec<i32>, target: i32,result: &mut Vec<Vec<i32>>,path: &mut Vec<i32>)  {
        if Self::sum(path.to_vec()) == target{
            
            
        },
    }

    pub fn sum(path: Vec<i32>) -> i32{
        let mut res = 0;
        for i in 1..path.len(){
            res = res + path[i];
        }
        res
    }
}
