use rand::{thread_rng, Rng};

mod default_adjectives;
mod default_nouns;

/// The `Haikunator` type
/// Holds settings and data that will be used when `haikunate` is called.
///
/// # Examples
///
/// ```
/// use haikunator::Haikunator;
///
/// let h = Haikunator {
///     adjectives: &["flying", "bubbly"],
///     nouns: &["bat", "soda"],
///     delimiter: "-",
///     token_length: 8,
///     token_hex: false,
///     token_chars: "0123456789忠犬ハチ公"
/// };
///
/// ```
///
/// **Note**: If `token_hex` is true, the value of `token_chars` is ignored.
#[derive(Debug)]
pub struct Haikunator<'a> {
    pub adjectives: &'a [&'a str],
    pub nouns: &'a [&'a str],
    pub delimiter: &'a str,
    pub token_length: usize,
    pub token_hex: bool,
    pub token_chars: &'a str,
}

impl<'a> Default for Haikunator<'a> {
    /// Constructs a new Haikunator with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use haikunator::Haikunator;
    ///
    /// let h = Haikunator::default();
    /// ```
    fn default() -> Self {
        Haikunator {
            adjectives: default_adjectives::DEFAULT_ADJECTIVES,
            nouns: default_nouns::DEFAULT_NOUNS,
            delimiter: "-",
            token_length: 4,
            token_hex: false,
            token_chars: "0123456789",
        }
    }
}

impl<'a> Haikunator<'a> {
    /// Generates random heroku-like short names using a combination
    // of adjective, noun, and the delimiter.
    ///
    /// # Examples
    ///
    /// ```
    /// use haikunator::Haikunator;
    ///
    /// let h = Haikunator::default();
    /// println!("{:?}", h.haikunate());
    /// ```
    pub fn haikunate(&self) -> String {
        // determine tokens to use
        let tokens;

        if self.token_hex {
            tokens = "0123456789abcdef";
        } else {
            tokens = self.token_chars;
        }

        let mut rng = thread_rng();

        // pick adjective and noun
        let adjective;
        let noun;

        // avoid panic when low >= high in gen_range
        if !self.adjectives.is_empty() {
            adjective = self.adjectives[rng.gen_range(0..self.adjectives.len())];
        } else {
            adjective = "";
        }

        if !self.nouns.is_empty() {
            noun = self.nouns[rng.gen_range(0..self.nouns.len())];
        } else {
            noun = "";
        }

        // create token
        let mut token = String::with_capacity(self.token_length);
        let count = tokens.chars().count();

        if count > 0 {
            for _ in 0..self.token_length {
                let index = rng.gen_range(0..count);
                token.push(tokens.chars().nth(index).unwrap());
            }
        }

        // create and return result
        let mut parts = vec![adjective, noun, &token];
        parts.retain(|s: &&str| !s.is_empty());
        parts.join(self.delimiter)
    }
}
