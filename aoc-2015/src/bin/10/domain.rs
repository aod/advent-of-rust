/// look-and-say sequence.
pub struct LookAndSay {
    prev: String,
    curr: String,
}

impl LookAndSay {
    pub fn new(input: &str) -> Self {
        LookAndSay {
            prev: String::new(),
            curr: input.to_string(),
        }
    }
}

/// Infinitly generates look-and-say sequences.
impl Iterator for LookAndSay {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev = self.curr.clone();
        self.curr.truncate(0);

        let mut chars = self.prev.chars();
        let mut current_char = chars.next()?;
        let mut amount = 1;

        for c in chars {
            if c == current_char {
                amount += 1;
            } else {
                self.curr.push(std::char::from_digit(amount, 10)?);
                self.curr.push(current_char);

                current_char = c;
                amount = 1;
            }
        }

        self.curr.push(std::char::from_digit(amount, 10)?);
        self.curr.push(current_char);

        Some(self.curr.clone())
    }
}

#[test]
fn it_works() {
    let sequences: Vec<_> = LookAndSay::new("1").take(5).collect();
    assert_eq!(sequences, vec!["11", "21", "1211", "111221", "312211"]);
}
