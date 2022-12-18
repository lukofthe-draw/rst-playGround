// Define an enum for the different types of food
enum FoodType {
    Vegetable,
    Meat,
    Dairy,
    Fruit,
    Bakery,
}

// Define a struct to represent a single food item
struct Food {
    name: String,
    food_type: FoodType,
    expiration: u32, // days until expiration
}

// Define a struct to represent the fridge
struct Fridge {
    foods: Vec<Food>,
}

// Implement methods on the Fridge struct
impl Fridge {
    // Add a new food item to the fridge
    fn add_food(&mut self, food: Food) {
        self.foods.push(food);
    }

    // Remove a food item from the fridge by name
    fn remove_food(&mut self, food_name: &str) {
        // Find the index of the food item with the given name
        let index = self.foods.iter().position(|food| food.name == food_name);
        // If the food item was found, remove it from the fridge
        if let Some(index) = index {
            self.foods.remove(index);
        }
    }

    // List all of the food items in the fridge
    fn list_foods(&self) {
        println!("Foods in fridge:");
        for food in &self.foods {
            println!("- {} (expires in {} days)", food.name, food.expiration);
        }
    }
}

fn main() {
    // Create a new fridge
    let mut fridge = Fridge { foods: Vec::new() };

    // Add some food items to the fridge
    fridge.add_food(Food {
        name: String::from("Carrots"),
        food_type: FoodType::Vegetable,
        expiration: 5,
    });
    fridge.add_food(Food {
        name: String::from("Ground beef"),
        food_type:
