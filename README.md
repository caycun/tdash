
# tdash

Create a dashboard where you can see all output from different terminal commands in a layout.


## Demo
![](https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExdXloYnc1Z2hsbjk0aTdjbDk0bDhucG5rMHV1MjBncDZzNTF0eTc5OSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/G0RZoyKexV71ZnB50Y/giphy.gif)

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
`tdash --file <yaml_file_name>`




