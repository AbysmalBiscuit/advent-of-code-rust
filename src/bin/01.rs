use std::{collections::HashMap, thread};

advent_of_code::solution!(1);

struct LocationLists {
    list1: Vec<i32>,
    list2: Vec<i32>,
}

fn load_input_into_vectors(input: &str) -> LocationLists {
    let input_vec: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let input_vec_int: Vec<Vec<i32>> = input_vec
        .iter()
        .map(|inner_vec| {
            inner_vec
                .iter()
                .map(|item| item.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let (list1, list2) = input_vec_int
        .into_iter()
        .map(|sub_vec| (sub_vec[0], sub_vec[1]))
        .unzip();
    LocationLists { list1, list2 }
}

pub fn part_one(input: &str) -> Option<u64> {
    let location_lists = load_input_into_vectors(input);
    let mut list1 = location_lists.list1;
    let mut list2 = location_lists.list2;
    list1.sort_unstable();
    list2.sort_unstable();
    let midpoint = list1.len() / 2;
    let handle1 = thread::scope(|scope| {
        scope
            .spawn(|| {
                list1[..midpoint]
                    .iter()
                    .zip(list2[..midpoint].iter())
                    .map(|(i1, i2)| (i1 - i2).abs())
                    .sum::<i32>()
            })
            .join()
    });
    let handle2 = thread::scope(|scope| {
        scope
            .spawn(|| {
                list1[midpoint..]
                    .iter()
                    .zip(list2[midpoint..].iter())
                    .map(|(i1, i2)| (i1 - i2).abs())
                    .sum::<i32>()
            })
            .join()
    });
    let score = (handle1.unwrap() + handle2.unwrap()) as u64;
    Some(score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let location_lists: LocationLists = load_input_into_vectors(input);
    let list1: Vec<i32> = location_lists.list1;
    let list2: Vec<i32> = location_lists.list2;
    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    let _ = thread::scope(|scope| {
        scope
            .spawn(|| {
                let _ = list2
                    .into_iter()
                    .map(|i| *hashmap.entry(i).and_modify(|num| *num += 1).or_insert(1))
                    .collect::<Vec<_>>();
            })
            .join()
    });
    let score: i32 = list1
        .iter()
        .map(|num| num * hashmap.get(num).unwrap_or(&0))
        .sum();
    Some(score as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
