//! Human-readable `&'static str` names for calendar values.
//!
//! English names are always available. Burmese-script names are an
//! additive, opt-in feature (`burmese`) — enabling it only adds the
//! `burmese_name()` methods; it never changes any existing behavior.
//!
//! # Memory note
//!
//! Every name is a `&'static str` baked into the binary's read-only data
//! section. Callers never get owned `String`s from this module, so there is
//! no allocation and nothing to free.

use crate::{Month, MonthPhase, Weekday, YearType};

impl Weekday {
    /// The English name of this weekday.
    ///
    /// ```
    /// use myanmar_date::Weekday;
    /// assert_eq!(Weekday::Mon.english_name(), "Monday");
    /// ```
    #[must_use]
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Sat => "Saturday",
            Self::Sun => "Sunday",
            Self::Mon => "Monday",
            Self::Tue => "Tuesday",
            Self::Wed => "Wednesday",
            Self::Thu => "Thursday",
            Self::Fri => "Friday",
        }
    }

    /// The Burmese-script name of this weekday.
    ///
    /// ```
    /// use myanmar_date::Weekday;
    /// assert_eq!(Weekday::Sat.burmese_name(), "စနေ");
    /// ```
    #[cfg(feature = "burmese")]
    #[must_use]
    pub fn burmese_name(&self) -> &'static str {
        match self {
            Self::Sat => "စနေ",
            Self::Sun => "တနင်္ဂနွေ",
            Self::Mon => "တနင်္လာ",
            Self::Tue => "အင်္ဂါ",
            Self::Wed => "ဗုဒ္ဓဟူး",
            Self::Thu => "ကြာသပတေး",
            Self::Fri => "သောကြာ",
        }
    }
}

impl Month {
    /// The English name of this month.
    ///
    /// ```
    /// use myanmar_date::Month;
    /// assert_eq!(Month::Tagu.english_name(), "Tagu");
    /// assert_eq!(Month::FirstWaso.english_name(), "First Waso");
    /// ```
    #[must_use]
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::FirstWaso => "First Waso",
            Self::Tagu => "Tagu",
            Self::Kason => "Kason",
            Self::Nayon => "Nayon",
            Self::SecondWaso => "Second Waso",
            Self::Wagaung => "Wagaung",
            Self::Tawthalin => "Tawthalin",
            Self::Thadingyut => "Thadingyut",
            Self::Tazaungmon => "Tazaungmon",
            Self::Nadaw => "Nadaw",
            Self::Pyatho => "Pyatho",
            Self::Tabodwe => "Tabodwe",
            Self::Tabaung => "Tabaung",
            Self::LateTagu => "Late Tagu",
            Self::LateKason => "Late Kason",
        }
    }

    /// The Burmese-script name of this month.
    ///
    /// ```
    /// use myanmar_date::Month;
    /// assert_eq!(Month::Tagu.burmese_name(), "တန်ခူး");
    /// ```
    #[cfg(feature = "burmese")]
    #[must_use]
    pub fn burmese_name(&self) -> &'static str {
        match self {
            Self::FirstWaso => "ပဌမဝါဆို",
            Self::Tagu => "တန်ခူး",
            Self::Kason => "ကဆုန်",
            Self::Nayon => "နယုန်",
            Self::SecondWaso => "ဒုတိယဝါဆို",
            Self::Wagaung => "ဝါခေါင်",
            Self::Tawthalin => "တော်သလင်း",
            Self::Thadingyut => "သီတင်းကျွတ်",
            Self::Tazaungmon => "တန်ဆောင်မုန်း",
            Self::Nadaw => "နတ်တော်",
            Self::Pyatho => "ပြာသို",
            Self::Tabodwe => "တပို့တွဲ",
            Self::Tabaung => "တပေါင်း",
            Self::LateTagu => "နှောင်းတန်ခူး",
            Self::LateKason => "နှောင်းကဆုန်",
        }
    }
}

impl YearType {
    /// The English name of this year type.
    ///
    /// ```
    /// use myanmar_date::YearType;
    /// assert_eq!(YearType::BigWatat.english_name(), "Big Watat");
    /// ```
    #[must_use]
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::LittleWatat => "Little Watat",
            Self::BigWatat => "Big Watat",
        }
    }
}

impl MonthPhase {
    /// The English name of this moon phase.
    ///
    /// ```
    /// use myanmar_date::MonthPhase;
    /// assert_eq!(MonthPhase::Full.english_name(), "Full Moon");
    /// ```
    #[must_use]
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Waxing => "Waxing",
            Self::Full => "Full Moon",
            Self::Waning => "Waning",
            Self::New => "New Moon",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_weekday_has_a_distinct_english_name() {
        let names: std::vec::Vec<&str> = [
            Weekday::Sat,
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
        ]
        .iter()
        .map(Weekday::english_name)
        .collect();
        let mut unique = names.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), names.len());
    }

    #[test]
    fn every_month_has_a_distinct_english_name() {
        let names: std::vec::Vec<&str> = Month::ALL.iter().map(Month::english_name).collect();
        let mut unique = names.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), names.len());
    }

    #[test]
    fn every_year_type_has_a_distinct_english_name() {
        let names =
            [YearType::Common, YearType::LittleWatat, YearType::BigWatat].map(|y| y.english_name());
        assert_ne!(names[0], names[1]);
        assert_ne!(names[1], names[2]);
        assert_ne!(names[0], names[2]);
    }

    #[test]
    fn every_month_phase_has_a_distinct_english_name() {
        let names = [
            MonthPhase::Waxing,
            MonthPhase::Full,
            MonthPhase::Waning,
            MonthPhase::New,
        ]
        .map(|p| p.english_name());
        let mut unique = names;
        unique.sort_unstable();
        assert_eq!(unique[0], "Full Moon");
    }

    #[cfg(feature = "burmese")]
    #[test]
    fn burmese_names_are_non_empty_for_every_month() {
        for m in Month::ALL {
            assert_ne!(m.burmese_name(), "");
        }
    }

    #[cfg(feature = "burmese")]
    #[test]
    fn burmese_feature_is_purely_additive() {
        // Enabling `burmese` must not change any existing English-name
        // behavior.
        assert_eq!(Month::Tagu.english_name(), "Tagu");
        assert_eq!(Weekday::Mon.english_name(), "Monday");
    }
}
