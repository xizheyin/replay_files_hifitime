fn main() {
    let _local0 = hifitime::prelude::Epoch::from_jde_gpst(1.7733415202279422e-294);
    let _local4 = hifitime::prelude::Epoch::to_mjd_tt_duration(&(_local0));
    let _local10 =
        if let Ok(x) = hifitime::prelude::Epoch::from_gregorian_str("\n\n94-11-05T08:15:34.0-:0") {
            x
        } else {
            use std::process;
            process::exit(0);
        };
    let _local11 = hifitime::prelude::Epoch::as_tdb_duration_since_j2000(&(_local10));
    let _local12 = hifitime::prelude::Duration::min(&(_local4), _local11);
    let _ = hifitime::prelude::Epoch::to_tai_parts(&(_local10));
}