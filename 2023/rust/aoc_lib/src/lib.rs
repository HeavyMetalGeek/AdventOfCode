pub struct NumWord {
    pub word: &'static str,
    pub value: u32,
    pub len: usize,
}

impl NumWord {
    pub const ONE: Self = Self::new("one", 1);
    pub const TWO: Self = Self::new("two", 2);
    pub const THREE: Self = Self::new("three", 3);
    pub const FOUR: Self = Self::new("four", 4);
    pub const FIVE: Self = Self::new("five", 5);
    pub const SIX: Self = Self::new("six", 6);
    pub const SEVEN: Self = Self::new("seven", 7);
    pub const EIGHT: Self = Self::new("eight", 8);
    pub const NINE: Self = Self::new("nine", 9);
    pub const fn new(word: &'static str, value: u32) -> Self {
        return Self {
            word,
            value,
            len: word.len(),
        };
    }

    pub fn match_to_chars(self, chars: &[char]) -> Option<Self> {
        if chars.len() < self.len {
            return None;
        }
        let word: String = chars[0..self.len].iter().collect();
        if word == self.word {
            return Some(self);
        }
        return None;
    }

    pub fn from_chars(chars: &[char]) -> Option<Self> {
        if chars.len() < 2 {
            return None;
        }
        let first_two: [char; 2] = [chars[0], chars[1]];
        return match first_two {
            ['o', 'n'] => Self::ONE.match_to_chars(chars),
            ['t', 'w'] => Self::TWO.match_to_chars(chars),
            ['t', 'h'] => Self::THREE.match_to_chars(chars),
            ['f', 'o'] => Self::FOUR.match_to_chars(chars),
            ['f', 'i'] => Self::FIVE.match_to_chars(chars),
            ['s', 'i'] => Self::SIX.match_to_chars(chars),
            ['s', 'e'] => Self::SEVEN.match_to_chars(chars),
            ['e', 'i'] => Self::EIGHT.match_to_chars(chars),
            ['n', 'i'] => Self::NINE.match_to_chars(chars),
            _ => None,
        };
    }
}

pub const INPUT_DAY1: &'static str = include_str!("input1d1.txt");
pub const INPUT_DAY2: &'static str = include_str!("input1d2.txt");
pub const INPUT_DAY3: &'static str = include_str!("input1d3.txt");
