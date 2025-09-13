use egui::Ui;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

/// Represents a basic inventory item
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub quantity: u32,
    pub item_type: ItemType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    Material,
    Quest,
}

/// Basic inventory operations
pub struct Inventory {
    items: Vec<Item>,
    capacity: usize,
}

impl Inventory {
    /// Creates a new empty inventory with the given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity,
        }
    }

    /// Adds an item to the inventory, returns true if successful
    pub fn add_item(&mut self, item: Item) -> bool {
        if self.items.len() < self.capacity {
            // Check if item already exists and stack if possible
            if let Some(existing) = self.items.iter_mut()
                .find(|i| i.name == item.name && i.item_type == item.item_type) {
                existing.quantity += item.quantity;
                true
            } else {
                self.items.push(item);
                true
            }
        } else {
            false // Inventory full
        }
    }

    /// Removes an item from the inventory, returns the removed item if successful
    pub fn remove_item(&mut self, name: &str, quantity: u32) -> Option<Item> {
        if let Some(pos) = self.items.iter().position(|item| item.name == name) {
            let item = &mut self.items[pos];
            if item.quantity >= quantity {
                item.quantity -= quantity;
                let removed = Item {
                    name: item.name.clone(),
                    quantity,
                    item_type: item.item_type.clone(),
                };
                
                if item.quantity == 0 {
                    self.items.remove(pos);
                }
                
                Some(removed)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Gets the current number of item slots used
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if the inventory is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Gets the inventory capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Gets a reference to all items
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    /// Checks if an item exists in the inventory
    pub fn has_item(&self, name: &str) -> bool {
        self.items.iter().any(|item| item.name == name)
    }

    /// Gets the quantity of a specific item
    pub fn get_quantity(&self, name: &str) -> u32 {
        self.items.iter()
            .find(|item| item.name == name)
            .map(|item| item.quantity)
            .unwrap_or(0)
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new(50) // Default capacity of 50 items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_creation() {
        let inventory = Inventory::new(10);
        assert_eq!(inventory.capacity(), 10);
        assert_eq!(inventory.len(), 0);
        assert!(inventory.is_empty());
    }

    #[test]
    fn test_add_item() {
        let mut inventory = Inventory::new(5);
        let item = Item {
            name: "Sword".to_string(),
            quantity: 1,
            item_type: ItemType::Weapon,
        };

        assert!(inventory.add_item(item.clone()));
        assert_eq!(inventory.len(), 1);
        assert!(!inventory.is_empty());
        assert!(inventory.has_item("Sword"));
        assert_eq!(inventory.get_quantity("Sword"), 1);
    }

    #[test]
    fn test_stack_items() {
        let mut inventory = Inventory::new(5);
        let item1 = Item {
            name: "Potion".to_string(),
            quantity: 3,
            item_type: ItemType::Consumable,
        };
        let item2 = Item {
            name: "Potion".to_string(),
            quantity: 2,
            item_type: ItemType::Consumable,
        };

        assert!(inventory.add_item(item1));
        assert!(inventory.add_item(item2));
        
        // Should still be 1 slot but with 5 quantity
        assert_eq!(inventory.len(), 1);
        assert_eq!(inventory.get_quantity("Potion"), 5);
    }

    #[test]
    fn test_remove_item() {
        let mut inventory = Inventory::new(5);
        let item = Item {
            name: "Arrow".to_string(),
            quantity: 10,
            item_type: ItemType::Weapon,
        };

        inventory.add_item(item);
        
        // Remove some arrows
        let removed = inventory.remove_item("Arrow", 3);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().quantity, 3);
        assert_eq!(inventory.get_quantity("Arrow"), 7);
        
        // Remove all remaining arrows
        let removed = inventory.remove_item("Arrow", 7);
        assert!(removed.is_some());
        assert!(!inventory.has_item("Arrow"));
        assert_eq!(inventory.len(), 0);
    }

    #[test]
    fn test_remove_too_many_items() {
        let mut inventory = Inventory::new(5);
        let item = Item {
            name: "Stone".to_string(),
            quantity: 5,
            item_type: ItemType::Material,
        };

        inventory.add_item(item);
        
        // Try to remove more than available
        let removed = inventory.remove_item("Stone", 10);
        assert!(removed.is_none());
        assert_eq!(inventory.get_quantity("Stone"), 5);
    }

    #[test]
    fn test_inventory_capacity() {
        let mut inventory = Inventory::new(2);
        
        let item1 = Item {
            name: "Item1".to_string(),
            quantity: 1,
            item_type: ItemType::Material,
        };
        let item2 = Item {
            name: "Item2".to_string(),
            quantity: 1,
            item_type: ItemType::Material,
        };
        let item3 = Item {
            name: "Item3".to_string(),
            quantity: 1,
            item_type: ItemType::Material,
        };

        assert!(inventory.add_item(item1));
        assert!(inventory.add_item(item2));
        assert!(!inventory.add_item(item3)); // Should fail - capacity reached
        
        assert_eq!(inventory.len(), 2);
    }

    #[test]
    fn test_item_types() {
        let weapon = Item {
            name: "Sword".to_string(),
            quantity: 1,
            item_type: ItemType::Weapon,
        };
        let armor = Item {
            name: "Shield".to_string(),
            quantity: 1,
            item_type: ItemType::Armor,
        };

        assert_eq!(weapon.item_type, ItemType::Weapon);
        assert_eq!(armor.item_type, ItemType::Armor);
        assert_ne!(weapon.item_type, armor.item_type);
    }

    #[test]
    fn test_default_inventory() {
        let inventory = Inventory::default();
        assert_eq!(inventory.capacity(), 50);
        assert!(inventory.is_empty());
    }

    #[test]
    fn test_item_clone() {
        let item1 = Item {
            name: "Test".to_string(),
            quantity: 5,
            item_type: ItemType::Quest,
        };
        let item2 = item1.clone();

        assert_eq!(item1, item2);
        assert_eq!(item1.name, item2.name);
        assert_eq!(item1.quantity, item2.quantity);
    }
}
