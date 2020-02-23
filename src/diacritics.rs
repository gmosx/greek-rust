use lazy_static::lazy_static;
use regex::Regex;

pub fn strip_diacritics(text: &str) -> String {
    let mut stripped = text.to_string();

    lazy_static! {
        static ref STRIP_DIACRITICS_RULES: [(Regex, &'static str); 14] = [
            (Regex::new("[άἀἁἂἃἄἅἆἇὰάᾀᾁᾂᾃᾄᾅᾆᾇᾰᾱᾲᾳᾴᾶᾷ]").unwrap(), "α"),
            (Regex::new("[ΆἈἉἊἋἌἍἎἏᾈᾉᾊᾋᾌᾍᾎᾏᾸᾹᾺΆᾼ]").unwrap(), "Α"),
            (Regex::new("[έἐἑἒἓἔἕὲέ]").unwrap(), "ε"),
            (Regex::new("[ΈἘἙἚἛἜἝ]").unwrap(), "Ε"),
            (Regex::new("[ΉἨἩἪἫἬἭἮἯ]").unwrap(), "Η"),
            (Regex::new("[ίἰἱἲἳἴἵὶῖϊ]").unwrap(), "ι"),
            (Regex::new("[ΊἶἷἸἹἺἻἼἽἾἿΪ]").unwrap(), "Ι"),
            (Regex::new("[ήἠἡἢἣἤἥἦἧῆὴῇ]").unwrap(), "η"),
            (Regex::new("[όὀὁὂὃὄὅὸ]").unwrap(), "ο"),
            (Regex::new("[ΌὈὉὊὋὌὍ]").unwrap(), "Ο"),
            (Regex::new("[ύὐὑὒὓὔὕὖὗ]").unwrap(), "υ"),
            (Regex::new("[ΎὙὛὝὟ]").unwrap(), "Υ"),
            (Regex::new("[ώὠὡὢὣὤὥὦὧῶ]").unwrap(), "ω"),
            (Regex::new("[ΏὨὩὪὫὬὭὮὯ]").unwrap(), "Ω"),
        ];
    }

    for (re, replacement) in STRIP_DIACRITICS_RULES.iter() {
        stripped = re.replace_all(&stripped, *replacement).to_string();
    }

    stripped
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
