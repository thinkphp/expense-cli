use std::env;
use std::fs;

struct Expense {

    amount: f64,
    category: String,
}

fn save_expenses(expenses: &[Expense]) {

   let mut content = String::new();

   for expense in expenses {

       content.push_str(&format!("{}|{}\n", expense.amount, expense.category) ); 
   }

   fs::write("expense.txt", content)
   .expect("Could not save expenses.");
}

fn total_expenses(expenses: &[Expense]) -> f64 {

       expenses.iter().map(|expense| expense.amount).sum()
}

fn load_expenses() -> Vec<Expense> {

   let content = match fs::read_to_string("expense.txt") {

       Ok(content) => content,
       Err(_) => return Vec::new(),   
   };

   let mut expenses = Vec::new();

   for line in content.lines() {

        let parts: Vec<&str> = line.split("|").collect();

        if parts.len() != 2 {
            continue;
        }

        let amount:f64 = match parts[0].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let category = parts[1].to_string();

        expenses.push(Expense{
            amount, category
        });
   };

   expenses
}

fn list_expenses(expenses: &[Expense]) {

     if expenses.is_empty() {
        println!("Does not exist expenses!");
        return;
     }

     for (index, expense) in expenses.iter().enumerate() {

        println!("{}. {:.2} - {}", index+1,expense.amount, expense.category);
     }
}

fn update_expense( expenses: &mut [Expense], number: usize, amount: f64, category: String ) -> bool {

   let Some(expense) = number.checked_sub(1).and_then(|i| expenses.get_mut(i)) else {

      return false;
   };

   expense.amount = amount;

   expense.category = category ;

   true
}

fn main() {

   let args: Vec<String> = env::args().collect();

  // let mut expenses: Vec<Expense> = Vec::new();
     let mut expenses = load_expenses();

   if args.len() < 2 {

      println!("Usage: ");
      println!("  expense add <amount> <category> ");
      println!("  expense add total");
      println!("  expense update <number> <amount> <category>");
      println!("  expense list");
      return;
   } 

   match args[1].as_str() {

         "add" =>  {

               if args.len() != 4 {
                  println!("  expense add <amount> <category> ");
                  return;
               }

               let amount: f64 = match args[2].parse() {

                   Ok(value) => value,

                   Err(_) => {

                      println!("Invalid sum");

                      return;
                   }
               };

               let category = args[3].clone();  

               let expense = Expense {

                   amount, 

                   category
               };

               expenses.push(expense);

               save_expenses(&expenses);

               println!("Added expense!");
         }  

         "list"=> {

            list_expenses( &expenses );

         } 

         "total" => {

                println!("Total expenses: {:.2}", total_expenses( &expenses ));
         }

         "update" => {

               if args.len() != 5 {
                  println!("Usage: expense update <number> <amount> <category>");
                  return;
               }

               let number: usize = match args[2].parse() {
                  Ok(value) => value,
                  Err(_) => {
                     println!("Invalid value");
                     return;
                  }
               };

               let amount:f64 = match args[3].parse() {
                  Ok(value) => value,
                  Err(_) => {
                     println!("Invalid value");
                     return;
                  }
               };

               if update_expense(&mut expenses, number, amount, args[4].clone()) {

                  save_expenses(&expenses);
                  println!("Expense updated!");

               } else {

                  println!("Expense number not found.");
               }
         }

         _ => {
            println!("Invalid command");
         }
   }

}
