pub trait CustomsAnswers {
    fn count(&self, s: &str, passengers: usize) -> usize;

    fn process(&self, lines: &Vec<String>) -> usize {
        let mut num: usize = 0;
        let mut buf = String::new();
        let mut passengers = 0;

        let mut lines_iter = lines.iter();
        loop {
            match lines_iter.next() {
                Some(line) => {
                    if line.is_empty() {
                        num += self.count(&buf, passengers);
                        buf = String::new();
                        passengers = 0;
                    } else {
                        buf.push_str(line);
                        passengers += 1;
                    }
                },
                None => {
                    num += self.count(&buf, passengers);
                    break;
                },
            }
        }
        num
    }
}