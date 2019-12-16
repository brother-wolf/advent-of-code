use crate::day3::geometry::{Vector, Line, Point, Direction};
use crate::helpers::converters::to_isize;


#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    distance: isize,
}


impl Instruction {
    pub fn from(s: &str) -> Option<Instruction> {
        let mut iter = s.chars();
        let d = iter.next().clone();
        let n = to_isize(iter.as_str().clone());
        if d.is_some() {
            let direction = Direction::from(d.unwrap());
            if direction.is_some() && n > 0 {
                Some(Instruction { direction: direction.unwrap(), distance: n })
            } else { None }
        } else { None }
    }

    pub fn to_vector(&self) -> Vector {
        let unit_vector = self.direction.to_unit_vector();
        let v_x = unit_vector.x * self.distance;
        let v_y = unit_vector.y * self.distance;
        Vector { x: v_x, y: v_y }
    }

}

fn instructions_from(raw_instructions: Vec<&str>) -> Vec<Instruction> {
    raw_instructions.iter()
        .flat_map(|raw| Instruction::from(raw))
        .collect::<Vec<Instruction>>()
}

fn generate_vectors_from(instructions: Vec<Instruction>) -> Vec<Vector> {
    let mut acc = vec![Vector { x: 0, y: 0}];
    instructions.iter().for_each(|instruction| {
//        let x1 = acc.last().unwrap();
        let vector = instruction.to_vector(); //.then(x1);
        acc.push(vector);
    });
    acc
}

fn generate_lines(vectors: Vec<Vector>) -> Vec<Line> {
    let mut acc = vec![Line::new(Point{x:0,y:0}, Point{x:0, y:0})];
    vectors.iter().skip(1).for_each(| vector| {
        let end_point = acc.last().unwrap().b;
        let next_point = Vector { x: end_point.x, y: end_point.y}.then(vector);
        let next_line = Line::new(end_point, Point{x: next_point.x, y: next_point.y});
        println!("{:?}", next_line);

        acc.push(next_line);

    });
    acc.remove(0);
    acc
}


pub fn find_intersection_points(wire_1: &Vec<Line>, wire_2: &Vec<Line>) -> Vec<Point> {
    let mut acc = vec![];
    for (c1, l1) in wire_1.iter().enumerate() { for (c2, l2) in wire_2.iter().enumerate() {
        match l2.intersect(&l1) {
            Some(inter) => if inter != (Point { x: 0, y: 0 }) { acc.push(inter) },
            None => (),
        }
    }}
    acc
}

pub fn build_path_from(input: Vec<&str>) -> Vec<Line> {
    generate_lines(
        generate_vectors_from(
            instructions_from(input)))
}

pub struct ManhattanDistance {
    pub distance: isize,
    pub point: Point,
}


pub fn manhattan_distance(point: Point) -> ManhattanDistance {
    ManhattanDistance { distance: point.x.abs() + point.y.abs(), point }
}


#[test]
fn instructions_returned_correctly() {
    assert_eq!(Instruction::from("R8").unwrap(), Instruction { direction: Direction::Right, distance: 8});
    assert_eq!(Instruction::from("L2").unwrap(), Instruction { direction: Direction::Left, distance: 2});
    assert_eq!(Instruction::from("U7").unwrap(), Instruction { direction: Direction::Up, distance: 7});
    assert_eq!(Instruction::from("D787").unwrap(), Instruction { direction: Direction::Down, distance: 787});
    assert!(Instruction::from("Y787").is_none());
    assert!(Instruction::from("D").is_none());
    assert!(Instruction::from("L0").is_none());
    assert!(Instruction::from("9").is_none());
    assert!(Instruction::from("").is_none());
}

#[test]
fn generate_vectors_from_raw_instructions() {
    let expected = vec![Vector { x: 0, y: 0 }, Vector{x:-1,y:0}, Vector{x:4,y:0}, Vector{x:0,y:4}, Vector{x:0,y:-19}];
    let one_line = generate_vectors_from(
        instructions_from(vec!["L1", "R4", "U4", "D19"]));
    assert_eq!(one_line, expected);
}

#[test]
fn generates_lines_correctly_from_one_raw_instructions() {
    let expected = vec![Line::new(Point{x:0,y:0}, Point{x:4,y:0})];
    let one_line = generate_lines(
        generate_vectors_from(
            instructions_from(vec!["R4"])));
    assert_eq!(one_line, expected);
}

#[test]
fn generates_lines_correctly_from_two_raw_instructions() {
    let expected = vec![
        Line::new(Point{x:0,y:0}, Point{x:4,y:0}),
        Line::new(Point{x:4,y:0}, Point{x:4,y:2})
    ];
    let vectors = generate_vectors_from(
        instructions_from(vec!["R4", "U2"]));
    println!("{:?}", vectors);
    let actual = generate_lines(
        vectors);

    assert_eq!(actual, expected);
}

#[test]
fn example_one_components() {
    let expected = vec![Point {x:6, y:5},Point {x:3, y:3}];

    let wire_1 = generate_lines(generate_vectors_from(instructions_from(vec!["R8","U5","L5","D3"])));
    let wire_2 = generate_lines(generate_vectors_from(instructions_from(vec!["U7","R6","D4","L4"])));
    let acc = find_intersection_points(&wire_1, &wire_2);

    assert_eq!(acc, expected);
}
