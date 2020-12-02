 use std::fs;

 //2-4 s: kswn
    fn main() {
        let mut part1_num = 0;
        let mut part2_num = 0;
            let filedata = fs::read_to_string("./src/data.txt")
            .expect("Something went wrong ");
            println!("{}",filedata);
        for line in filedata.split('\n') {
            let vec: Vec<&str> = line.split('-').collect();//-4 s:kswn
            if vec.len()>1 {
                let min = vec[0].parse::<i32>().unwrap_or(0);
                let temp_vec: Vec<&str> = vec[1].split(' ').collect();//[-4] [s:] [kswm]
                let max = temp_vec[0].parse::<i32>().unwrap();
                let search_letter = vec[1].chars().skip(vec[1].find(':').unwrap()-1).take(1).next().unwrap();
                let pw = vec[1].split(": ").collect::<Vec<&str>>()[1].to_owned();
                let mut let_num = 0;
                let mut index = 1;
                let mut boolean = false;
                for letter in pw.chars() {
                    if index == min && letter == search_letter {
                        boolean = true
                    }
                    if index == max && letter == search_letter {
                        if boolean {
                           boolean = false; 
                        }
                        else{
                            boolean = true;
                        }
                    }
                    if letter == search_letter{
                        let_num +=1;
                    }
                    index +=1;
                }
                if boolean {
                    part2_num +=1;
                }
                if let_num>= min && let_num<= max {
                    part1_num +=1;
                }
            }
        }
        println!("Part 1: {}",part1_num);
        println!("Part 2: {}",part2_num);
    }