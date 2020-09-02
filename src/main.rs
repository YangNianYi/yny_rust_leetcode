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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_the_winner() {
        assert_eq!(predict_the_winner(vec![0]), false);
        assert_eq!(predict_the_winner(vec![1, 5, 2, 4, 6]), true);
    }
}

fn main() {}
