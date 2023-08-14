//! Time-related utilities.

// pub static GREEK_MONTHS_MAP_SHORT: Lazy<HashMap<&str, u32>> = Lazy::new(|| {
//     let mut m = HashMap::new();

//     m.insert("Ιαν", 1);
//     m.insert("Φεβ", 2);
//     // ...

//     m
// });

pub fn month_from_greek_name_short(greek_name: &str) -> Option<u32> {
    match greek_name {
        "Ιαν" => Some(1),
        "Φεβ" => Some(2),
        "Μαρ" => Some(3),
        "Απρ" => Some(4),
        "Μαι" => Some(5),
        "Ιουν" => Some(6),
        "Ιουλ" => Some(7),
        "Αυγ" => Some(8),
        "Σεπ" => Some(9),
        "Οκτ" => Some(10),
        "Νοε" => Some(11),
        "Δεκ" => Some(12),
        _ => None,
    }
}
