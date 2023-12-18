use crate::resource::Resource;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub static DATA: Lazy<Mutex<HashMap<u32, Resource>>> = Lazy::new(|| {
    Mutex::new(HashMap::from([
        (
            100001,
            Resource {
                id: 100001,
                name: "Jane Doe".into(),
                email: "jane@doe.com".into(),
                role: "Senior Manager".into(),
                emp_type: "Fulltime".into(),
                manager: "Jack Jones".into(),
            },
        ),
        (
            100002,
            Resource {
                id: 100002,
                name: "John Doe".into(),
                email: "john@doe.com".into(),
                role: "Consultant, Client Services 4".into(),
                emp_type: "Fulltime".into(),
                manager: "Jane Doe".into(),
            },
        ),
        (
            100003,
            Resource {
                id: 100003,
                name: "Jill Doe".into(),
                email: "jill@doe.com".into(),
                role: "Consultant, Client Services 3".into(),
                emp_type: "Contractor".into(),
                manager: "Jane Doe".into(),
            },
        ),
    ]))
});
