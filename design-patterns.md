### Creational: Singleton Pattern

```
use once_cell::sync::Lazy;
use std::sync::Mutex;

struct Database {
    // Database fields
}

impl Database {
    fn new() -> Self {
        Database {
            // initialization code
        }
    }

    fn query(&self, query: &str) -> String {
        // Perform a query
        "Result".to_string()
    }
}

static INSTANCE: Lazy<Mutex<Database>> = Lazy::new(|| {
    let db = Database::new();
    Mutex::new(db)
});

fn get_database_instance() -> &'static Mutex<Database> {
    &INSTANCE
}
```


### Structural: Adapter Pattern

```
trait Target {
    fn request(&self) -> String;
}

struct Adaptee {
    // Existing functionality
}

impl Adaptee {
    fn specific_request(&self) -> String {
        "specific result".to_string()
    }
}

struct Adapter {
    adaptee: Adaptee,
}

impl Target for Adapter {
    fn request(&self) -> String {
        self.adaptee.specific_request()
    }
}

fn client_code(target: &dyn Target) {
    println!("{}", target.request());
}
```


### Behavioral: Observer Pattern

```
use std::{cell::RefCell, rc::{Rc, Weak}};

trait Observer {
    fn update(&self, message: &str);
}

struct ConcreteObserver {
    // Observer state
    name: String,
}

impl Observer for ConcreteObserver {
    fn update(&self, message: &str) {
        println!("{} received message: {}", self.name, message);
    }
}

struct Subject {
    observers: Vec<Weak<RefCell<dyn Observer>>>,
}

impl Subject {
    fn new() -> Self {
        Subject { observers: vec![] }
    }

    fn subscribe(&mut self, observer: Weak<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn notify(&self, message: &str) {
        for observer in &self.observers {
            if let Some(observer) = observer.upgrade() {
                observer.borrow().update(message);
            }
        }
    }
}
```