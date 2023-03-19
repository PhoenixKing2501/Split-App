use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
	name = "Split App",
	about = "A Command Line App to Split Expenses Among Friends"
)]
pub enum CmdArgs {
	/// List all records
	Index,

	/// Add expense of a person
	Share {
		/// Name of record
		#[structopt(short, long)]
		name: String,

		/// Person number
		#[structopt()]
		person: String,

		/// Expense of that person
		#[structopt()]
		amount: f32,
	},

	/// Give an amount from a person to another person
	Lend {
		/// Name of record
		#[structopt(short, long)]
		name: String,

		/// Who is giving
		#[structopt()]
		from: String,

		/// Who is receiving
		#[structopt()]
		to: String,

		/// Amount given / received
		#[structopt(default_value = "0.0")]
		amount: f32,
	},

	/// Shows the current pending balance
	Show {
		/// Name of record
		#[structopt(short, long)]
		name: String,
	},

	/// Settle up the amount
	SettleUp {
		/// Name of record
		#[structopt(short, long)]
		name: String,
	},

	/// Lists all expenses of a record
	List {
		/// Name of record
		#[structopt(short, long)]
		name: String,

		/// Display the list in chronological order.
		/// Default order is recent first
		#[structopt(short = "o", long)]
		in_order: bool,
	},

	/// Delete a record
	Delete {
		/// Name of record
		#[structopt(short, long)]
		name: String,
	},
}
