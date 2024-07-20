pub mod music;
pub mod errors;

use crate::utils::prelude::{Data, Error};

pub fn all_commands() -> Vec<poise::Command<Data, Error>> {
    let mut all_commands = vec![
        // Music
        music::play(),
        music::join(),
        music::leave(),
        music::volume(),
        music::queue(),
        music::skip(),
        music::pause(),
        music::resume(),
        music::stop(),
        music::seek(),
        music::clear(),
        music::remove(),
        music::swap()
    ];

    poise::framework::set_qualified_names(&mut all_commands);

    // for command in all_commands.iter_mut() {
    //     preprocess_command(command);
    // }

    all_commands
}