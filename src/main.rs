use rusqlite::{Connection, OpenFlags};

mod tables;
mod util;

use tables::{handle::Handle, messages::Message, table::Table};
use util::dates::format;

fn main() {
    let db_path = "/Users/chris/Library/Messages/chat.db";
    let db = match Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_ONLY) {
        Ok(res) => res,
        Err(why) => panic!("Unable to read from chat database: {}\nEnsure full disk access is enabled for your terminal emulator in System Preferences > Security and Privacy > Full Disk Access", why),
    };

    // Get contacts
    let contacts = Handle::make_cache(&db);
    let unknown = "Unknown".to_string();

    let mut statement = Message::get(&db);
    let messages = statement
        .query_map([], |row| Ok(tables::messages::Message::from_row(row)))
        .unwrap();
    for message in messages {
        let msg = message.unwrap().unwrap();
        let handle = contacts.get(&msg.handle_id).unwrap_or(&unknown);
        if *handle == unknown {
            println!(
                "{:?} | {} {:?}",
                format(&msg.date()),
                match msg.is_from_me {
                    true => "Me",
                    false => handle,
                },
                msg.text
            );
        }
    }
}
