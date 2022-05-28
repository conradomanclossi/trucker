use chrono::{DateTime, Utc};
use core::fmt;

pub struct Category {
    // A Category modelation
    pub id: u32,
    pub name: &'static str,
    pub created: DateTime<Utc>,
}

impl fmt::Debug for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("created", &self.created)
            .finish()
    }
}

impl Category {
    pub fn add(id: u32, name: &'static str) -> Category {
        // Add a Category and the date was created
        Category {
            id: id,
            name: name,
            created: Utc::now(),
        }
    }
}
