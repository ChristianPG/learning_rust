use std::{collections::HashMap, io};

fn main() {
    let mut company_employees: HashMap<String, Vec<String>> = Default::default();

    loop {
        println!("========================================");
        println!("Employee Database");
        println!("Select an option:");
        println!("1. Add an employee");
        println!("2. List the employees in a department");
        println!("3. List all the employees in the company");
        println!("========================================");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Please select an option");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        if option == 1 {
            add_employee(&mut company_employees)
        } else if option == 2 {
            list_department_employees(&mut company_employees);
        } else if option == 3 {
            list_all_employees(&mut company_employees);
        }
    }
}

fn add_employee(employee_database: &mut HashMap<String, Vec<String>>) {
    println!("Enter the name of the employee:");
    let mut employee_name = String::new();
    io::stdin()
        .read_line(&mut employee_name)
        .expect("Please enter a name for the employee");
    let employee_name = employee_name.trim();

    println!("Enter the name of the department:");
    let mut department_name = String::new();
    io::stdin()
        .read_line(&mut department_name)
        .expect("Please enter a name for the department");
    let department_name = department_name.trim();

    println!("Adding {employee_name} to {department_name}");
    let department_list = employee_database
        .entry(department_name.to_string())
        .or_insert(vec![]);

    department_list.push(employee_name.to_string());
    println!("{employee_name} was successfully added to {department_name}");
}

fn list_department_employees(employee_database: &mut HashMap<String, Vec<String>>) {
    println!("Enter the name of the department:");
    let mut department_name = String::new();
    io::stdin()
        .read_line(&mut department_name)
        .expect("Please enter a name for the department");
    let department_name = department_name.trim();

    match employee_database.get_mut(department_name) {
        Some(department_employees) => {
            println!("Here is the list of employees in {department_name}:");
            department_employees.sort();
            for employee_name in department_employees.iter() {
                println!("{employee_name}");
            }
        }
        None => {
            println!("The department {department_name} does not exist.");
        }
    }
}

fn list_all_employees(employee_database: &mut HashMap<String, Vec<String>>) {
    println!("These are all the employees grouped by department:");
    for (department_name, department_employees) in employee_database {
        println!("************ {department_name} ************");
        department_employees.sort();
        for employee_name in department_employees.iter() {
            println!("{employee_name}");
        }
    }
}
