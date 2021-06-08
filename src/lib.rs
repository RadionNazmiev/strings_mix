use std::cmp::Ordering::*;
use std::collections::HashMap;

fn str2meta(s: &str) -> HashMap<u8, u64> {
    let mut meta = HashMap::new();
    s.chars().for_each(|c| if c.is_ascii_lowercase() {*meta.entry(c as u8).or_insert(0) += 1});
    meta
}

fn mix(s1: &str, s2: &str) -> String {
    let (hm1, hm2) = (str2meta(s1),str2meta(s2));
    let mut res = ('a'..='z').filter_map(|c|
        match (hm1.get(&(c as u8)), hm2.get(&(c as u8))) {
            (Some(&x), Some(&y)) if x > 1 || y > 1 =>
                match x.cmp(&y) {
                    Less    => Some(format!("2:{}", c.to_string().repeat(y as usize))),
                    Equal   => Some(format!("=:{}", c.to_string().repeat(x as usize))),
                    Greater => Some(format!("1:{}", c.to_string().repeat(x as usize))),
                },
            (Some(&x), None) if x > 1 => Some(format!("1:{}", c.to_string().repeat(x as usize))),
            (None, Some(&y)) if y > 1 => Some(format!("2:{}", c.to_string().repeat(y as usize))),
            _ => None,

    }).collect::<Vec<String>>();
    res.as_mut_slice().sort_unstable_by(|a, b| match b.len().cmp(&a.len()) {
        Equal => a.cmp(&b),
        p => p
    });
    res.join("/")
}
#[cfg(test)]
mod tests {
    use super::mix;

    #[test]
    fn basics_mix() {
        testing("Are they here", "yes, they are here",
                "2:eeeee/2:yy/=:hh/=:rr");
        testing("looping is fun but dangerous", "less dangerous than coding",
                "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
        testing(" In many languages", " there's a pair of functions",
                "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
        testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
        testing("codewars", "codewars", "");
        testing("A generation must confront the looming ", "codewarrs",
                "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");
    }

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        assert_eq!(&mix(s1, s2), exp)
    }
}