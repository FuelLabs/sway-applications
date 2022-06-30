library data_structures;

dep data_structures;
use data_structures::Task;

abi Todo {
    // returns back an array of 5 items, of type Task
    fn get_tasks() -> [Task; 5];
    // sets a new Task into array at index, with `completed` defaulted to `false`
    fn add_task(index: u64, value: str[20]);
    // toggles the Task property `completed` on Task at this index
    fn toggle_task(index: u64);
    // removes the Task at this index
    fn remove_task(index: u64);
}
