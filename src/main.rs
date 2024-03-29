use std::collections::HashMap;
use std::io::{self, stdin, Read, Write};
use std::path::Path;
use serde_json::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};



#[derive(Serialize, Deserialize)]
struct Todo {
    id: i32,
    name: String,
    description: String,
    done: bool,
}

fn main() {
    // Open the file and read its contents into a string
    let mut list_file = File::options().read(true).write(true).create(true).open("todos_list.json").expect("Something went wrong connecting to file");
    let mut string_json = String::new();
    list_file.read_to_string(&mut string_json).expect("Something went wrong reading file");

    // Deserialize the JSON string into a HashMap if it's not empty
    let mut todos_list: HashMap<String, String> = HashMap::new();
    if !string_json.is_empty() {
        todos_list = serde_json::from_str(&string_json).expect("Something went wrong reading the file");
    }

    println!("Welcome to local todo app\nInput -help for a complete list of commands");

    loop {
        // Prompt the user for input and display the menu
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("you're a genius if u made him panic here");

        match choice.trim() {
            "create" => {
                // Prompt the user for a todo name
                println!("Your todo name: ");
                let mut todo_name = String::new();
                io::stdin().read_line(&mut todo_name).expect("Invalid todo name");

                // Check if a file with the same name already exists
                if !Path::new(&format!("{}.json", todo_name.trim())).exists() {
                    // Insert the todo into the todos_list HashMap
                    todos_list.insert(todo_name.trim().to_string(), create_todo(todo_name));
                } else {
                    println!("Todo already exists");
                }
            },
            "display" => display_todos(&todos_list),
            "update" => {
                // Prompt the user for a todo name
                let mut todo_name = String::new();
                io::stdin().read_line(&mut todo_name).expect("Invalid todo name");
                todo_name = todo_name.trim().to_string();

                // Update the todo's name and description
                let new_name = update_todo(todo_name.clone());
                if new_name != todo_name {
                    todos_list.remove(&todo_name);
                    todos_list.insert(new_name.trim().to_string(), new_name.trim().to_string());
                }
            },
            "delete" => {
                // Prompt the user for a todo name
                let mut todo_name = String::new();
                io::stdin().read_line(&mut todo_name).expect("Invalid todo name");
                todo_name = todo_name.trim().to_string();

                // Remove the todo from the todos_list HashMap and delete the corresponding file
                todos_list.remove(&todo_name);
                delete_todo(todo_name);
            },
            "md" => {
                // Prompt the user for a todo name
                let mut todo_name = String::new();
                io::stdin().read_line(&mut todo_name).expect("Invalid todo name");
                mark_done(todo_name);
            },
            "-help" => println!("create: create todo\ndisplay: display todo's\nupdate: update todo's\nmd: mark todo as done\ndelete: delete todo\nclose: close app"),
            "close" => {
                // Save the todos_list HashMap to the file and exit the loop
                update_todo_list(todos_list);
                break;
            },
            _ => println!("{} - is not a valid command", choice.trim())
        };
    }
}

fn update_todo_list(todos_list: HashMap<String, String>) {
    let mut file = File::options().write(true).truncate(true).create(true).open("todos_list.json").unwrap();

    let serialized = serde_json::to_string(&todos_list).unwrap();
    
    write!(file, "{}", serialized).expect("Error saving todo list");
}

fn create_todo(todo_name: String) -> String {
    let file_name = format!("{}.json", String::from(todo_name.trim()));
    let mut file = File::create(&file_name).unwrap();
    let todo_id = rand::random::<u16>();

    let mut todo_desc = String::new();
    println!("{}'s content/description: ", todo_name);
    io::stdin().read_line(&mut todo_desc).expect("Something isn't right about the content of your future todo");
    
    let todo = json!(
        {
            "id": todo_id,
            "name": todo_name.trim(),
            "description": todo_desc.trim(),
            "done": false
        }
    );

    let serialized_todo = serde_json::to_string(&todo).unwrap(); 
    write!(file, "{}", serialized_todo).expect("Saving to json failed");

    println!("Created todo..");
    String::from(todo_name.trim())

}

fn display_todos(todos_list: &HashMap<String, String>) {
    for todo in todos_list.iter() {
        let todo_structure = read_from_json(format!("{}.json", todo.1)).unwrap();
        println!("{}\n{}\n{}", todo_structure.name, todo_structure.description, todo_structure.done)
    }
}

fn update_todo(todo_name: String) -> String {
    let file_name = format!("{}.json", todo_name.trim());
    let mut new_name = String::new();

    if Path::new(&file_name).exists() {
        loop {
            new_name.clear();
            println!("New todo name: ");
            io::stdin().read_line(&mut new_name).expect("Invalid todo name");

            println!("New todo content/description: ");
            let mut new_content = String::new();
            let _ = stdin().read_line(&mut new_content);

            println!("New todo name: {}\nNew content:\n{}\n\nIs all correct? (Y/n)", new_name, new_content);
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).expect("Error reading answer");
            println!("{answer}");
            if answer.trim() == "Y" || answer.trim() == "y"{
                match read_from_json(file_name.clone()) {
                    Ok(mut data) => {
                        println!("Todo before changes:\nName: {}\nContent:\n{}", data.name, data.description);
                        data.name = new_name.clone();
                        data.description = new_content;
                        println!("\nTodo after changes:\nName: {}\nContent:\n{}", data.name, data.description);
                        let mut file = File::options().write(true).truncate(true).read(true).open(&file_name).unwrap();
                        
                        let todo = json!(
                            {
                                "id": data.id,
                                "name": data.name.trim(),
                                "description": data.description.trim(),
                                "done": data.done,
                            }
                        );
                    
                        let serialized_todo = serde_json::to_string(&todo).unwrap(); 
                        write!(file, "{}", serialized_todo).expect("Saving to json failed");
                        let new_file_name = format!("{}.json", data.name.trim());
                        std::fs::rename(file_name, new_file_name.clone()).expect("Something went wrong with renaming");
                    },
                    Err(e) => println!("Error reading file: {:?}", e),
                }
                break;
            }
        }
        println!("Todo updated successfully");
        new_name
    }
    else {
        println!("Such TODO doesn't exist");
        todo_name
    }
}

fn mark_done(todo_name: String) {
    let file_name = format!("{}.json", todo_name.trim());

    match read_from_json(file_name.clone()) {
        Ok(mut data) => {
            data.done = !data.done;
            let mut file = File::options().write(true).truncate(true).read(true).open(&file_name).unwrap();
            
            let todo = json!(
                {
                    "id": data.id,
                    "name": data.name,
                    "description": data.description,
                    "done": data.done,
                }
            );
        
            let serialized_todo = serde_json::to_string(&todo).unwrap(); 
            write!(file, "{}", serialized_todo).expect("Saving to json failed");
        },
        Err(e) => println!("Error reading file: {:?}", e),
    }
}

fn delete_todo(todo_name: String) {
    let file_name = format!("{}.json", todo_name.trim());

    if Path::new(&file_name).exists() {
        let _ = std::fs::remove_file(file_name);
        println!("Todo deleted successfully");
    }
    else {
        println!("Given todo name does not exist"); 
    }
}

fn read_from_json(file_path: String) -> Result<Todo> {
    let file = OpenOptions::new().write(true).read(true).open(file_path).expect("Error when connecting to file");

    let todo = serde_json::from_reader(file)?;

    Ok(todo)
}