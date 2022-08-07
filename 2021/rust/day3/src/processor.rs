use std::cmp::Ordering;

pub struct Processor {
    pub input: Vec<Vec<u32>>,
    pub bin_len: usize,
    gamma_bits: Vec<u32>,
    pub gamma: Option<u32>,
    epsilon_bits: Vec<u32>,
    pub epsilon: Option<u32>,
    pub power_consumption: Option<u32>,
    pub o2_rating: Option<u32>,
    pub co2_rating: Option<u32>,
    pub life_support_rating: Option<u32>,
}

impl Processor {
    pub fn new(input: Vec<Vec<char>>) -> Self {
        let input_converted: Vec<Vec<u32>> = input
            .iter()
            .map(|i| {
                i.iter()
                    .map(|j| j.to_digit(10).expect("Failed to convert char to u32"))
                    .collect()
            })
            .collect();

        Self {
            input: input_converted,
            bin_len: input[0].len(),
            gamma_bits: Vec::new(),
            gamma: None,
            epsilon_bits: Vec::new(),
            epsilon: None,
            power_consumption: None,
            o2_rating: None,
            co2_rating: None,
            life_support_rating: None,
        }
        .set_rates()
        .set_ratings()
    }

    fn set_rates(mut self) -> Self {
        self.set_rates_mut();
        self
    }

    fn set_rates_mut(&mut self) {
        for i in 0..self.input.first().unwrap().len() {
            match Self::get_most_freq(&self.input, i, 1) {
                Ok(b) => self.gamma_bits.push(b),
                Err(e) => panic!("{}", e),
            }
        }

        for i in 0..self.input.first().unwrap().len() {
            match Self::get_least_freq(&self.input, i, 0) {
                Ok(b) => self.epsilon_bits.push(b),
                Err(e) => panic!("{}", e),
            }
        }

        self.gamma = Some(Self::bits_into_int(&self.gamma_bits));
        self.epsilon = Some(Self::bits_into_int(&self.epsilon_bits));

        self.power_consumption = self.gamma.zip(self.epsilon).map(|o| o.0 * o.1);
    }

    pub fn set_ratings(mut self) -> Self {
        self.set_ratings_mut();
        self
    }

    pub fn set_ratings_mut(&mut self) {
        if self.gamma.is_none() || self.epsilon.is_none() {
            self.set_rates_mut();
        }

        let mut o2_bits = self.input.clone();
        for i in 0..self.input.first().unwrap().len() {
            if o2_bits.len() == 1 {
                break;
            }
            let most_freq = match Self::get_most_freq(&o2_bits, i, 1) {
                Ok(b) => b,
                Err(e) => panic!("{}", e),
            };
            o2_bits.retain(|b| b[i] == most_freq);
        }

        let o2 = Self::bits_into_int(&o2_bits[0]);
        self.o2_rating = Some(o2);

        let mut co2_bits = self.input.clone();
        for i in 0..self.input.first().unwrap().len() {
            if co2_bits.len() == 1 {
                break;
            }
            let least_freq = match Self::get_least_freq(&co2_bits, i, 0) {
                Ok(b) => b,
                Err(e) => panic!("{}", e),
            };
            co2_bits.retain(|b| b[i] == least_freq);
        }

        let co2 = Self::bits_into_int(&co2_bits[0]);
        self.co2_rating = Some(co2);

        self.life_support_rating = Some(co2 * o2);
    }

    fn get_most_freq(input: &[Vec<u32>], idx: usize, default: u32) -> Result<u32, String> {
        if input.first().is_none() {
            return Err(format!("Input was empty {:?}", input));
        }
        if input.first().unwrap().get(idx).is_none() {
            return Err(format!("Invalid index {}", idx));
        }
        let ones = input.iter().filter(|i| i[idx] == 1).count();
        let zeros = input.iter().filter(|i| i[idx] == 0).count();
        match ones.cmp(&zeros) {
            Ordering::Less => Ok(0),
            Ordering::Greater => Ok(1),
            Ordering::Equal => Ok(default),
        }
    }

    fn get_least_freq(input: &[Vec<u32>], idx: usize, default: u32) -> Result<u32, String> {
        if input.first().is_none() {
            return Err(format!("Input was empty {:?}", input));
        }
        if input.first().unwrap().get(idx).is_none() {
            return Err(format!("Invalid index {}", idx));
        }
        let ones = input.iter().filter(|i| i[idx] == 1).count();
        let zeros = input.iter().filter(|i| i[idx] == 0).count();
        match ones.cmp(&zeros) {
            Ordering::Less => Ok(1),
            Ordering::Greater => Ok(0),
            Ordering::Equal => Ok(default),
        }
    }

    fn bits_into_int(bits: &[u32]) -> u32 {
        bits.iter()
            .rev()
            .enumerate()
            .map(|i| i.1 * 2_u32.pow(i.0 as u32))
            .sum()
    }
}
