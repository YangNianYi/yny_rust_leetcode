pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let mut visited = vec![false; rooms.len()];
    let mut visit = vec![0];
    while let Some(i) = visit.pop() {
        if !visited[i] {
            visited[i] = true;
            visit.extend(rooms[i].iter().map(|&x| x as usize));
        }
    }
    visited.iter().all(|&x| x)
}

pub fn count_and_say(n: i32) -> String {
    let mut say = "1".to_string();
    if n > 1 {
        for _ in 2..=n {
            say.push_str(" ");
            say = say
                .chars()
                .fold(
                    (0, 0 as char, "".to_string()),
                    |(mut last_count, last_char, mut res), next_char| {
                        if next_char != last_char {
                            if last_count > 0 {
                                res.push_str(&format!("{}{}", last_count, last_char));
                                // res = format!("{}{}{}", res, last_count, last_char);
                            }
                            last_count = 1
                        } else {
                            last_count += 1
                        }
                        (last_count, next_char, res)
                    },
                )
                .2;
        }
    }
    say
}

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.iter()
        .fold(vec![vec![]], |mut res: Vec<Vec<i32>>, &num| {
            res.extend(res.clone().iter_mut().map(|v| {
                v.push(num);
                v.clone()
            }));
            res
        })
}

pub fn is_valid(s: String) -> bool {
    let mut rv = Vec::with_capacity(s.chars().count());
    for c in s.chars() {
        match c {
            '(' => rv.push(')'),
            '[' => rv.push(']'),
            '{' => rv.push('}'),
            ls => {
                if Some(ls) != rv.pop() {
                    return false;
                }
            }
        }
    }
    rv.is_empty()
}

pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    let n = nums.len() - 1;
    let mut dp = nums.clone();
    for i in (0..n).rev() {
        for j in i + 1..=n {
            dp[j] = (nums[i] - dp[j]).max(nums[j] - dp[j - 1])
        }
    }
    dp[n] >= 0
}

pub fn rob(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((0, 0), |(prev, res), &i| (res, res.max(prev + i)))
        .1
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};

    nums.iter()
        .fold(HashMap::new(), |mut m, &i| {
            *m.entry(i).or_insert(0) += 1;
            m
        })
        .iter()
        .fold(BinaryHeap::new(), |mut h, (&k_num, &v)| {
            if h.len() < k as usize {
                h.push(Reverse((v, k_num)));
            } else if let Some(&Reverse((c, _))) = h.peek() {
                if v > c {
                    h.pop();
                    h.push(Reverse((v, k_num)));
                }
            }
            h
        })
        .iter()
        .map(|&Reverse((_, k))| k)
        .collect()
}

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    nums.iter()
        .fold(BinaryHeap::new(), |mut h, &i| {
            if h.len() < k as usize {
                h.push(Reverse(i));
            } else if let Some(&Reverse(v)) = h.peek() {
                if v < i {
                    h.pop();
                    h.push(Reverse(i));
                }
            }
            h
        })
        .peek()
        .map(|&Reverse(v)| v)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_the_winner() {
        assert_eq!(predict_the_winner(vec![0]), true);
        assert_eq!(predict_the_winner(vec![1, 5, 2, 4, 6]), true);
    }

    #[test]
    fn test_top_k_frequent() {
        let mut v1 = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        v1.sort();

        let mut v2 = top_k_frequent(vec![1], 1);
        v2.sort();

        let mut v3 = top_k_frequent(vec![2, 3, 4, 5, 2, 3, 4, 2, 3, 2], 2);
        v3.sort();

        assert_eq!(v1, vec![1, 2]);
        assert_eq!(v2, vec![1]);
        assert_eq!(v3, vec![2, 3]);
    }

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
}

fn main() {}
