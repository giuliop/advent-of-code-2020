use std::collections::HashMap;

struct Cups {
    // cup -> next_cup
    next: HashMap<usize, usize>,
    first: usize,
    max: usize,
}

impl Cups {
    fn from_str(s: &str, max: usize) -> Self {
        let digits: Vec<usize> = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        let first = digits[0];
        let last = digits[digits.len() - 1];

        let mut next: HashMap<usize, usize> =
            digits.windows(2).map(|w| (w[0], w[1])).collect();
        next.insert(last, first);

        if max > digits.len() {
            next.insert(last, 10);
            for i in 10..max {
                next.insert(i, i + 1);
            }
            next.insert(max, first);
        }

        Self { next, first, max }
    }

    fn part_1(&self) -> String {
        let mut s = String::new();
        let mut next = self.next[&1];
        while next != 1 {
            s.push_str(&next.to_string());
            next = self.next[&next];
        }
        s
    }

    fn part_2(self) -> usize {
        self.next[&1] * self.next[&self.next[&1]]
    }
    fn do_rounds(mut self, rounds: usize) -> Self {
        let mut current = self.first;

        for _r in 0..rounds {
            let picked: [usize; 3] = [
                self.next[&current],
                self.next[&self.next[&current]],
                self.next[&self.next[&self.next[&current]]],
            ];
            self.next.insert(current, self.next[&picked[2]]);

            let mut dest = current - 1;

            loop {
                if dest == 0 {
                    dest = self.max;
                }
                if picked.iter().all(|&n| n != dest) {
                    break;
                } else {
                    dest -= 1;
                }
            }
            self.next.insert(picked[2], self.next[&dest]);
            self.next.insert(dest, picked[0]);

            current = self.next[&current];
        }
        self
    }
}

pub fn a() -> String {
    let cups = Cups::from_str("784235916", 9);
    cups.do_rounds(100).part_1().to_string()
}

pub fn b() -> String {
    let cups = Cups::from_str("784235916", 1000000);
    cups.do_rounds(10000000).part_2().to_string()
}
