fn main() {
    let _local0 =
        if let Ok(x) = hifitime::prelude::Epoch::maybe_from_gregorian_utc(1792, 1, 1, 0, 0, 0, 0) {
            x
        } else {
            use std::process;
            process::exit(0);
        };
    let _local5 = hifitime::prelude::Epoch::duration_in_year(&(_local0));
    let _ = hifitime::prelude::Epoch::to_utc_seconds(&(_local0));
    let _ = hifitime::prelude::Duration::decompose(&(_local5));
    let _ = hifitime::prelude::Epoch::ceil(&(_local0), _local5);
}