fn main() {
    let _local0 = if let Ok(x) =
        hifitime::prelude::Epoch::maybe_from_gregorian_utc(2130738944, 2, 31, 8, 8, 8, 134744072)
    {
        x
    } else {
        use std::process;
        process::exit(0);
    };
    let _ = hifitime::prelude::Epoch::to_tdb_days_since_j2000(&(_local0));
    let _ = hifitime::prelude::Epoch::to_et_centuries_since_j2000(&(_local0));
    let _local5 = hifitime::prelude::Epoch::duration_in_year(&(_local0));
    let _ = hifitime::prelude::Epoch::to_utc_seconds(&(_local0));
}
