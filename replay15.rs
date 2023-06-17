fn main() {
    let _ =
        hifitime::prelude::Epoch::from_format_str("2\u{f}\u{1f}91@Jb0JJJJJ", "J%z%JAJJJ\u{1}\0JJJ");
}
