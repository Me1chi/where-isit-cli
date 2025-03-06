use colored::Colorize;
use std::io;

struct Item<'a> {
    item_name: String,
    possible_locales: &'a Vec<String>,
    locale_index: usize,
}

impl<'a> Item<'a> {
    fn find_index_locale(&mut self, locale_to_find: String) {
        for (index, locale) in self.possible_locales.iter().enumerate() {
            if locale_to_find.eq(locale) {
                self.locale_index = index;
            }
        }
    }
}

struct Collection<'a> {
    coll_name: String,
    items: Vec<Item<'a>>,
}

fn main() {
    //the locales and collections
    let mut user_locales: Vec<String> = Vec::new();
    let mut user_collections: Vec<Collection> = Vec::new();

    //TESTS BELOW
    user_locales.push(String::from("Quarto"));
    user_locales.push(String::from("Sala"));
    user_locales.push(String::from("Cozinha"));

    let item_one = Item {
        item_name: String::from("Nirvana"),
        possible_locales: &user_locales,
        locale_index: 0,
    };

    let item_two = Item {
        item_name: String::from("Metallica"),
        possible_locales: &user_locales,
        locale_index: 0,
    };

    let item_three = Item {
        item_name: String::from("AC/DC"),
        possible_locales: &user_locales,
        locale_index: 0,
    };

    let my_collection = Collection {
        coll_name: String::from("discos"),
        items: vec![item_one, item_two, item_three],
    };

    user_collections.push(my_collection);

    println!("This is my {} collection: ", user_collections[0].coll_name);

    for (index, item) in user_collections[0].items.iter().enumerate() {
        println!("{}: {}", index + 1, item.item_name);
    }
    //TESTS ENDING
}
