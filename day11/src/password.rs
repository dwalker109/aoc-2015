use std::fmt::Display;

#[derive(Debug)]
pub struct Password {
    raw: [u8; 8],
}

impl Password {
    pub fn from(input: &str) -> Password {
        let mut raw: [u8; 8] = [0; 8];
        for (i, &digit) in input.as_bytes().iter().enumerate() {
            raw[i] = digit;
        }

        Password { raw }
    }

    pub fn increment(&mut self) {
        for i in (0..8).rev() {
            self.raw[i] += 1;
            if self.raw[i] > b'z' {
                self.raw[i] = b'a';
            } else {
                break;
            }
        }
    }

    pub fn has_increasing_straight(&self) -> bool {
        for digit in self.raw.windows(3) {
            let a = digit[0];
            let b = digit[1];
            let c = digit[2];

            if a + 1 == b && b + 1 == c {
                return true;
            }
        }

        false
    }

    pub fn contains_no_mistakeables(&self) -> bool {
        !(self.raw.contains(&b'i') || self.raw.contains(&b'o') || self.raw.contains(&b'l'))
    }

    pub fn has_two_pairs(&self) -> bool {
        let mut count: u8 = 0;
        let mut windows = self.raw.windows(2);
        while let Some(digit) = windows.next() {
            if digit[0] == digit[1] {
                count += 1;
                if count > 1 {
                    return true;
                }
                windows.next(); // consume next to avoid overlaps
            }
        }

        false
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            self.raw[0] as char,
            self.raw[1] as char,
            self.raw[2] as char,
            self.raw[3] as char,
            self.raw[4] as char,
            self.raw[5] as char,
            self.raw[6] as char,
            self.raw[7] as char,
        )
    }
}

impl PartialEq<&str> for Password {
    fn eq(&self, other: &&str) -> bool {
        self.raw == other.as_bytes()
    }
}
