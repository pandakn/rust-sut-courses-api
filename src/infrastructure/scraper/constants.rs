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
