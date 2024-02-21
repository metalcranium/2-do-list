use std::io;

struct Tasks{
    nums: u32,
    todo_list: Vec<String>,
}
impl Tasks {
   fn add_vec(&mut self){
    let mut should_add: bool = true;
    let mut my_string = String::new();
    while should_add == true {
        println!("enter the task that you would like to add");
        io::stdin().read_line(&mut my_string).expect("failed to read line");
        if my_string.trim() == "done"{
            should_add = false;
        }
        else {
            self.todo_list.push(my_string);
        }
        my_string = String::new();
    }
}
    fn remove_vec(&mut self){
        let mut should_remove: bool = true;
        let mut position = String::new();
        while should_remove == true {
            read_vec(self.todo_list.clone(), self.nums.clone());
            println!("enter the digit of a task to remove from list.");
            io::stdin().read_line(&mut position).expect("failed to read line");
            let mut position: u32 = position.trim().parse().expect("expected a number");
            if position == 0 {
                should_remove = false;
            }
            else {
                position-=1;
                self.todo_list.remove(position.try_into().unwrap());                
            } 
            self.remove_vec();
        }
    }
}
fn main() {
    let mut my_tasks = Tasks {
        nums: 0,
        todo_list: Vec::new(),
    };
    let mut choice = String::new();

    println!("Welcome to 2-do!");
    println!("Main menu:");
    println!("(1) to make a to do list.");
    println!("(2) to exit.");
    io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice: u32 = choice.trim().parse().expect("expected a number");
    match choice {
        1 => my_tasks.add_vec(),
        2 => std::process::exit(0),
        _ => todo!(),
    }
    my_tasks.remove_vec();
}
fn read_vec(my_vec: Vec<String>,mut nums: u32){
    for read in my_vec {
        nums += 1;
        print!("{nums}: {read}");
    }
}
