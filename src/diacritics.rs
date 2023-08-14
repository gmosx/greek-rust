use std::sync::OnceLock;

use crate::util::Rule;

fn diacritics_rules() -> &'static Vec<Rule> {
    static DIACRITICS_RULES: OnceLock<Vec<Rule>> = OnceLock::new();
    DIACRITICS_RULES.get_or_init(|| {
        vec![
            Rule::new("[άἀἁἂἃἄἅἆἇὰάᾀᾁᾂᾃᾄᾅᾆᾇᾰᾱᾲᾳᾴᾶᾷ]", "α"),
            Rule::new("[ΆἈἉἊἋἌἍἎἏᾈᾉᾊᾋᾌᾍᾎᾏᾸᾹᾺΆᾼ]", "Α"),
            Rule::new("[έἐἑἒἓἔἕὲέ]", "ε"),
            Rule::new("[ΈἘἙἚἛἜἝ]", "Ε"),
            Rule::new("[ΉἨἩἪἫἬἭἮἯ]", "Η"),
            Rule::new("[ίἰἱἲἳἴἵὶῖϊ]", "ι"),
            Rule::new("[ΊἶἷἸἹἺἻἼἽἾἿΪ]", "Ι"),
            Rule::new("[ήἠἡἢἣἤἥἦἧῆὴῇ]", "η"),
            Rule::new("[όὀὁὂὃὄὅὸ]", "ο"),
            Rule::new("[ΌὈὉὊὋὌὍ]", "Ο"),
            Rule::new("[ύὐὑὒὓὔὕὖὗ]", "υ"),
            Rule::new("[ΎὙὛὝὟ]", "Υ"),
            Rule::new("[ώὠὡὢὣὤὥὦὧῶ]", "ω"),
            Rule::new("[ΏὨὩὪὫὬὭὮὯ]", "Ω"),
        ]
    })
}

pub fn strip_diacritics(input: &str) -> String {
    diacritics_rules()
        .iter()
        .fold(input.to_string(), |output, rule| rule.apply(&output))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strip_diacritics() {
        assert_eq!(strip_diacritics("Λαϊκά"), "Λαικα");
        assert_eq!(
            strip_diacritics("Αρνάκι άσπρο και παχύ"),
            "Αρνακι ασπρο και παχυ"
        );
    }
}
