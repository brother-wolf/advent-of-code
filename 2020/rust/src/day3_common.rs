pub fn toboggan(down: usize, right: usize, tree_lines: &Vec<String>) -> usize {
    let mut x = 0 - right as isize;
    tree_lines.iter().step_by(down).map(|tree_line| {
        x += right as isize;
        if x >= (tree_line.len() as isize) { x -= tree_line.len() as isize }
        if x < 0 { panic!(format!("x is : {}\n\ttree line: {}", x, tree_line)) }
        if tree_line.chars().nth(x as usize).unwrap() == '#' { 1 } else { 0 }
    }).sum::<usize>()
}