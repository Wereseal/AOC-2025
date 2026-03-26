/*
use std::fs;

pub fn solve_safe(){
    let mut position: i32 = 50;
    let mut direction: &str;
    let mut magnitude: i32;
    let mut counter: i32 = 0;
    let path = "resources/day1/safe_rotations.txt";
    println!("solving safe...");
    let instructions: String = fs::read_to_string(path)
        .unwrap();
    for instruction in instructions.lines(){
        direction = &instruction[..1];
        magnitude = instruction[1..].parse().unwrap();

        counter += magnitude / 100;
        magnitude = magnitude % 100;

        match direction{
            "L" => position -= magnitude,
            "R" => position += magnitude,
            _ => println!("Direction is not either of the options it is {direction}"),
        }
        if position > 100 {
            counter += 1;
            position -= 100;
        }
        else if position <= -1 {
            counter += 1;
            position += 100;
        }
        else if position == 0{
            counter += 1;
        }

    }
    println!("the code is: {counter}");
}
*/
