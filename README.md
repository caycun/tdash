
# tdash

Create a dashboard where you can see all output from different terminal commands in a layout.


## Demo

GIF HERE


## How to use

#### Step  1

Create a .yaml file with the commands you want to use

#### Example:

```
- command: "ping google.com"
size: 40
direction: 1
- command: "ping facebook.com"
size: 20
direction: 2
- command: "ping youtube.com"
size: 10
direction: 2
```

Currently, direction system is not implemented yet. Direction is whether the screen should be horizontal[1] or vertical[2]

#### Step 2

Run the command:
`cargo run -- --file <yaml_file_name>`

## Keybinds

| Function             | Key                                                                |
| ----------------- | ------------------------------------------------------------------ |
| Focus screen | `f + number`|
| Full screen mode | `Enter` |
| Exit full screen | `Esc`|
| Quit tdash | `q` |


