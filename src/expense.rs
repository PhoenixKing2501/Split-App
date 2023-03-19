use anyhow::{Error, Result};
use chrono::{/* serde::ts_nanoseconds, */ DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::{
	collections::{HashMap, HashSet},
	fmt,
	fs::OpenOptions,
	path::PathBuf,
	vec,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record {
	pub name: String,

	// #[serde(with = "ts_nanoseconds")]
	pub created_at: DateTime<Utc>,

	// #[serde(with = "ts_nanoseconds")]
	pub modified_at: DateTime<Utc>,

	pub expenses: Vec<Expense>,
}

impl Record {
	pub fn new(name: &String) -> Self {
		Self {
			name: name.to_owned(),
			created_at: Utc::now(),
			modified_at: Utc::now(),
			expenses: vec![],
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Expense {
	Share {
		who: String,
		amount: f32,

		// #[serde(with = "ts_nanoseconds")]
		created_at: DateTime<Utc>,
	},
	Lend {
		from: String,
		to: String,
		amount: f32,

		// #[serde(with = "ts_nanoseconds")]
		created_at: DateTime<Utc>,
	},
	SettleUp {
		// #[serde(with = "ts_nanoseconds")]
		created_at: DateTime<Utc>,
	},
}

impl fmt::Display for Expense {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Share {
				who,
				amount,
				created_at,
			} => write!(
				f,
				"[{}] {who} pays \u{20B9}{amount:.2}",
				created_at.with_timezone(&Local).format("%a %b %d %Y %r")
			),
			Self::Lend {
				from,
				to,
				amount,
				created_at,
			} => write!(
				f,
				"[{}] {from} pays {to} \u{20B9}{amount:.2}",
				created_at.with_timezone(&Local).format("%a %b %d %Y %r")
			),
			Self::SettleUp { created_at } => {
				write!(
					f,
					"[{}] Settled Up!",
					created_at.with_timezone(&Local).format("%a %b %d %Y %r")
				)
			},
		}
	}
}

fn get_records(path: &PathBuf) -> Result<Vec<Record>> {
	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open(path)?;

	let records = match serde_json::from_reader(file) {
		Ok(records) => records,
		Err(e) if e.is_eof() => Vec::new(),
		Err(e) => Err(e)?,
	};

	Ok(records)
}

fn save_records(path: &PathBuf, records: Vec<Record>) -> Result<()> {
	let file = OpenOptions::new().write(true).open(path)?;
	file.set_len(0)?;
	serde_json::to_writer(file, &records)?;
	Ok(())
}

fn get_record<'a>(
	records: &'a mut Vec<Record>,
	name: &String,
	create: bool,
) -> Result<&'a mut Record> {
	match records.iter().position(|record| record.name.eq(name)) {
		Some(pos) => Ok(&mut records[pos]),
		None => {
			if !create {
				Err(Error::msg("Record doesn't exist!"))
			} else {
				records.push(Record::new(&name));
				let len = records.len();
				Ok(&mut records[len - 1])
			}
		},
	}
}

pub fn index_records(path: &PathBuf) -> Result<()> {
	let records = get_records(path)?;

	records.iter().rev().for_each(|record| {
		println!(
			"Created At: [{}], Last Modified At: [{}], Record: \"{}\"",
			record
				.created_at
				.with_timezone(&Local)
				.format("%a %b %d %Y %r"),
			record
				.modified_at
				.with_timezone(&Local)
				.format("%a %b %d %Y %r"),
			record.name
		);
	});

	Ok(())
}

pub fn delete_record(path: &PathBuf, name: &String) -> Result<()> {
	let mut records = get_records(path)?;

	match records.iter().position(|elem| elem.name.eq(name)) {
		Some(index) => {
			records.remove(index);
		},
		None => Err(Error::msg("Record doesn't exist!"))?,
	}

	save_records(path, records)
}

pub fn share_expense(path: &PathBuf, name: &String, who: &String, amount: f32) -> Result<()> {
	let mut records = get_records(path)?;
	let record = get_record(&mut records, name, true)?;

	let created_at = Utc::now();
	record.modified_at = created_at;
	record.expenses.push(Expense::Share {
		who: who.to_owned(),
		amount,
		created_at,
	});

	save_records(path, records)
}

pub fn lend_expense(
	path: &PathBuf,
	name: &String,
	from: &String,
	to: &String,
	amount: f32,
) -> Result<()> {
	let mut records = get_records(path)?;
	let record = get_record(&mut records, name, true)?;

	let created_at = Utc::now();
	record.modified_at = created_at;
	record.expenses.push(Expense::Lend {
		from: from.to_owned(),
		to: to.to_owned(),
		amount,
		created_at,
	});

	save_records(path, records)
}

pub fn settle_up(path: &PathBuf, name: &String) -> Result<()> {
	let mut records = get_records(path)?;
	let record = get_record(&mut records, name, false)?;

	let created_at = Utc::now();
	record.modified_at = created_at;
	record.expenses.push(Expense::SettleUp { created_at });

	save_records(path, records)
}

pub fn list_expenses(path: &PathBuf, name: &String, in_order: bool) -> Result<()> {
	let mut records = get_records(path)?;
	let record = get_record(&mut records, name, false)?;

	let digits = ((record.expenses.len() as f64).log10() + 1.0) as usize;

	if in_order {
		record.expenses.iter().enumerate().for_each(|(i, expense)| {
			println!("{:>digits$} => {}", i + 1, expense);
		});
	} else {
		record
			.expenses
			.iter()
			.rev()
			.enumerate()
			.for_each(|(i, expense)| {
				println!("{:>digits$} => {}", i + 1, expense);
			});
	};

	Ok(())
}

pub fn show_balance(path: &PathBuf, name: &String) -> Result<()> {
	let mut records = get_records(path)?;
	let record = get_record(&mut records, name, false)?;

	let mut people: HashSet<String> = HashSet::new();

	record.expenses.iter().for_each(|expense| {
		if let Expense::Share { who, .. } = expense {
			people.insert(who.to_owned());
		} else if let Expense::Lend { from, to, .. } = expense {
			people.insert(from.to_owned());
			people.insert(to.to_owned());
		}
	});

	let people: HashMap<String, usize> = people
		.into_iter()
		.enumerate()
		.map(|(i, name)| (name, i))
		.collect();

	let len = people.len();
	let mut balance = vec![vec![0.0_f32; len]; len];

	record.expenses.iter().for_each(|expense| match expense {
		Expense::Share { who, amount, .. } => {
			let share = amount / len as f32;
			balance.iter_mut().for_each(|bal| bal[people[who]] += share);
			balance[people[who]]
				.iter_mut()
				.for_each(|bal| *bal -= share);
		},
		Expense::Lend {
			from, to, amount, ..
		} => {
			balance[people[to]][people[from]] += amount;
			balance[people[from]][people[to]] -= amount;
		},
		Expense::SettleUp { .. } => balance = vec![vec![0.0_f32; len]; len],
	});

	println!("people = {:#?}", people);
	balance.iter().for_each(|bal| println!("{bal:?}"));

	Ok(())
}
