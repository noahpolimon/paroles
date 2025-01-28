macro_rules! provider_list {
    [$($p:expr), +] => {
        {
            let mut seen = vec![];
            let mut providers: $crate::providers::ProviderList = vec![];

            ($({
                let id = $crate::utils::typeid_of_val($p);
                if !seen.contains(&id) {
                    seen.push(id);
                    providers.push($p);
                }
            }
            ), +);

            providers
        }
    };
    [$($p:expr), +] => {
        $crate::providers::macros::provider_list![$($p), +]
    };
}

macro_rules! lyrics_finder {
    ($($p:expr), +) => {
            $crate::providers::LyricsFinder::new(
                $crate::providers::macros::provider_list![$($p), +]
            )
    };
    ($($p:expr,) +) => {
        $crate::providers::macros::lyrics_finder!($($p), +)
    };
}

pub(crate) use lyrics_finder;
pub(crate) use provider_list;
