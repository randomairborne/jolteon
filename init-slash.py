#!/usr/bin/python3

import requests
import os
import sys
import tagdefs

token = os.environ.get("DISCORD_TOKEN")
id = sys.argv[1]

if not isinstance(id, str):
    print("The first argument must be a string!")

if not isinstance(id, str):
    print("Expected DISCORD_TOKEN in the environment.")

print(f"Bot ID {id}, using token: {token}")


global_cmds = requests.get(
    f"https://discord.com/api/v10/applications/{id}/commands",
    headers={"Authorization": f"Bot {token}"},
)

commands = []
command_names = []

for command in global_cmds.json():
    commands.append(command)
    command_names.append(command["name"])


if "tag" not in command_names:
    r = requests.post(
        f"https://discord.com/api/v10/applications/{id}/commands",
        headers={"Authorization": f"Bot {token}"},
        json=tagdefs.tag,
    )
if "tagmanage" not in command_names:
    r = requests.post(
        f"https://discord.com/api/v10/applications/{id}/commands",
        headers={"Authorization": f"Bot {token}"},
        json=tagdefs.manage_tag,
    )

if tagdefs.manage_tag not in commands:
    r = requests.patch(
        f"https://discord.com/api/v10/applications/{id}/commands/",
        headers={"Authorization": f"Bot {token}"},
        json=tagdefs.manage_tag,
    )