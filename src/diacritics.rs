use regex::Regex;

// var stripDiacriticsRules = []rule{
//     {regexp.MustCompile("[άἀἁἂἃἄἅἆἇὰάᾀᾁᾂᾃᾄᾅᾆᾇᾰᾱᾲᾳᾴᾶᾷ]"), "α"},
//     {regexp.MustCompile("[ΆἈἉἊἋἌἍἎἏᾈᾉᾊᾋᾌᾍᾎᾏᾸᾹᾺΆᾼ]"), "Α"},
//     {regexp.MustCompile("[έἐἑἒἓἔἕὲέ]"), "ε"},
//     {regexp.MustCompile("[ΈἘἙἚἛἜἝ]"), "Ε"},
//     {regexp.MustCompile("[ήἠἡἢἣἤἥἦἧῆὴῇ]"), "η"},
//     {regexp.MustCompile("[ΉἨἩἪἫἬἭἮἯ]"), "Η"},
//     {regexp.MustCompile("[ίἰἱἲἳἴἵὶῖϊ]"), "ι"},
//     {regexp.MustCompile("[ΊἶἷἸἹἺἻἼἽἾἿΪ]"), "Ι"},
//     {regexp.MustCompile("[όὀὁὂὃὄὅὸ]"), "ο"},
//     {regexp.MustCompile("[ΌὈὉὊὋὌὍ]"), "Ο"},
//     {regexp.MustCompile("[ύὐὑὒὓὔὕὖὗ]"), "υ"},
//     {regexp.MustCompile("[ΎὙὛὝὟ]"), "Υ"},
//     {regexp.MustCompile("[ώὠὡὢὣὤὥὦὧῶ]"), "ω"},
//     {regexp.MustCompile("[ΏὨὩὪὫὬὭὮὯ]"), "Ω"},
// }

// const strip_diacritics_rules: [(Regex, &str)] = [
//     (Regex::new(["άἀἁἂἃἄἅἆἇὰάᾀᾁᾂᾃᾄᾅᾆᾇᾰᾱᾲᾳᾴᾶᾷ]"), "α"),
// ];

pub fn strip_diacritics(text: &str) -> String {
    let strip_diacritics_rules = vec![
        (Regex::new("[άἀἁἂἃἄἅἆἇὰάᾀᾁᾂᾃᾄᾅᾆᾇᾰᾱᾲᾳᾴᾶᾷ]").unwrap(), "α"),
        (Regex::new("[ΆἈἉἊἋἌἍἎἏᾈᾉᾊᾋᾌᾍᾎᾏᾸᾹᾺΆᾼ]").unwrap(), "Α"),
        (Regex::new("[ίἰἱἲἳἴἵὶῖϊ]").unwrap(), "ι"),
    ];

    let mut stripped = text.to_string();

    for (re, replacement) in strip_diacritics_rules {
        stripped = re.replace_all(&stripped, replacement).to_string();
    }

    stripped
}

// {"Αρνάκι άσπρο και παχύ", "Αρνακι ασπρο και παχυ"},

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strip_diacritics() {
        assert_eq!(strip_diacritics("Λαϊκά"), "Λαικα");
    }
}
