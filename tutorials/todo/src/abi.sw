library abi;

dep tasks;
use tasks::Task;

abi Todo {
    fn get_tasks() -> [Task; 5];
    fn add_task(index: u64, value: str[20]);
    fn toggle_task(index: u64);
    fn remove_task(index: u64);
}
