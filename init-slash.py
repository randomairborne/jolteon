#!/usr/bin/python3

import requests
import os
import sys

token = os.environ.get("DISCORD_TOKEN")
id = sys.argv[1]

if not isinstance(id, str):
    print("The first argument must be a string!")

if not isinstance(id, str):
    print("Expected DISCORD_TOKEN in the environment.")

print(f"Bot ID {id}, using token: {token}")

commands_r = requests.get(f"https://discord.com/api/v10/applications/{id}/commands", headers={"Authorization": f"Bot {token}"})

print(commands_r.text)
commands = []

for command in commands_r.json():
    commands.append(command['name'])

if "tag" not in commands:
    cmd = {
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
                    "required": False
                }
            ]
        }
    r = requests.post(f"https://discord.com/api/v10/applications/{id}/commands", headers={"Authorization": f"Bot {token}"}, json=cmd)
    print(r.text)

if "managetags" not in commands:
    cmd = {
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
                            "required": False
                        }
                    ]
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
                            "required": True
                        }
                    ]
                }
            ]
        }
    r = requests.post(f"https://discord.com/api/v10/applications/{id}/commands", headers={"Authorization": f"Bot {token}"}, json=cmd)
    print(r.text)
