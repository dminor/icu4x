mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use icu_locale::{LanguageIdentifier, Locale};

fn langid_benches(c: &mut Criterion) {
    let path = "./benches/fixtures/langid.json";
    let data: fixtures::LocaleList = helpers::read_fixture(path).expect("Failed to read a fixture");

    // Construct
    {
        let mut group = c.benchmark_group("langid/construct");

        construct!(group, LanguageIdentifier, "langid", &data.canonicalized);
        construct!(group, Locale, "locale", &data.canonicalized);

        group.finish();
    }

    // Stringify
    {
        let mut group = c.benchmark_group("langid/to_string");

        let langids: Vec<LanguageIdentifier> = data
            .canonicalized
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();

        to_string!(group, LanguageIdentifier, "langid", &langids);
        to_string!(group, Locale, "locale", &langids);

        group.finish();
    }

    // Compare
    {
        let mut group = c.benchmark_group("langid/compare");

        let langids: Vec<LanguageIdentifier> = data
            .canonicalized
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        let langids2: Vec<LanguageIdentifier> = data
            .canonicalized
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();

        compare_struct!(group, LanguageIdentifier, "langid", &langids, &langids2);
        compare_struct!(group, Locale, "locale", &langids, &langids2);

        compare_str!(
            group,
            LanguageIdentifier,
            "langid",
            &langids,
            &data.canonicalized
        );
        compare_str!(group, Locale, "locale", &langids, &data.canonicalized);

        group.finish();
    }

    // Canonicalize
    {
        let mut group = c.benchmark_group("langid/canonicalize");

        canonicalize!(group, LanguageIdentifier, "langid", &data.casing);
        canonicalize!(group, Locale, "locale", &data.casing);

        group.finish();
    }
}

criterion_group!(benches, langid_benches,);
criterion_main!(benches);
