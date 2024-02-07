
# tdash

Create a dashboard where you can see all output from different terminal commands in a layout.


## Demo

GIF HERE


## How to use

#### Step  1

Create a yaml file with the commands you want to use

#### Example:

```
one: node index.js
two: cargo run
three: ping github.com
```

#### Step 2

Run the command:
`cargo run -- --file <yaml_file_name>`

