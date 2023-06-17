fn main() {
    let _ = hifitime::prelude::Epoch::from_format_str("%%%1밀%j0%", "%M밀%z\r%Y\0\0");
}