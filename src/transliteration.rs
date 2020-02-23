use crate::diacritics::strip_diacritics;
use crate::util::Rule;
use lazy_static::lazy_static;

pub fn to_greeklish(input: &str) -> String {
    lazy_static! {
        static ref TO_GREEKLISH_RULES: [Rule; 77] = [
            Rule::new("ΓΧ", "GX"),
            Rule::new("γχ", "gx"),
            Rule::new("ΤΘ", "T8"),
            Rule::new("τθ", "t8"),
            Rule::new("(θη|Θη),", "8h"),
            Rule::new("ΘΗ", "8H"),
            Rule::new("αυ", "au"),
            Rule::new("Αυ", "Au"),
            Rule::new("ΑΥ", "AY"),
            Rule::new("ευ", "eu"),
            Rule::new("εύ", "eu"),
            Rule::new("εϋ", "ey"),
            Rule::new("εΰ", "ey"),
            Rule::new("Ευ", "Eu"),
            Rule::new("Εύ", "Eu"),
            Rule::new("Εϋ", "Ey"),
            Rule::new("Εΰ", "Ey"),
            Rule::new("ΕΥ", "EY"),
            Rule::new("ου", "ou"),
            Rule::new("ού", "ou"),
            Rule::new("οϋ", "oy"),
            Rule::new("οΰ", "oy"),
            Rule::new("Ου", "Ou"),
            Rule::new("Ού", "Ou"),
            Rule::new("Οϋ", "Oy"),
            Rule::new("Οΰ", "Oy"),
            Rule::new("ΟΥ", "OY"),
            Rule::new("Α", "A"),
            Rule::new("α", "a"),
            // Rule::new("ά", "a"),
            // Rule::new("Ά", "A"),
            Rule::new("Β", "B"),
            Rule::new("β", "b"),
            Rule::new("Γ", "G"),
            Rule::new("γ", "g"),
            Rule::new("Δ", "D"),
            Rule::new("δ", "d"),
            Rule::new("Ε", "E"),
            Rule::new("ε", "e"),
            // Rule::new("έ", "e"),
            // Rule::new("Έ", "E"),
            Rule::new("Ζ", "Z"),
            Rule::new("ζ", "z"),
            Rule::new("Η", "H"),
            Rule::new("η", "h"),
            // Rule::new("ή", "h"),
            // Rule::new("Ή", "H"),
            Rule::new("Θ", "TH"),
            Rule::new("θ", "th"),
            Rule::new("Ι", "I"),
            // Rule::new("Ϊ", "I"),
            Rule::new("ι", "i"),
            // Rule::new("ί", "i"),
            // Rule::new("ΐ", "i"),
            // Rule::new("ϊ", "i"),
            // Rule::new("Ί", "I"),
            Rule::new("Κ", "K"),
            Rule::new("κ", "k"),
            Rule::new("Λ", "L"),
            Rule::new("λ", "l"),
            Rule::new("Μ", "M"),
            Rule::new("μ", "m"),
            Rule::new("Ν", "N"),
            Rule::new("ν", "n"),
            Rule::new("Ξ", "KS"),
            Rule::new("ξ", "ks"),
            Rule::new("Ο", "O"),
            Rule::new("ο", "o"),
            // Rule::new("Ό", "O"),
            // Rule::new("ό", "o"),
            Rule::new("Π", "P"),
            Rule::new("π", "p"),
            Rule::new("Ρ", "R"),
            Rule::new("ρ", "r"),
            Rule::new("Σ", "S"),
            Rule::new("σ", "s"),
            Rule::new("Τ", "T"),
            Rule::new("τ", "t"),
            Rule::new("Υ", "Y"),
            // Rule::new("Ύ", "Y"),
            // Rule::new("Ϋ", "Y"),
            // Rule::new("ΰ", "y"),
            // Rule::new("ύ", "y"),
            // Rule::new("ϋ", "y"),
            Rule::new("υ", "y"),
            Rule::new("Φ", "F"),
            Rule::new("φ", "f"),
            Rule::new("Χ", "X"),
            Rule::new("χ", "x"),
            Rule::new("Ψ", "Ps"),
            Rule::new("ψ", "ps"),
            Rule::new("Ω", "w"),
            Rule::new("ω", "w"),
            // Rule::new("Ώ", "w"),
            // Rule::new("ώ", "w"),
            Rule::new("ς", "s"),
            Rule::new(";", "?"),
        ];
    }

    TO_GREEKLISH_RULES
        .iter()
        .fold(strip_diacritics(&input), |output, rule| rule.apply(&output))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_greeklish() {
        assert_eq!(to_greeklish(""), "");
        assert_eq!(to_greeklish("Ευτυχία"), "Eutyxia");
        assert_eq!(to_greeklish("Λαϊκά"), "Laika");
        assert_eq!(
            to_greeklish("Αρνάκι άσπρο και παχύ"),
            "Arnaki aspro kai paxy"
        );
        assert_eq!(to_greeklish("Γιώργος Μοσχοβίτης"), "Giwrgos Mosxobiths");
    }
}
