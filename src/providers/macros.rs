macro_rules! provider_list {
    [$(&$p:expr), +] => {
        {
            type OptionalProvidersList<'a> = ::std::vec::Vec<
                ::std::option::Option<&'a dyn $crate::providers::Provider>>;

            let mut seen = vec![];
            let providers: OptionalProvidersList = vec![$({
                let id = $crate::utils::typeid_of_val(&$p);
                if !seen.contains(&id) {
                    seen.push(id);
                    Some(&$p)
                } else {
                    None
                }
            }
            ), +];

            ::std::iter::IntoIterator::into_iter(providers)
                .flatten()
                .collect()
        }
    };
    [$($p:expr), +] => {
        $crate::providers::macros::provider_list![$(&$p), +]
    };
    [$($p:expr,) +] => {
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
