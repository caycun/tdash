
# tdash

Create a dashboard where you can see all output from different terminal commands in a layout.


## Demo
![](https://media.discordapp.net/attachments/1055343147833626757/1269101410205827143/Showcase.gif?ex=66aed64a&is=66ad84ca&hm=9c2075c67728ac9a43e2b7a97a280c925452b4ca05613a44c7fc2441ff8acc25&=)

## Keybinds

| Function             | Key                                                                |
| ----------------- | ------------------------------------------------------------------ |
| Focus screen | `f + number`|
| Full screen mode | `Enter` |
| Exit full screen | `Esc`|
| Quit tdash | `q` |

## How to use
#### Step 1
```
cargo install tdash
```

#### Step  2

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

#### Step 3

Run the command:
`tdash -file <yaml_file_name>`




