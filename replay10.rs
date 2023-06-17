fn main() {
    let _local0 = hifitime::prelude::Epoch::from_unix_milliseconds(-1.0872393402666701e193);
    let _local11 = hifitime::prelude::Epoch::to_tdb_duration_since_j1900(&(_local0));
    let _local13 = hifitime::prelude::Epoch::to_mjd_tt_duration(&(_local0));
    let _ = hifitime::prelude::Duration::max(&(_local11), _local13);
    let _ = hifitime::prelude::Duration::abs(&(_local11));
}