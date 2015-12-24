extern crate rand;

use rand::{thread_rng, Rng};

// Default adjectives
pub const DEFAULT_ADJECTIVES: &'static [&'static str] = &[
    "autumn", "hidden", "bitter", "misty", "silent", "empty", "dry", "dark",
    "summer", "icy", "delicate", "quiet", "white", "cool", "spring", "winter",
    "patient", "twilight", "dawn", "crimson", "wispy", "weathered", "blue",
    "billowing", "broken", "cold", "damp", "falling", "frosty", "green",
    "long", "late", "lingering", "bold", "little", "morning", "muddy", "old",
    "red", "rough", "still", "small", "sparkling", "throbbing", "shy",
    "wandering", "withered", "wild", "black", "young", "holy", "solitary",
    "fragrant", "aged", "snowy", "proud", "floral", "restless", "divine",
    "polished", "ancient", "purple", "lively", "nameless", "lucky", "odd", "tiny",
    "free", "dry", "yellow", "orange", "gentle", "tight", "super", "royal", "broad",
    "steep", "flat", "square", "round", "mute", "noisy", "hushy", "raspy", "soft",
    "shrill", "rapid", "sweet", "curly", "calm", "jolly", "fancy", "plain", "shinny"
];

// Default nouns
pub const DEFAULT_NOUNS: &'static [&'static str] = &[
    "waterfall", "river", "breeze", "moon", "rain", "wind", "sea", "morning",
    "snow", "lake", "sunset", "pine", "shadow", "leaf", "dawn", "glitter",
    "forest", "hill", "cloud", "meadow", "sun", "glade", "bird", "brook",
    "butterfly", "bush", "dew", "dust", "field", "fire", "flower", "firefly",
    "feather", "grass", "haze", "mountain", "night", "pond", "darkness",
    "snowflake", "silence", "sound", "sky", "shape", "surf", "thunder",
    "violet", "water", "wildflower", "wave", "water", "resonance", "sun",
    "wood", "dream", "cherry", "tree", "fog", "frost", "voice", "paper",
    "frog", "smoke", "star", "atom", "band", "bar", "base", "block", "boat",
    "term", "credit", "art", "fashion", "truth", "disk", "math", "unit", "cell",
    "scene", "heart", "recipe", "union", "limit", "bread", "toast", "bonus",
    "lab", "mud", "mode", "poetry", "tooth", "hall", "king", "queen", "lion", "tiger",
    "penguin", "kiwi", "cake", "mouse", "rice", "coke", "hola", "salad", "hat"
];

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
            adjectives: DEFAULT_ADJECTIVES,
            nouns: DEFAULT_NOUNS,
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
        if self.adjectives.len() > 0{
            adjective = self.adjectives[rng.gen_range(0, self.adjectives.len())];
        } else {
            adjective = "";
        }

        if self.nouns.len() > 0 {
            noun = self.nouns[rng.gen_range(0, self.nouns.len())];
        } else {
            noun = "";
        }
        
        // create token
        let mut token = String::with_capacity(self.token_length);
        let count = tokens.chars().count();
        
        if count > 0 {
            for _ in 0..self.token_length {
                let index = rng.gen_range(0, count);
                token.push(tokens.chars().nth(index).unwrap());
            }
        }

        // create and return result
        let mut parts = vec![adjective, noun, &token];
        parts.retain(|s: &&str| s.len() > 0);
        parts.join(self.delimiter)
    }
}
