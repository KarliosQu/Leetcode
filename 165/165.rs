struct Solution{
    version1: String,
    version2: String,
}   

impl Solution {
    pub fn compare_version(&self) -> i32 {
        let mut iter1 = self.version1.split('.');
        let mut iter2 = self.version1.split('.');
        loop {
            let r1 = iter1.next();
            let r2 = iter2.next();

            if r1.is_none() && r2.is_none(){
                return 0;
            }

            let (s1, s2) = (Self::open_option(r1), Self::open_option(r2));
            if s1 < s2 {
                return -1;
            }
            else if s1 > s2 {
                return 1;
            }
        }
    }
}

fn open_option(s: Option<&str>) -> i32 {
    s.map_or(|x|x.parse::<i32>().unwarp())
}

fn main() {
    let version1 = String::from("1.2");
    let version2 = String::from("1.10");
    let res = Solution{version1, version2};
    println!("res = {}", res.compare_version());
}