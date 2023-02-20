use std::collections::HashMap;

struct VariableStore<T> {
    store: HashMap<String, T>,
}

impl<T> VariableStore<T> {
    fn new() -> VariableStore<T> {
        VariableStore {
            store: HashMap::new(),
        }
    }

    fn add_variable(&mut self, name: String, value: T) {
        self.store.insert(name, value);
    }

    fn set_variable(&mut self, name: &str, value: T) -> Result<(), &'static str> {
        match self.store.get(name) {
            Some(_) => {
                self.store.insert(name.to_string(), value);
                Ok(())
            }
            None => Err("Variable not found"),
        }
    }

    fn get_variable(&self, name: &str) -> Result<&T, &'static str> {
        match self.store.get(name) {
            Some(value) => Ok(value),
            None => Err("Variable not found"),
        }
    }
}

fn main() {
    let mut store = VariableStore::new();

    // Add variables to store
    store.add_variable("age".to_string(), 30);

    // Set variable values
    let _ = store.set_variable("age", 31);

    // Try to get a variable that doesn't exist
    let res = store.get_variable("weight");
    assert_eq!(res, Err("Variable not found"));

    // Retrieve variable values and print
    let age = store.get_variable("age").unwrap();
    let name = store.get_variable("name").unwrap();
    println!("Age: {:?}", age);
    println!("Name: {:?}", name);
}