mod advent {
    pub mod day1 {
        use std::fs::File;
        use std::io::{BufReader, BufRead};   

        struct Elf {
            carried_calories : Vec<i32>,
        }

        trait CarryBag {
            fn add_portion(&mut self, calories : i32);
            fn get_all_calories(&self) -> i32;
        }

        impl CarryBag for Elf {
            fn add_portion(&mut self, calories : i32){
                self.carried_calories.push(calories);
            }

            fn get_all_calories(&self) -> i32 {
                let mut all_calories = 0;

                for c in self.carried_calories.iter() {
                    all_calories += c;
                }

                all_calories
            }
        }

        pub fn read_input(){
            let path = "./data/day1";
            let input = File::open(path).unwrap();

            let buffered = BufReader::new(input);

            let mut elfes: Vec<Elf> = Vec::new();
            elfes.push(Elf{carried_calories: Vec::new()});

            for line in buffered.lines() {
                let l = line.unwrap();
                if l.is_empty() {
                    elfes.push(Elf{carried_calories: Vec::new()})
                } else {
                    let calories: i32 = l
                        .trim()
                        .parse()
                        .expect("Input not an integer");
                    elfes.last_mut().unwrap().add_portion(calories);
                }
            }

            let mut i = 1;
            let mut most_calories: Vec<i32> = Vec::new();
            println!("We have {} Elfes.", elfes.len());
            for elf in elfes.iter(){
                let all_calories = elf.get_all_calories();
                println!("Total calories of elf numer {} are {}", i, all_calories);
                i+=1;

                most_calories.push(all_calories);
            }

            most_calories.sort();

            println!("Most calories a elf is carried are {}", most_calories.last().unwrap());

            let top3_count = most_calories[most_calories.len()-1] + most_calories[most_calories.len()-2] + most_calories[most_calories.len()-3];

            println!("The calorie count of the Top3 elf carriers are {}", top3_count);
            

        }
    }
}

pub fn open_day_1() {
    crate::advent::advent::day1::read_input();
}