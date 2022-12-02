use std::fs;


pub fn get_groups() -> Vec<Vec<i32>> {
    let contents: String = fs::read_to_string("input1").expect("File not found");


    let mut groups: Vec<Vec<i32>> = Vec::new();
    let mut group: Vec<i32> = Vec::new();

    for line in contents.lines() {
        if line.len() == 0 {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line.parse::<i32>().unwrap());
        }
    }

    //push last group
    groups.push(group);
    groups
}
pub fn run() {
    let groups: Vec<Vec<i32>> = get_groups();

    let mut sum: i32 = 0;

    //getting the biggest from each group
    for group in groups {
        sum = sum.max(group.iter().sum())
    }

    println!("Day one result: {}", sum);

    run2()
}

pub fn run2() {
    let groups: Vec<Vec<i32>> = get_groups();

    //getting the total for each group
    let mut total: Vec<i32> = groups.iter().map(|group|
        group.iter().sum()
    ).collect();

    //sorting it
    total.sort();
    //reverse it so the largest is at the beginning
    total.reverse();

    let sum: i32 = total.get(0..3).expect("Not enough elves").iter().sum();

    println!("Day one part two answer: {}", sum);
}
