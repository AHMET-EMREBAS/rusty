fn main() {
    // Numbers
    let nums: (i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64) = (
        8,
        16,
        32,
        64,
        1231231231231,
        8,
        16,
        32,
        64,
        985712894512768,
        8.1,
        16.1,
    );

    let mut user_info: (&str, &str) = ("Firstname", "Lastname");

    user_info.0 = "New first name";
    user_info.1 = "New last name";
}
