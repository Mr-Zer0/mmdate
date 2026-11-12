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

use crate::{
    AstroDay, Mahabote, Month, MonthPhase, NagaDirection, Nakhat, Pyathada, Sabbath, Weekday,
    YearName, YearType,
};

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
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
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
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
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

    /// The Burmese-script name of this year type.
    ///
    /// Note: unlike the other `burmese_name()` methods, these terms are the
    /// standard Burmese calendar vocabulary rather than entries from mmcal's
    /// `ceMmTranslate` catalog (which has no year-type terms).
    ///
    /// ```
    /// use myanmar_date::YearType;
    /// assert_eq!(YearType::Common.burmese_name(), "သာမန်နှစ်");
    /// ```
    #[cfg(feature = "burmese")]
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
    #[must_use]
    pub fn burmese_name(&self) -> &'static str {
        match self {
            Self::Common => "သာမန်နှစ်",
            Self::LittleWatat => "ဝါငယ်ထပ်နှစ်",
            Self::BigWatat => "ဝါကြီးထပ်နှစ်",
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

    /// The Burmese-script name of this moon phase.
    ///
    /// ```
    /// use myanmar_date::MonthPhase;
    /// assert_eq!(MonthPhase::Waxing.burmese_name(), "လဆန်း");
    /// ```
    #[cfg(feature = "burmese")]
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
    #[must_use]
    pub fn burmese_name(&self) -> &'static str {
        match self {
            Self::Waxing => "လဆန်း",
            Self::Full => "လပြည့်",
            Self::Waning => "လဆုတ်",
            Self::New => "လကွယ်",
        }
    }
}

impl Sabbath {
    /// The English name of this sabbath state.
    ///
    /// ```
    /// use myanmar_date::Sabbath;
    /// assert_eq!(Sabbath::Sabbath.english_name(), "Sabbath");
    /// assert_eq!(Sabbath::SabbathEve.english_name(), "Sabbath Eve");
    /// ```
    #[must_use]
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Sabbath => "Sabbath",
            Self::SabbathEve => "Sabbath Eve",
        }
    }

    /// The Burmese-script name of this sabbath state.
    ///
    /// ```
    /// use myanmar_date::Sabbath;
    /// assert_eq!(Sabbath::Sabbath.burmese_name(), "ဥပုသ်");
    /// assert_eq!(Sabbath::SabbathEve.burmese_name(), "အဖိတ်");
    /// ```
    #[cfg(feature = "burmese")]
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
    #[must_use]
    pub fn burmese_name(&self) -> &'static str {
        match self {
            Self::Sabbath => "ဥပုသ်",
            Self::SabbathEve => "အဖိတ်",
        }
    }
}

impl Pyathada {
    /// The English name of this pyathada state.
    ///
    /// ```
    /// use myanmar_date::Pyathada;
    /// assert_eq!(Pyathada::Pyathada.english_name(), "Pyathada");
    /// assert_eq!(Pyathada::Afternoon.english_name(), "Afternoon Pyathada");
    /// ```
    #[must_use]
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Pyathada => "Pyathada",
            Self::Afternoon => "Afternoon Pyathada",
        }
    }

    /// The Burmese-script name of this pyathada state.
    ///
    /// The afternoon variant composes mmcal's `Afternoon` (`မွန်းလွဲ`) and
    /// `Pyathada` (`ပြဿဒါး`) catalog terms.
    ///
    /// ```
    /// use myanmar_date::Pyathada;
    /// assert_eq!(Pyathada::Pyathada.burmese_name(), "ပြဿဒါး");
    /// ```
    #[cfg(feature = "burmese")]
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
    #[must_use]
    pub fn burmese_name(&self) -> &'static str {
        match self {
            Self::Pyathada => "ပြဿဒါး",
            Self::Afternoon => "မွန်းလွဲ ပြဿဒါး",
        }
    }
}

impl NagaDirection {
    /// The English name of this direction.
    ///
    /// ```
    /// use myanmar_date::NagaDirection;
    /// assert_eq!(NagaDirection::South.english_name(), "South");
    /// ```
    #[must_use]
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::West => "West",
            Self::North => "North",
            Self::East => "East",
            Self::South => "South",
        }
    }

    /// The Burmese-script name of this direction.
    ///
    /// ```
    /// use myanmar_date::NagaDirection;
    /// assert_eq!(NagaDirection::South.burmese_name(), "တောင်");
    /// ```
    #[cfg(feature = "burmese")]
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
    #[must_use]
    pub fn burmese_name(&self) -> &'static str {
        match self {
            Self::West => "အနောက်",
            Self::North => "မြောက်",
            Self::East => "အရှေ့",
            Self::South => "တောင်",
        }
    }
}

impl Mahabote {
    /// The English name of this mahabote category.
    ///
    /// ```
    /// use myanmar_date::Mahabote;
    /// assert_eq!(Mahabote::Marana.english_name(), "Marana");
    /// ```
    #[must_use]
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Binga => "Binga",
            Self::Atun => "Atun",
            Self::Yaza => "Yaza",
            Self::Adipati => "Adipati",
            Self::Marana => "Marana",
            Self::Thike => "Thike",
            Self::Puti => "Puti",
        }
    }

    /// The Burmese-script name of this mahabote category.
    ///
    /// ```
    /// use myanmar_date::Mahabote;
    /// assert_eq!(Mahabote::Marana.burmese_name(), "မရဏ");
    /// ```
    #[cfg(feature = "burmese")]
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
    #[must_use]
    pub fn burmese_name(&self) -> &'static str {
        match self {
            Self::Binga => "ဘင်္ဂ",
            Self::Atun => "အထွန်း",
            Self::Yaza => "ရာဇ",
            Self::Adipati => "အဓိပတိ",
            Self::Marana => "မရဏ",
            Self::Thike => "သိုက်",
            Self::Puti => "ပုတိ",
        }
    }
}

impl Nakhat {
    /// The English name of this nakhat category.
    ///
    /// ```
    /// use myanmar_date::Nakhat;
    /// assert_eq!(Nakhat::Human.english_name(), "Human");
    /// ```
    #[must_use]
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Ogre => "Ogre",
            Self::Elf => "Elf",
            Self::Human => "Human",
        }
    }

    /// The Burmese-script name of this nakhat category.
    ///
    /// ```
    /// use myanmar_date::Nakhat;
    /// assert_eq!(Nakhat::Human.burmese_name(), "လူ");
    /// ```
    #[cfg(feature = "burmese")]
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
    #[must_use]
    pub fn burmese_name(&self) -> &'static str {
        match self {
            Self::Ogre => "ဘီလူး",
            Self::Elf => "နတ်",
            Self::Human => "လူ",
        }
    }
}

impl YearName {
    /// The English (Sanskrit-derived) name of this year name.
    ///
    /// ```
    /// use myanmar_date::YearName;
    /// assert_eq!(YearName::Jyeshtha.english_name(), "Jyeshtha");
    /// ```
    #[must_use]
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Hpusha => "Hpusha",
            Self::Magha => "Magha",
            Self::Phalguni => "Phalguni",
            Self::Chitra => "Chitra",
            Self::Visakha => "Visakha",
            Self::Jyeshtha => "Jyeshtha",
            Self::Ashadha => "Ashadha",
            Self::Sravana => "Sravana",
            Self::Bhadrapaha => "Bhadrapaha",
            Self::Asvini => "Asvini",
            Self::Krittika => "Krittika",
            Self::Mrigasiras => "Mrigasiras",
        }
    }

    /// The Burmese-script name of this year name.
    ///
    /// ```
    /// use myanmar_date::YearName;
    /// assert_eq!(YearName::Jyeshtha.burmese_name(), "စိဿနှစ်");
    /// ```
    #[cfg(feature = "burmese")]
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
    #[must_use]
    pub fn burmese_name(&self) -> &'static str {
        match self {
            Self::Hpusha => "ပုဿနှစ်",
            Self::Magha => "မာခနှစ်",
            Self::Phalguni => "ဖ္လကိုန်နှစ်",
            Self::Chitra => "စယ်နှစ်",
            Self::Visakha => "ပိသျက်နှစ်",
            Self::Jyeshtha => "စိဿနှစ်",
            Self::Ashadha => "အာသတ်နှစ်",
            Self::Sravana => "သရဝန်နှစ်",
            Self::Bhadrapaha => "ဘဒြနှစ်",
            Self::Asvini => "အာသိန်နှစ်",
            Self::Krittika => "ကြတိုက်နှစ်",
            Self::Mrigasiras => "မြိက္ကသိုဝ်နှစ်",
        }
    }
}

impl AstroDay {
    /// The English name of this astrological day, matching upstream
    /// `cal_astro`'s own strings.
    ///
    /// ```
    /// use myanmar_date::AstroDay;
    /// assert_eq!(AstroDay::Thamanyo.english_name(), "Thamanyo");
    /// ```
    #[must_use]
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Thamanyo => "Thamanyo",
            Self::Amyeittasote => "Amyeittasote",
            Self::Warameittugyi => "Warameittugyi",
            Self::Warameittunge => "Warameittunge",
            Self::Yatpote => "Yatpote",
            Self::Thamaphyu => "Thamaphyu",
            Self::Nagapor => "Nagapor",
            Self::Yatyotema => "Yatyotema",
            Self::Mahayatkyan => "Mahayatkyan",
            Self::Shanyat => "Shanyat",
        }
    }

    /// The Burmese-script name of this astrological day.
    ///
    /// ```
    /// use myanmar_date::AstroDay;
    /// assert_eq!(AstroDay::Thamanyo.burmese_name(), "သမားညို");
    /// ```
    #[cfg(feature = "burmese")]
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
    #[must_use]
    pub fn burmese_name(&self) -> &'static str {
        match self {
            Self::Thamanyo => "သမားညို",
            Self::Amyeittasote => "အမြိတ္တစုတ်",
            Self::Warameittugyi => "ဝါရမိတ္တုကြီး",
            Self::Warameittunge => "ဝါရမိတ္တုငယ်",
            Self::Yatpote => "ရက်ပုပ်",
            Self::Thamaphyu => "သမားဖြူ",
            Self::Nagapor => "နဂါးပေါ်",
            Self::Yatyotema => "ရက်ယုတ်မာ",
            Self::Mahayatkyan => "မဟာရက်ကြမ်း",
            Self::Shanyat => "ရှမ်းရက်",
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

    #[test]
    fn every_year_name_has_a_distinct_english_name() {
        let names: std::vec::Vec<&str> = [
            YearName::Hpusha,
            YearName::Magha,
            YearName::Phalguni,
            YearName::Chitra,
            YearName::Visakha,
            YearName::Jyeshtha,
            YearName::Ashadha,
            YearName::Sravana,
            YearName::Bhadrapaha,
            YearName::Asvini,
            YearName::Krittika,
            YearName::Mrigasiras,
        ]
        .iter()
        .map(YearName::english_name)
        .collect();
        let mut unique = names.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), names.len());
    }

    #[test]
    fn every_astro_day_has_a_distinct_english_name() {
        let names: std::vec::Vec<&str> = [
            AstroDay::Thamanyo,
            AstroDay::Amyeittasote,
            AstroDay::Warameittugyi,
            AstroDay::Warameittunge,
            AstroDay::Yatpote,
            AstroDay::Thamaphyu,
            AstroDay::Nagapor,
            AstroDay::Yatyotema,
            AstroDay::Mahayatkyan,
            AstroDay::Shanyat,
        ]
        .iter()
        .map(AstroDay::english_name)
        .collect();
        let mut unique = names.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), names.len());
    }

    #[test]
    fn mahabote_nakhat_sabbath_pyathada_naga_direction_names_are_non_empty() {
        for m in [
            Mahabote::Binga,
            Mahabote::Atun,
            Mahabote::Yaza,
            Mahabote::Adipati,
            Mahabote::Marana,
            Mahabote::Thike,
            Mahabote::Puti,
        ] {
            assert_ne!(m.english_name(), "");
        }
        for n in [Nakhat::Ogre, Nakhat::Elf, Nakhat::Human] {
            assert_ne!(n.english_name(), "");
        }
        assert_ne!(
            Sabbath::Sabbath.english_name(),
            Sabbath::SabbathEve.english_name()
        );
        assert_ne!(
            Pyathada::Pyathada.english_name(),
            Pyathada::Afternoon.english_name()
        );
        for d in [
            NagaDirection::West,
            NagaDirection::North,
            NagaDirection::East,
            NagaDirection::South,
        ] {
            assert_ne!(d.english_name(), "");
        }
    }

    #[cfg(feature = "burmese")]
    #[test]
    fn year_name_burmese_names_are_non_empty_and_distinct() {
        let names: std::vec::Vec<&str> = [
            YearName::Hpusha,
            YearName::Magha,
            YearName::Phalguni,
            YearName::Chitra,
            YearName::Visakha,
            YearName::Jyeshtha,
            YearName::Ashadha,
            YearName::Sravana,
            YearName::Bhadrapaha,
            YearName::Asvini,
            YearName::Krittika,
            YearName::Mrigasiras,
        ]
        .iter()
        .map(YearName::burmese_name)
        .collect();
        for n in &names {
            assert_ne!(*n, "");
        }
        let mut unique = names.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), names.len());
    }

    #[cfg(feature = "burmese")]
    #[test]
    fn remaining_enums_have_distinct_non_empty_burmese_names() {
        fn all_distinct_non_empty(names: &[&str]) {
            for n in names {
                assert_ne!(*n, "");
            }
            let mut u = names.to_vec();
            u.sort_unstable();
            u.dedup();
            assert_eq!(u.len(), names.len());
        }

        all_distinct_non_empty(&[
            YearType::Common.burmese_name(),
            YearType::LittleWatat.burmese_name(),
            YearType::BigWatat.burmese_name(),
        ]);
        all_distinct_non_empty(&[
            Sabbath::Sabbath.burmese_name(),
            Sabbath::SabbathEve.burmese_name(),
        ]);
        all_distinct_non_empty(&[
            Pyathada::Pyathada.burmese_name(),
            Pyathada::Afternoon.burmese_name(),
        ]);
        all_distinct_non_empty(&[
            NagaDirection::West.burmese_name(),
            NagaDirection::North.burmese_name(),
            NagaDirection::East.burmese_name(),
            NagaDirection::South.burmese_name(),
        ]);
        all_distinct_non_empty(&[
            Mahabote::Binga.burmese_name(),
            Mahabote::Atun.burmese_name(),
            Mahabote::Yaza.burmese_name(),
            Mahabote::Adipati.burmese_name(),
            Mahabote::Marana.burmese_name(),
            Mahabote::Thike.burmese_name(),
            Mahabote::Puti.burmese_name(),
        ]);
        all_distinct_non_empty(&[
            Nakhat::Ogre.burmese_name(),
            Nakhat::Elf.burmese_name(),
            Nakhat::Human.burmese_name(),
        ]);
        all_distinct_non_empty(&[
            AstroDay::Thamanyo.burmese_name(),
            AstroDay::Amyeittasote.burmese_name(),
            AstroDay::Warameittugyi.burmese_name(),
            AstroDay::Warameittunge.burmese_name(),
            AstroDay::Yatpote.burmese_name(),
            AstroDay::Thamaphyu.burmese_name(),
            AstroDay::Nagapor.burmese_name(),
            AstroDay::Yatyotema.burmese_name(),
            AstroDay::Mahayatkyan.burmese_name(),
            AstroDay::Shanyat.burmese_name(),
        ]);
    }
}
