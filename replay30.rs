fn main() {
    let _local10 = if let Ok(x) =
        hifitime::prelude::Epoch::from_gregorian_str("2022-09-16T23:15:8 ၄၄၄7-")
    {
        x
    } else {
        use std::process;
        process::exit(0);
    };
    let _local11 = hifitime::prelude::Epoch::as_tdb_duration_since_j2000(&(_local10));
    let _ = hifitime::prelude::Epoch::to_tai_parts(&(_local10));
    let _ = hifitime::prelude::Epoch::to_gregorian_tai(&(_local10));
    let _local16 = hifitime::prelude::Epoch::as_jde_tdb_duration(&(_local10));
}
