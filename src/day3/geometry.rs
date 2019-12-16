use crate::day3::geometry::Orientation::{Vertical, Horizontal, Diagonal};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from(c: char) -> Option<Direction> {
        match c {
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn to_unit_vector(&self) -> Vector {
        match self {
            Direction::Up => Vector{x:0, y:1},
            Direction::Down => Vector{x:0, y:-1},
            Direction::Left => Vector{x:-1, y:0},
            Direction::Right => Vector{x:1, y:0},
        }
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

impl Vector {
    pub fn then(&self, next: &Vector) -> Vector {
        let vector = Vector { x: self.x + next.x, y: self.y + next.y };
        println!("next: [{},{}] (+) self: [{},{}] -> result: [{},{}]", next.x, next.y, self.x, self.y, vector.x, vector.y);
        vector
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {

}

#[derive(Debug,PartialEq)]
pub struct Line {
    pub a: Point,
    pub b: Point,
    pub orientation: Orientation,
    pub range: Vec<isize>,
}

#[derive(Debug,PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Line {
        Line { a, b,
            orientation: Line::orientation(&a, &b),
            range: Line::orientation_range(&a, &b)}
    }

    pub fn orientation(a: &Point, b: &Point) -> Orientation {
        if a.x == b.x {
            Vertical
        } else if a.y == b.y {
            Horizontal
        } else {
            Diagonal
        }
    }

    fn range(a: isize, b: isize) -> Vec<isize> {
        if a > b { b..a+1 } else { a..b+1}.collect()
    }

    pub fn orientation_range(a: &Point, b: &Point) -> Vec<isize> {
        match Line::orientation(a, b) {
            Vertical => Line::range(a.y, b.y),
            Horizontal => Line::range(a.x, b.x),
            Diagonal => vec![],
        }
    }

    pub fn orientation_constant(&self) -> Option<isize> {
        match self.orientation {
            Vertical => Some(self.a.x),
            Horizontal => Some(self.a.y),
            Diagonal => None,
        }
    }

    pub fn orientation_range_contains(&self, b: &Line) -> bool {
        match b.orientation_constant() {
            Some(constant) => self.range.contains(&constant),
            None => false,
        }
    }


    pub fn intersect(&self, b: &Line) -> Option<Point> {
        if self.orientation != b.orientation {
            if self.orientation_range_contains(b) && b.orientation_range_contains(self) {
                if self.orientation == Horizontal {
                    Some(Point { x: b.orientation_constant().unwrap(), y: self.orientation_constant().unwrap() })
                } else if self.orientation == Vertical {
                    Some(Point { x: self.orientation_constant().unwrap(), y: b.orientation_constant().unwrap() })
                } else {
                    None
                }
            } else { None }
        } else { None }
    }
}


//    line a: (0,1)(7,1)
//    line b: (4,0)(4,2)
//    intersect at (4,1)
#[test]
fn two_lines_intersecting_are_identified() {
    let a = Line::new(Point{x: 0, y: 1}, Point{x: 7, y: 1});
    let b = Line::new(Point{x: 4, y: 0}, Point{x: 4, y: 2});
    let expected_intersect = Point { x: 4, y: 1 };

    assert_eq!(b.intersect(&a).unwrap(), expected_intersect);
    assert_eq!(a.intersect(&b).unwrap(), expected_intersect);
}

#[test]
fn orientation_is_returned_correctly() {
    let horizontal_line = Line::new(Point{x: 0, y: 1}, Point{x: 7, y: 1});
    let vertical_line = Line::new(Point{x: 4, y: 0}, Point{x: 4, y: 2});
    let diagonal_line = Line::new(Point{x: 4, y: 0}, Point{x: 6, y: 2});

    assert_eq!(horizontal_line.orientation, Horizontal);
    assert_eq!(vertical_line.orientation, Vertical);
    assert_eq!(diagonal_line.orientation, Diagonal);
}



#[test]
fn range_is_returned_correctly() {
    assert_eq!(Line::range(0,7), vec![0,1,2,3,4,5,6,7]);
    assert_eq!(Line::range(7,0), vec![0,1,2,3,4,5,6,7]);
    assert_eq!(Line::range(1,5), vec![1,2,3,4,5]);
}

#[test]
fn orientation_range_is_returned_correctly() {
    let horizontal_line = Line::new(Point{x: 0, y: 1}, Point{x: 7, y: 1});
    let vertical_line = Line::new(Point{x: 4, y: 0}, Point{x: 4, y: 2});
    let diagonal_line = Line::new(Point{x: 4, y: 0}, Point{x: 6, y: 2});

    assert_eq!(horizontal_line.range, vec![0,1,2,3,4,5,6,7]);
    assert_eq!(vertical_line.range, vec![0,1,2]);
    assert_eq!(diagonal_line.range, vec![]);
}


#[test]
fn correct_constant_returned_for_lines() {
    let horizontal_line = Line::new(Point{x: 0, y: 1}, Point{x: 7, y: 1});
    let vertical_line = Line::new(Point{x: 4, y: 0}, Point{x: 4, y: 2});
    let diagonal_line = Line::new(Point{x: 4, y: 0}, Point{x: 6, y: 2});

    let horizontal_line_orientation_constant = horizontal_line.orientation_constant();
    let vertical_line_orientation_constant = vertical_line.orientation_constant();
    let diagonal_line_orientation_constant = diagonal_line.orientation_constant();

    assert!(horizontal_line_orientation_constant.is_some());
    assert!(vertical_line_orientation_constant.is_some());
    assert!(diagonal_line_orientation_constant.is_none());

    assert_eq!(horizontal_line_orientation_constant.unwrap(), 1);
    assert_eq!(vertical_line_orientation_constant.unwrap(), 4);
}

#[test]
fn crossed_lines_should_contain_a_common_point() {
    let a = Line::new(Point { x: 0, y: 1 }, Point { x: 7, y: 1 } );
    let b = Line::new(Point { x: 4, y: 0 }, Point { x: 4, y: 2 } );

    assert!(a.orientation_range_contains(&b));
}

//    line a: (2,0)(5,0)
//    line b: (7,1)(7,2)
//    no intersect
#[test]
fn two_lines_not_intersecting_are_dismissed() {
    let a = Line::new( Point { x: 2, y: 0 }, Point { x: 5, y: 0 } );
    let b = Line::new( Point { x: 7, y: 1 }, Point { x: 7, y: 2 } );

    assert_eq!(a.intersect(&b), None);
}


#[test]
fn direction_returned_correctly() {
    assert_eq!(Direction::from('U').unwrap(), Direction::Up);
    assert_eq!(Direction::from('D').unwrap(), Direction::Down);
    assert_eq!(Direction::from('L').unwrap(), Direction::Left);
    assert_eq!(Direction::from('R').unwrap(), Direction::Right);
    assert!(Direction::from('x').is_none());
}
