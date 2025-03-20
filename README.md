# SUT COURSE API (Rust version) 📚

This project scrapes course data from Suranaree Uni of Tech's Reg.

## Getting started 🚀

Clone this repository

```bash
git clone https://github.com/pandakn/rust-sut-courses-api.git

cd rust-sut-courses-api.git
```

### Start Project 🦀

```bash
cargo run

# run test
cargo test
```

## API Reference

#### Search Courses

```bash
{{url}}/courses?acad_year=2567&semester=2&course_code=523332&course_name=
```

## Example API Response

> **Note** : I'll adjust this response pretty soon 👀.

```json
[
  {
    "course_code": "523332",
    "version": "2",
    "course_name": {
      "en": "SOFTWARE ENGINEERING",
      "th": "วิศวกรรมซอฟต์แวร์"
    },
    "credit": "4 (3-3-9)",
    "degree": "1",
    "department": "วิศวกรรมคอมพิวเตอร์",
    "faculty": "สำนักวิชาวิศวกรรมศาสตร์",
    "course_status": "ใช้งาน",
    "course_condition": ["523331"],
    "continue_course": ["523435"],
    "equivalent_course": [],
    "sections_count": 2,
    "sections": [
      {
        "id": "45045cda-17a8-4666-b72e-68c51e0b51c2",
        "url": "http://reg.sut.ac.th/registrar/class_info_2.asp?backto=home&option=0&courseid=1009172&coursecode=523332&acadyear=2567&semester=2&avs438812873=1",
        "section": "1",
        "status": "เปิดลงปกติ สามารถลงทะเบียนผ่าน WEB ได้",
        "note": "สำหรับหลักสูตรปรับปรุง พ.ศ. 2560",
        "professors": ["AA AAิ์", "BB BB", "CC CC", "DD DD"],
        "language": "TH",
        "seat": {
          "total_seat": "5",
          "registered": "5",
          "remain": "0"
        },
        "class_schedule": [
          {
            "day": "Mo",
            "times": "09:00-12:00",
            "room": "B1121"
          },
          {
            "day": "We",
            "times": "09:00-12:00",
            "room": "F11-422.Software"
          }
        ],
        "exams": {
          "midterm": {
            "date": "9",
            "month": "December",
            "times": "12:00-14:00",
            "year": "2567",
            "room": "อาคารB2ห้องB5204(สอบตามตารางมหาวิทยาลัย)"
          },
          "final": {
            "date": "27",
            "month": "January",
            "times": "09:00-11:00",
            "year": "2568",
            "room": "อาคารBห้องB2102(สอบตามตารางมหาวิทยาลัย)"
          }
        }
      },
      {
        "id": "3b329743-2c51-4733-b6a0-a96483af9dab",
        "url": "http://reg.sut.ac.th/registrar/class_info_2.asp?backto=home&option=0&courseid=1009172&coursecode=523332&acadyear=2567&semester=2&avs438812873=2",
        "section": "2",
        "status": "เปิดลงปกติ สามารถลงทะเบียนผ่าน WEB ได้",
        "note": "สำหรับหลักสูตรปรับปรุง พ.ศ. 2560",
        "professors": ["AA AAิ์", "BB BB", "CC CC", "DD DD"],
        "language": "TH",
        "seat": {
          "total_seat": "5",
          "registered": "5",
          "remain": "0"
        },
        "class_schedule": [
          {
            "day": "Mo",
            "times": "09:00-12:00",
            "room": "B1121"
          },
          {
            "day": "We",
            "times": "13:00-16:00",
            "room": "F11-422.Software"
          }
        ],
        "exams": {
          "midterm": {
            "date": "9",
            "month": "December",
            "times": "12:00-14:00",
            "year": "2567",
            "room": "อาคารB2ห้องB5204(สอบตามตารางมหาวิทยาลัย)"
          },
          "final": {
            "date": "27",
            "month": "January",
            "times": "09:00-11:00",
            "year": "2568",
            "room": "อาคารBห้องB2102(สอบตามตารางมหาวิทยาลัย)"
          }
        }
      }
    ]
  }
]
```
