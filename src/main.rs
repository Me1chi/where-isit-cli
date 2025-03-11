use colored::Colorize;
use std::cmp::Ordering;
use std::io;

struct Item {
    item_name: String,
    locale_index: usize,
}

impl Item {
    fn add_to_collection(item_name: Option<String>, coll_target: &mut Collection) {
        let possible_locales = &coll_target.locales;
        let item_name = match item_name {
            Some(name) => name,
            None => loop {
                let mut buffer: String = String::new();

                println!("Enter item name: ");

                match io::stdin().read_line(&mut buffer) {
                    Ok(_) => break String::from(buffer.trim()),
                    Err(_) => (),
                }
            },
        };

        let locale_index = {
            println!("Choose the item locale: (Use the index)");

            for (index, locale) in possible_locales.iter().enumerate() {
                println!("{}: {}", index, locale);
            }

            loop {
                let index = input_usize("Sorry, input an index from the list");

                match index.cmp(&possible_locales.len()) {
                    Ordering::Less => break index,
                    _ => println!("Sorry, input an index from the list"),
                }
            }
        };

        coll_target.items.push(Item {
            item_name,
            locale_index,
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

    let mut my_collection: Collection = Collection::new("Disks", "Shelves", user_locales);

    Item::add_to_collection(None, &mut my_collection);

    user_collections.push(my_collection);

    user_collections[0].items.push(item_one);
    user_collections[0].items.push(item_two);
    user_collections[0].items.push(item_three);

    println!("This is my {} collection: ", user_collections[0].coll_name);

    for (index, item) in user_collections[0].items.iter().enumerate() {
        println!("{}: {}", index, item.item_name);
    }
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
