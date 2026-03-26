/*use std::fs;

pub fn sum_invalid_id_1(){
    let path = "resources/day2/ID_ranges";
    let file = fs::read_to_string(path).expect("File not found.");
    let ranges = file.split(',');
    let mut invalid_id_sum: u64 = 0;
    let mut id_len: usize;
    for range in ranges{
        println!("the range is: {range}");
        let mut bounds = range.split('-');
        let lower_bound: u64 = match bounds.next(){
            Some(bound) => {
                bound.trim().parse().expect("Lower bound can't be parsed.")
            }
            None => panic!("No lower bound."),
        };
        let upper_bound: u64 = match bounds.next(){
            Some(bound) => {
                println!("{}", bound.trim());
                bound.trim().parse().expect("Upper bound can't be parsed.")
            }
            None => panic!("No upper bound."),
        };
        for id in lower_bound..upper_bound+1{
            let id_str = id.to_string();
            id_len = (id_str).len();
            if id_len >= 2 && id_len % 2 == 0{
                if id_str[..id_len/2] == id_str[id_len/2..] {
                    invalid_id_sum += id;
                }
            }
        }
    }
    println!("Sum is: {}", invalid_id_sum);
}
fn check_valid(id: u64) -> bool {
    let id_str = id.to_string();
    let id_len = id_str.len();
    let mut partitions = Vec::<&str>::new();
    let mut factors = Vec::<usize>::new();
    let mut valid = true;
    for i in 1..id_len{
        if id_len % i == 0{
            factors.push(i); 
        }
    }
    for factor in &factors{
        valid = true;
        partitions.clear();
        for i in 0..id_len/factor{
            partitions.push(&id_str[i*factor..(i+1)*factor]);
        }
        for part in &partitions{
            if valid == true{
                valid = part == &partitions[0];
            }
        }
        if valid {
            return true;
        }
    }
    return false;
    
}

pub fn sum_invalid_id_2(){
    let path = "resources/day2/ID_ranges";
    let file = fs::read_to_string(path).expect("File not found.");
    let ranges = file.split(',');
    let mut invalid_id_sum: u64 = 0;
    for range in ranges{
        let mut bounds = range.split('-');
        let lower_bound: u64 = match bounds.next(){
            Some(bound) => {
                bound.trim().parse().expect("Lower bound can't be parsed.")
            }
            None => panic!("No lower bound."),
        };
        let upper_bound: u64 = match bounds.next(){
            Some(bound) => {
                bound.trim().parse().expect("Upper bound can't be parsed.")
            }
            None => panic!("No upper bound."),
        };
        for id in lower_bound..upper_bound+1{
            if check_valid(id){
                invalid_id_sum += id
            }
        }
    }
    println!("Sum is: {}", invalid_id_sum);
}
*/
