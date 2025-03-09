use colored::Colorize;
use std::io;

struct Item {
    item_name: String,
    locale_index: usize,
}

impl Item {
    fn add(item_name: String, add_to: &mut Collection) {
        let possible_locales = &add_to.items;

        add_to.items.push(Item {
            item_name,
            locale_index: possible_locales.len(),
        });
    }
}

struct Collection {
    coll_name: String,
    kind_of_location: String,
    locales: Vec<String>,
    items: Vec<Item>,
}

impl Collection {
    fn new(coll_name: &str, kind_of_location: &str, locales: Vec<String>) -> Collection {
        let coll_name = String::from(coll_name);
        let kind_of_location = String::from(kind_of_location);

        Collection {
            coll_name,
            kind_of_location,
            locales,
            items: Vec::new(),
        }
    }
}

fn main() {
    //the locales and collections
    let mut user_locales: Vec<String> = Vec::new();
    let mut user_collections: Vec<Collection> = Vec::new();

    //TESTS BELOW
    user_locales.push(String::from("First"));
    user_locales.push(String::from("Second"));
    user_locales.push(String::from("Last"));

    let item_one = Item {
        item_name: String::from("Nirvana"),
        locale_index: 0,
    };

    let item_two = Item {
        item_name: String::from("Metallica"),
        locale_index: 0,
    };

    let item_three = Item {
        item_name: String::from("AC/DC"),
        locale_index: 0,
    };

    let my_collection: Collection = Collection::new("Disks", "Shelves", user_locales);

    user_collections.push(my_collection);

    user_collections[0].items.push(item_one);
    user_collections[0].items.push(item_two);
    user_collections[0].items.push(item_three);

    println!("This is my {} collection: ", user_collections[0].coll_name);

    for (index, item) in user_collections[0].items.iter().enumerate() {
        println!("{}: {}", index + 1, item.item_name);
    }

    println!("{}", input_usize("Not that one!"));
    //TESTS ENDING
}

pub fn input_usize(error_message: &str) -> usize {
    let mut read: String = String::new();
    let mut read_success: bool = false;

    let read = loop {
        read.clear();

        while !read_success {
            read_success = io::stdin().read_line(&mut read).is_ok();
        }

        match read.trim().parse::<usize>() {
            Ok(number) => break number,
            Err(_) => {
                read_success = false;
                println!("{}", error_message);
            }
        }
    };

    read
}
