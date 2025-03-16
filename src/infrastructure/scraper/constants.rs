use std::collections::HashMap;

// Course status mapping
lazy_static::lazy_static! {
    pub static ref COURSE_STATUS_OBJ: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("A", "เพิ่มผ่าน WEB ได้เท่านั้น");
        m.insert("C", "ปิดไม่รับลง");
        m.insert("D", "ถอนผ่าน WEB ได้เท่านั้น");
        m.insert("N", "เปิดลงปกติ ทำการโดยเจ้าหน้าที่เท่านั้น");
        m.insert("W", "เปิดลงปกติ สามารถลงทะเบียนผ่าน WEB ได้");
        m.insert("X", "เปลี่ยนกลุ่มผ่าน WEB ได้เท่านั้น");
        m
    };
}

lazy_static::lazy_static! {
    pub static ref MONTH_ABBREVIATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("ม.ค.", "January");
        m.insert("ก.พ.", "February");
        m.insert("มี.ค.", "March");
        m.insert("เม.ย.", "April");
        m.insert("พ.ค.", "May");
        m.insert("มิ.ย.", "June");
        m.insert("ก.ค.", "July");
        m.insert("ส.ค.", "August");
        m.insert("ก.ย.", "September");
        m.insert("ต.ค.", "October");
        m.insert("พ.ย.", "November");
        m.insert("ธ.ค.", "December");
        m
    };
}
