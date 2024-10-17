# Description
This is the server side of the TaskFlow project

TaskFlow is a well-rounded to-do list. With task statuses, deadlines, and priorities. This is not a task manager because tasks are only individual

# Preparation
To install the project, run the command:
```sh
cargo install
```

After that, build the project:
```sh
cargo build --release
```

# Run
To start, you must specify the config file from which the project settings will be taken:
```sh
RUST_LOG=info ./target/release/task_flow_backend -c settings/dev.toml
```

An example of such a file can be found in the `settings` directory

# Docs
After launching, you can work with the API by looking at the documentation at the /docs/ link
