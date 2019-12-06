use advent_of_code_2019_rust::helpers::files::read_file;
use advent_of_code_2019_rust::helpers::converters::to_usize;
use advent_of_code_2019_rust::day2::compute::compute;

fn main() {
    let target: usize = 19690720;
    (0..100).for_each(|noun| {
        (0..100).for_each( |verb| {
            let mut data: Vec<usize> = read_file("./data/day2/input.txt").split(",").map(|s| to_usize(s)).collect();
            data[1] = noun;
            data[2] = verb;
            if compute(data)[0] == target {
                println!("{}", (noun * 100) + verb);
            }
        })
    })

}