mod day1 {
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

    pub fn day1(){
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
mod day2 {
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    enum Shape {
        Rock,
        Paper,
        Scissor,
    }

    enum Result {
        Lose,
        Draw,
        Win,
    }

    fn shape_value(shape : &Shape) -> i32 {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }

    fn evaluate_move(own_shape : &Shape, enemy_shape : &Shape) -> i32 {
        let mut score = shape_value(own_shape);

        score += match own_shape {
            Shape::Rock => match enemy_shape {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissor => 6,
            },
            Shape::Paper => match enemy_shape {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissor => 0,
            },
            Shape::Scissor => match enemy_shape {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissor => 3,
            }
        };

        score
    }

    fn get_shape_by_result(result : &Result, enemy_shape : &Shape) -> Shape {
        match enemy_shape {
            Shape::Rock => match result {
                Result::Lose => Shape::Scissor,
                Result::Draw => Shape::Rock,
                Result::Win => Shape::Paper,
            },
            Shape::Paper =>match result {
                Result::Lose => Shape::Rock,
                Result::Draw => Shape::Paper,
                Result::Win => Shape::Scissor,
            },
            Shape::Scissor => match result {
                Result::Lose => Shape::Paper,
                Result::Draw => Shape::Scissor,
                Result::Win => Shape::Rock,
            },
        }
    }

    pub fn day2() {
        let data_path = "./data/day2";
        let input = File::open(data_path).unwrap();

        let buffered = BufReader::new(input);

        let mut my_shape: Shape;
        let mut enemy_shape : Shape;
        let mut score = 0;
        for line in buffered.lines() {
            let l = line.unwrap();
            let split = l.split(' ');
            let x : Vec<&str> = split.collect();
            match x[0] {
                "A" => enemy_shape = Shape::Rock,
                "B" => enemy_shape = Shape::Paper,
                "C" => enemy_shape = Shape::Scissor,
                _ => panic!("false input"),
            }
            match x[1] {
                "X" => my_shape = Shape::Rock,
                "Y" => my_shape = Shape::Paper,
                "Z" => my_shape = Shape::Scissor,
                _ => panic!("false input"),
            }

            score += evaluate_move(&my_shape, &enemy_shape);

        }

        println!("Total score after turnier is : {}", score);
    }

    pub fn day2_correct() {
        let data_path = "./data/day2";
        let input = File::open(data_path).unwrap();

        let buffered = BufReader::new(input);

        let mut my_shape: Shape;
        let mut enemy_shape : Shape;
        let mut score = 0;
        for line in buffered.lines() {
            let l = line.unwrap();
            let split = l.split(' ');
            let x : Vec<&str> = split.collect();
            match x[0] {
                "A" => enemy_shape = Shape::Rock,
                "B" => enemy_shape = Shape::Paper,
                "C" => enemy_shape = Shape::Scissor,
                _ => panic!("false input"),
            }
            match x[1] {
                "X" => my_shape = get_shape_by_result(&Result::Lose, &enemy_shape),
                "Y" => my_shape = get_shape_by_result(&Result::Draw, &enemy_shape),
                "Z" => my_shape = get_shape_by_result(&Result::Win, &enemy_shape),
                _ => panic!("false input"),
            }

            score += evaluate_move(&my_shape, &enemy_shape);
        }

        println!("Total score after turnier with the correct plan is : {}", score); 
    }
}

pub fn open_day_1() {
    crate::advent::day1::day1();
}

pub fn open_day_2() {
    crate::advent::day2::day2();
    crate::advent::day2::day2_correct();
}