contract;

use std::storage::{get, store};

struct Todo {
    completed: bool,
    value: str[20],
}

abi TodoContract {
    fn get_todos() -> [Todo; 5];
    fn add_todo(index: u8, value: str[20]) -> [Todo; 5];
    fn toggle_todo(index: u8) -> [Todo; 5];
    fn remove_todo(index: u8) -> [Todo; 5];
}

// TODO: remove hack array once mutable arrays are ready see https://github.com/FuelLabs/sway/issues/1626
// 🫠🫠🫠 a virtual mutable array 🫠🫠🫠
const TODO_KEY_0 = 0x0000000000000000000000000000000000000000000000000000000000000000;
const TODO_KEY_1 = 0x0000000000000000000000000000000000000000000000000000000000000001;
const TODO_KEY_2 = 0x0000000000000000000000000000000000000000000000000000000000000002;
const TODO_KEY_3 = 0x0000000000000000000000000000000000000000000000000000000000000003;
const TODO_KEY_4 = 0x0000000000000000000000000000000000000000000000000000000000000004;

impl TodoContract for Contract {
     fn add_todo(index: u8, value: str[20]) -> [Todo; 5] {
        let todo = Todo {
            value,
            completed: false,
        };
        // @see TODO: ^
        match index {
            0 => {
                store(TODO_KEY_0, todo);
            },
            1 => {
                store(TODO_KEY_1, todo);
            },
            2 => {
                store(TODO_KEY_2, todo);
            },
            3 => {
                store(TODO_KEY_3, todo);
            },
            4 => {
                store(TODO_KEY_4, todo);
            },
            _ => {
                
            },
        };

        // @see TODO: ^
        [
            get::<Todo>(TODO_KEY_0),
            get::<Todo>(TODO_KEY_1),
            get::<Todo>(TODO_KEY_2),
            get::<Todo>(TODO_KEY_3),
            get::<Todo>(TODO_KEY_4),
        ]
    }
    fn toggle_todo(index: u8) -> [Todo; 5] {
        // @see TODO: ^
        match index {
            0 => {
                let current = get::<Todo>(TODO_KEY_0);
                store(TODO_KEY_0, Todo {
                    value: current.value,
                    completed: !current.completed,
                });
            },
            1 => {
                let current = get::<Todo>(TODO_KEY_1);
                store(TODO_KEY_1, Todo {
                    value: current.value,
                    completed: !current.completed,
                });
            },
            2 => {
                let current = get::<Todo>(TODO_KEY_2);
                store(TODO_KEY_2, Todo {
                    value: current.value,
                    completed: !current.completed,
                });
            },
            3 => {
                let current = get::<Todo>(TODO_KEY_3);
                store(TODO_KEY_3, Todo {
                    value: current.value,
                    completed: !current.completed,
                });
            },
            4 => {
                let current = get::<Todo>(TODO_KEY_4);
                store(TODO_KEY_4, Todo {
                    value: current.value,
                    completed: !current.completed,
                });
            },
            _ => {
                
            },
        };

        // @see TODO: ^
        [
            get::<Todo>(TODO_KEY_0),
            get::<Todo>(TODO_KEY_1),
            get::<Todo>(TODO_KEY_2),
            get::<Todo>(TODO_KEY_3),
            get::<Todo>(TODO_KEY_4),
        ]
    }
   fn remove_todo(index: u8) -> [Todo; 5] {
        let value: str[20] = "                    ";
        let todo = Todo {
            value,
            completed: false,
        };
        // @see TODO: ^
        match index {
            0 => {
                store(TODO_KEY_0, todo);
            },
            1 => {
                store(TODO_KEY_1, todo);
            },
            2 => {
                store(TODO_KEY_2, todo);
            },
            3 => {
                store(TODO_KEY_3, todo);
            },
            4 => {
                store(TODO_KEY_4, todo);
            },
            _ => {
                
            },
        };

        // @see TODO: ^
        [
            get::<Todo>(TODO_KEY_0),
            get::<Todo>(TODO_KEY_1),
            get::<Todo>(TODO_KEY_2),
            get::<Todo>(TODO_KEY_3),
            get::<Todo>(TODO_KEY_4),
        ]
    }
    fn get_todos() -> [Todo; 5] {
        [
            get::<Todo>(TODO_KEY_0),
            get::<Todo>(TODO_KEY_1),
            get::<Todo>(TODO_KEY_2),
            get::<Todo>(TODO_KEY_3),
            get::<Todo>(TODO_KEY_4),
        ]
    }
}
