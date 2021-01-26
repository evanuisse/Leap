pub fn is_leap_year(year: u64) -> bool {
    let mut leap = false;
    if year % 4 == 0 {
        leap = true;
        if year % 100 == 0 {
            leap = false;
            if year % 400 == 0 {
                leap = true;
            }
        }
    }
    leap
}
}
