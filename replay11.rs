fn main() {
    let _local0 = if let Ok(x) =
        hifitime::prelude::Epoch::maybe_from_gregorian_utc(-2147483640, 8, 8, 8, 8, 8, 134744072)
    {
        x
    } else {
        use std::process;
        process::exit(0);
    };
    let _ = hifitime::prelude::Epoch::as_jde_tt_days(&(_local0));
    let _ = hifitime::prelude::Epoch::to_tdb_days_since_j2000(&(_local0));
    let _ = hifitime::prelude::Epoch::to_et_centuries_since_j2000(&(_local0));
    let _ = hifitime::prelude::Epoch::as_jde_tai_days(&(_local0));
}
