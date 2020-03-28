use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Debug)]
pub struct MovementDirection {
    dir : Direction,
    distance : i32,
}

pub fn read_03_input(filename : String, first_wire : &mut Vec<MovementDirection>, second_wire : &mut Vec<MovementDirection>) {
    let filename = format!("inputs/{}", filename);
    let input_str = read_to_string(filename);
    let parsed_string = match input_str {
        Ok(str) => {str},
        Err(e) => {panic!(e)},
    };
    let masses : Vec<String> = parsed_string.split_whitespace().map(|str| str.to_string()).collect();

    masses[0].split(',').map(|str| {
        let mut string : String = str.to_string();
        let first_char = string.chars().next().unwrap();
        let dir_result = match first_char {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'D' => Direction::Down,
            'U' => Direction::Up,
            _   => panic!()
        };
        string.remove(0);
        let input_distance =  string.parse::<i32>().unwrap();
        MovementDirection{
            dir : dir_result,
            distance : input_distance
        }
    }).for_each(|mov_dir| {
        first_wire.push(mov_dir);
    });

    masses[1].split(',').map(|str| {
        let mut string : String = str.to_string();
        let first_char = string.chars().next().unwrap();
        let dir_result = match first_char {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'D' => Direction::Down,
            'U' => Direction::Up,
            _   => panic!()
        };
        string.remove(0);
        let input_distance =  string.parse::<i32>().unwrap();
        MovementDirection{
            dir : dir_result,
            distance : input_distance
        }
    }).for_each(|mov_dir| {
        second_wire.push(mov_dir);
    });

}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Coordinates {
    x : i32,
    y : i32,
}

#[derive(Debug)]
pub struct GridStep {
    x : i32,
    y : i32,
    step : i32,
}

impl Ord for GridStep {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl PartialOrd for GridStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for GridStep {
    fn eq(&self, other: &Self) -> bool {
        (self.x, self.y) == (other.x, other.y)
    }
}

impl Eq for GridStep { }

pub struct WireGrid {
    x_coord : i32,
    y_coord : i32,
    num_of_steps : i32,
    grid_codes : BTreeSet<GridStep>
}

impl WireGrid {
    pub fn new() -> WireGrid {
        let mut wire = WireGrid {
            x_coord : 0,
            y_coord : 0,
            num_of_steps : 0,
            grid_codes : BTreeSet::new(),
        };
        wire
    }
    pub fn process_directions(&mut self, wire : Vec<MovementDirection>) {
        for direction in wire {
            match direction.dir {
                Direction::Up => self.move_up(direction.distance),
                Direction::Down => self.move_down(direction.distance),
                Direction::Left => self.move_left(direction.distance),
                Direction::Right => self.move_right(direction.distance),
            }
        }
        //self.grid_codes.remove(&(0, 0));
    }

    fn move_up(&mut self, distance : i32) {
        for i in 0..distance {
            self.y_coord += 1;
            self.num_of_steps += 1;
            let step = GridStep{
                x : self.x_coord,
                y : self.y_coord,
                step : self.num_of_steps,
            };
            self.grid_codes.insert(step);
        }
    }
    fn move_down(&mut self, distance : i32) {
        for i in 0..distance {
            self.y_coord -= 1;
            self.num_of_steps += 1;
            let step = GridStep{
                x : self.x_coord,
                y : self.y_coord,
                step : self.num_of_steps,
            };
            self.grid_codes.insert(step);
        }
    }
    fn move_left(&mut self, distance : i32) {
        for i in 0..distance {
            self.x_coord -= 1;
            self.num_of_steps += 1;
            let step = GridStep{
                x : self.x_coord,
                y : self.y_coord,
                step : self.num_of_steps,
            };
            self.grid_codes.insert(step);
        }
    }
    fn move_right(&mut self, distance : i32) {
        for i in 0..distance {
            self.x_coord += 1;
            self.num_of_steps += 1;
            let step = GridStep{
                x : self.x_coord,
                y : self.y_coord,
                step : self.num_of_steps,
            };
            self.grid_codes.insert(step);
        }
    }
}


#[test]
fn small_test_03() {
    let mut wire1 = Vec::new();
    let mut wire2 = Vec::new();

    read_03_input("03_small_input.txt".to_string(), &mut wire1, &mut wire2);
    println!("first wire: {:?}", wire1);
    println!("second wire {:?}", wire2);

    let mut first_wire = WireGrid::new();
    let mut second_wire = WireGrid::new();

    first_wire.process_directions(wire1);
    second_wire.process_directions(wire2);

    let results : Vec<_> = first_wire.grid_codes.intersection(&second_wire.grid_codes).clone().collect();
    println!("{:?}", results);

    let max_distance = results.into_iter().map(|pair| pair.x.abs() + pair.y.abs()).min().unwrap();
    println!("Max dist: {}", max_distance);
}

#[test]
fn basic_test_03() {
    let mut wire1 = Vec::new();
    let mut wire2 = Vec::new();

    read_03_input("03_input.txt".to_string(), &mut wire1, &mut wire2);
    println!("first wire: {:?}", wire1);
    println!("second wire {:?}", wire2);

    let mut first_wire = WireGrid::new();
    let mut second_wire = WireGrid::new();

    first_wire.process_directions(wire1);
    second_wire.process_directions(wire2);

    let results : Vec<_> = first_wire.grid_codes.intersection(&second_wire.grid_codes).clone().collect();
    println!("{:?}", results);

    let max_distance = results.into_iter().map(|pair| pair.x.abs() + pair.y.abs()).min().unwrap();
    println!("Max dist: {}", max_distance);
    assert_eq!(651, max_distance);
}

#[test]
fn small_adv_test_03() {
    let mut wire1 = Vec::new();
    let mut wire2 = Vec::new();

    read_03_input("03_small_input.txt".to_string(), &mut wire1, &mut wire2);
    println!("first wire: {:?}", wire1);
    println!("second wire {:?}", wire2);

    let mut first_wire = WireGrid::new();
    let mut second_wire = WireGrid::new();

    first_wire.process_directions(wire1);
    second_wire.process_directions(wire2);

    let results : Vec<_> = first_wire.grid_codes.intersection(&second_wire.grid_codes).clone().collect();
    println!("{:?}", results);

    let max_distance = results.iter()
        .map(|grid_step| {
            let grid_coord = GridStep{
                x : grid_step.x,
                y : grid_step.y,
                step : 0,
            };
            let step1 = first_wire.grid_codes.get(&grid_coord).unwrap().step;
            let step2 = second_wire.grid_codes.get(&grid_coord).unwrap().step;
            step1 + step2
        }).min().unwrap();

    println!("Max dist: {}", max_distance);
}

#[test]
fn advanced_test_03() {
    let mut wire1 = Vec::new();
    let mut wire2 = Vec::new();

    read_03_input("03_input.txt".to_string(), &mut wire1, &mut wire2);
    println!("first wire: {:?}", wire1);
    println!("second wire {:?}", wire2);

    let mut first_wire = WireGrid::new();
    let mut second_wire = WireGrid::new();

    first_wire.process_directions(wire1);
    second_wire.process_directions(wire2);

    let results : Vec<_> = first_wire.grid_codes.intersection(&second_wire.grid_codes).clone().collect();
    println!("{:?}", results);

    let max_distance = results.iter()
        .map(|grid_step| {
            let grid_coord = GridStep{
                x : grid_step.x,
                y : grid_step.y,
                step : 0,
            };
            let step1 = first_wire.grid_codes.get(&grid_coord).unwrap().step;
            let step2 = second_wire.grid_codes.get(&grid_coord).unwrap().step;
            step1 + step2
        }).min().unwrap();
    println!("Max dist: {}", max_distance);
}