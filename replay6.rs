fn main() {
    let _ = hifitime::prelude::Epoch::from_format_str(
        "\u{f}4\u{1d}11-0\u{10}j0%%Y\u{c}밀%B",
        "BBBz%%d\0\0",
    );
}