use poise::serenity_prelude::CreateEmbed;

fn make_base_embed() -> CreateEmbed {
    CreateEmbed::default()
}

pub fn base_embed() -> CreateEmbed {
    make_base_embed()
}

pub async fn make_success_embed(text: &str) -> CreateEmbed {
    make_base_embed()
        .title("Успех")
        .description(format!("{}", text))
        .color(0xb8bb26u32)
}

pub async fn make_error_embed(text: &str) -> CreateEmbed {
    make_base_embed()
        .title("Ошибка")
        .description(format!("{}", text))
        .color(0xfb4934u32)
}