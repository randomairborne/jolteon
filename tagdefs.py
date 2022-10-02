manage_tag = {
    "name": "managetags",
    "description": "Manage tags in this server",
    "options": [
        {
            "name": "create",
            "description": "Create a new tag or overwrite an old one",
            "type": 1,
            "options": [
                {
                    "name": "name",
                    "description": "The tag that should be used to send this message",
                    "type": 3,
                    "required": True,
                },
                {
                    "name": "content",
                    "description": "What you want the bot to send when this tag is triggered",
                    "type": 3,
                    "required": True,
                },
                {
                    "name": "allow_pings",
                    "description": "If you want to send notifications to mentioned users",
                    "type": 5,
                    "required": False,
                },
                {
                    "name": "public",
                    "description": "Allow this tag to be autocompleted and the name revealed when used",
                    "type": 5,
                    "required": False,
                },
            ],
        },
        {
            "name": "edit",
            "description": "Edit an existing tag",
            "type": 1,
            "options": [
                {
                    "name": "name",
                    "description": "The tag that should be used to send this message",
                    "type": 3,
                    "required": True,
                },
                {
                    "name": "content",
                    "description": "What you want the bot to send when this tag is triggered",
                    "type": 3,
                    "required": False,
                },
                {
                    "name": "allow_pings",
                    "description": "If you want to send notifications to mentioned users",
                    "type": 5,
                    "required": False,
                },
                {
                    "name": "public",
                    "description": "Allow this tag to be autocompleted and the name revealed when used",
                    "type": 5,
                    "required": False,
                },
            ],
        },
        {
            "name": "delete",
            "description": "Delete a tag that is no longer needed",
            "type": 1,
            "options": [
                {
                    "name": "name",
                    "description": "The name of the tag you want to delete",
                    "type": 3,
                    "required": True,
                }
            ],
        },
    ],
}

tag = {
    "name": "tag",
    "type": 1,
    "description": "Send a tag",
    "options": [
        {
            "name": "name",
            "description": "Name of the tag you want to send",
            "type": 3,
            "required": True,
        },
        {
            "name": "mention",
            "description": "Person to mention in the tag, if it's for just one person",
            "type": 6,
            "required": False,
        },
    ],
}