fn main() {
    let mut names: Vec<&'static str> = chrono_tz::TZ_VARIANTS.iter().map(|tz| tz.name()).collect();
    names.sort_unstable();
    for n in names {
        println!("{}", n);
    }
}
