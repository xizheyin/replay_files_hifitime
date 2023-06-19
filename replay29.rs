fn main() {
    let _local10 = if let Ok(x) = hifitime::prelude::Epoch::from_gregorian_str("2004-07-06T23:25:38.1168423218    9              000000872180000000230-05:009 5\"\"\u{8106c}") {
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
