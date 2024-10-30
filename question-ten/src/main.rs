use number_to_words::number_to_words;

// Number of vowels which are always in the sentence
const VOWELS: u32 = 31;
// Number of words which are always in the sentence
const WORDS: u32 = 17;

#[derive(Debug)]
struct Sentence {
    words: String,
    vowels: String,
}

impl Sentence {
    pub fn try_from(words: u32, vowels: u32) -> Option<Self> {
        let words_str = &number_to_words(words, false);
        let vowels_str = &number_to_words(vowels, false);

        if Self::vowels(words_str) + Self::vowels(vowels_str) + VOWELS == vowels
            && Self::words(words_str) + Self::words(vowels_str) + WORDS == words
        {
            println!(
                "\n{} Words",
                Self::words(words_str) + Self::words(vowels_str) + WORDS
            );
            println!(
                "{} Vowels",
                Self::vowels(words_str) + Self::vowels(vowels_str) + VOWELS
            );
            Some(Self {
                words: words_str.to_string(),
                vowels: vowels_str.to_string(),
            })
        } else {
            None
        }
    }

    fn vowels(str: &str) -> u32 {
        str.chars()
            .map(|c| u32::from(['a', 'e', 'i', 'o', 'u'].contains(&c)))
            .sum::<u32>()
    }

    fn words(str: &str) -> u32 {
        str.chars()
            .map(|c| match c {
                '-' => ' ',
                _ => c,
            })
            .collect::<String>()
            .split_whitespace()
            .count() as u32
    }
}

fn main() {
    for i in 10..100 {
        for j in 10..100 {
            Sentence::try_from(i, j);
        }
    }
}
