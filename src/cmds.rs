use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::command::{
    BooleanBuilder, CommandBuilder, StringBuilder, SubCommandBuilder, UserBuilder,
};

fn get() -> [Command; 2] {
    [
        CommandBuilder::new(
            "managetags",
            "Manage tags in this server",
            CommandType::ChatInput,
        )
        .option(
            SubCommandBuilder::new("create", "Add a new tag")
                .option(StringBuilder::new("name", "Tag name").required(true))
                .option(StringBuilder::new("content", "What the tag should send").required(true))
                .option(BooleanBuilder::new(
                    "allow_pings",
                    "Allow users to be notified when mentioned",
                )),
        )
        .option(
            SubCommandBuilder::new("edit", "Edit a tag")
                .option(
                    StringBuilder::new("name", "Tag name")
                        .required(true)
                        .autocomplete(true),
                )
                .option(StringBuilder::new("content", "What the tag should send"))
                .option(BooleanBuilder::new(
                    "allow_pings",
                    "Allow users to be notified when mentioned",
                )),
        )
        .option(
            SubCommandBuilder::new("delete", "Delete a tag").option(
                StringBuilder::new("name", "Tag name")
                    .required(true)
                    .autocomplete(true),
            ),
        )
        .validate()
        .expect("Level slash command is invalid!")
        .build(),
        CommandBuilder::new("tag", "Send a tag", CommandType::ChatInput)
            .option(StringBuilder::new("name", "Name of the tag you want to send").required(true))
            .option(UserBuilder::new("mention", "Person to mention in the tag").required(false))
            .validate()
            .expect("Rank slash command is invalid!")
            .build(),
    ]
}
