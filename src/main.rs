struct Solution;

enum Kind {
    Vowel,
    Consonant,
    Other,
}

impl Solution {
    pub fn vowel_consonant_score(s: String) -> i32 {
        let mut consonants = 0;
        let mut vowels = 0;
        for ch in s.chars() {
            match Self::check(ch) {
                Kind::Vowel => vowels += 1,
                Kind::Consonant => consonants += 1,
                _ => (),
            };
        }

        let score = match consonants > 0 {
            true => (vowels as f64) / (consonants as f64),
            _ => 0.0f64,
        };

        score.floor() as i32
    }

    fn check(ch: char) -> Kind {
        match ch {
            '0'..='9' | ' ' => Kind::Other,
            'a' | 'e' | 'i' | 'o' | 'u' => Kind::Vowel,
            _ => Kind::Consonant,
        }
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "cooear".to_string(),
        },
        Input {
            s: "axeyizou".to_string(),
        },
        Input {
            s: "au 123".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::vowel_consonant_score(input.s);
        println!("{:?}", result);
    }
}
