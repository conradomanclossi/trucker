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

pub struct Categories {
    // A Categories modelation
    pub vec: Vec<Category>,
}

impl fmt::Debug for Categories {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Categories").field("vec", &self.vec).finish()
    }
}

impl Categories {
    pub fn new() -> Categories {
        // Create a new Categories
        Categories { vec: Vec::new() }
    }

    pub fn post(&mut self, id: u32, name: &'static str) {
        // Add a Category to the Categories
        self.vec.push(Category {
            id: id,
            name: name,
            created: Utc::now(),
        });
    }

    pub fn get(&self, id: u32) -> Option<&Category> {
        // Get a Category from the Categories
        self.vec.iter().find(|&category| category.id == id)
    }

    pub fn put(&mut self, id: u32, name: &'static str) {
        // Update a Category from the Categories
        for category in &mut self.vec {
            if category.id == id {
                category.name = name;
            }
        }
    }

    pub fn delete(&mut self, id: u32) {
        // Delete a Category from the Categories
        self.vec.retain(|category| category.id != id);
    }
}
