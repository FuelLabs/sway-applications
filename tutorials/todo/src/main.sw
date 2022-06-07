contract;

dep abi;
dep tasks;

use std::storage::StorageMap;
use abi::Todo;
use tasks::Task;

storage {
    tasks: StorageMap<u64, Task>,
}

// TODO: remove hack array once mutable arrays are ready see https://github.com/FuelLabs/sway/issues/1626

impl Todo for Contract {
     fn add_task(index: u64 value: str[20]) {
        // logic needs to be fixed to make sure they can only add up to 5 tasks
        storage.tasks.insert(index, Task {
            value,
            completed: false,
        });
    }

    fn toggle_task(index: u64) {
        // logic needs to be fixed to make sure they only toggle a task that exists
        let mut task = strorage.tasks.get(index);
        task.completed = !task.completed;
        storage.tasks.insert(index, task);
    }

   fn remove_task(index: u64) {
        // Note: that this doesn't actually remove anything... it actually adds a new task at `index`
        // this should probably be fixed with better logic for an example
        storage.tasks.insert(index, Task {
            value: "                    ",
            completed: false,
        });
    }

    fn get_tasks() -> [Task; 5] {
        // logic needs to be fixed since the user can add a task at any index
        [
            storage.tasks.get(0),
            storage.tasks.get(1),
            storage.tasks.get(2),
            storage.tasks.get(3),
            storage.tasks.get(4),
        ]
    }
}
