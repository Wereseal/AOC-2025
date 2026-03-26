use std::fs;

struct Highest {
    value: u128,
    position: usize,
}
fn get_highest(battery: String) -> Highest{
    let mut char_as_num: u128;
    let mut highest = Highest {
        value: 0,
        position: 0,
    };
    for (index, char) in battery.chars().enumerate(){
        char_as_num = (char as u128)-48;
        if char_as_num > highest.value {
            highest.value = char_as_num;
            highest.position = index;
        }
    }
    highest
}

pub fn total_jolt(){
    let path = "resources/day3/joltage_banks";
    let batteries = fs::read_to_string(path).unwrap();
    let mut total: u128 = 0;
    let mut partition_point: usize = 0;
    let battery = String::from("818181911112111");
    let mut highest = Highest {
        value: 0,
        position: 0,
    };
    for battery in batteries.lines(){
        partition_point = 0;
        for i in 0..12{
            highest = get_highest(battery[partition_point..battery.len()-(11-i)].to_string());
            total += highest.value * (10_u64.pow((11-i) as u32)) as u128;
            partition_point += highest.position+1;
        }
    }
    println!("{}", total);
}

