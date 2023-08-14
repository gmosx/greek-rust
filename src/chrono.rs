//! Time-related utilities.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn month_from_greekname() {
        assert_eq!(month_from_greek_name_short("Ιαν").unwrap(), 1);
        assert_eq!(month_from_greek_name_short("Οκτ").unwrap(), 10);
        assert_eq!(month_from_greek_name_short("Δεκ").unwrap(), 12);
        assert_eq!(month_from_greek_name_short("Λάθος"), None);
    }
}
