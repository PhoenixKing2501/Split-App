mod cli;
mod expense;

use cli::CmdArgs;
use expense::{
	delete_record, index_records, lend_expense, list_expenses, settle_up, share_expense,
};
use structopt::StructOpt;

use crate::expense::show_balance;

fn main() -> anyhow::Result<()> {
	let opt = CmdArgs::from_args();
	let mut path = std::env::current_dir()?;
	path.push("data");
	path.push("index.json");
	let path = path;

	match opt {
		CmdArgs::Index => {
			println!("List all records");
			index_records(&path)
		},

		CmdArgs::Delete { name } => {
			println!("Deleted record {name}");
			delete_record(&path, &name)
		},

		CmdArgs::List { name, in_order } => {
			println!("All expenses");
			list_expenses(&path, &name, in_order)
		},

		CmdArgs::Share {
			name,
			person,
			amount,
		} => {
			println!("{person} added \u{20B9}{amount:.2}");
			share_expense(&path, &name, &person, amount)
		},
		CmdArgs::Lend {
			name,
			from,
			to,
			amount,
		} => {
			println!("{from} gave {to} \u{20B9}{amount:.2}");
			lend_expense(&path, &name, &from, &to, amount)
		},

		CmdArgs::SettleUp { name } => {
			println!("All settled up!");
			settle_up(&path, &name)
		},
		// _ => println!("Other CmdArgs"),
		CmdArgs::Show { name } => {
			println!("Pending balance");
			show_balance(&path, &name)
		},
	}

	// println!("Enter Number of People: ");

	// let mut n = String::new();
	// stdin().read_line(&mut n).expect("Error reading n");
	// let n: usize = n.trim().parse().expect("Expected a number");

	// let mut people = vec![0f64; n];

	// loop {
	// 	println!("Enter person number and expense: ");

	// 	let mut input = String::new();
	// 	stdin().read_line(&mut input).expect("Error reading input");

	// 	let mut input = input.trim().split(' ');

	// 	let expense: (usize, f64) = (
	// 		input
	// 			.next()
	// 			.unwrap()
	// 			.trim()
	// 			.parse()
	// 			.expect("Expected person number"),
	// 		input
	// 			.next()
	// 			.unwrap_or("0")
	// 			.trim()
	// 			.parse()
	// 			.expect("Expected expense"),
	// 	);

	// 	if expense.0 == 0 {
	// 		break;
	// 	} else if expense.0 > people.len() {
	// 		println!("Invalid person number! Try Again!");
	// 		continue;
	// 	}

	// 	println!("{:?}", expense);

	// 	people[expense.0 - 1] += expense.1;
	// }

	// println!("{:?}", people);

	// let sum: f64 = people.iter().sum();
	// let share = sum / people.len() as f64;

	// println!("Share is {:.2}", share);

	// let dues: Vec<f64> = people.iter().map(|exp| exp - share).collect();
	// println!("[{:.2},{:.2}]", dues[0], dues[1]);
}
