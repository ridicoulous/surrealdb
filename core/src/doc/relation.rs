use crate::ctx::Context;
<<<<<<< HEAD:lib/src/doc/relation.rs
use crate::dbs::Statement;
use crate::dbs::{Options, Transaction};
use crate::doc::Document;
use crate::err::Error;
use crate::sql::TableType;
=======
use crate::dbs::Options;
use crate::dbs::{Statement, Workable};
use crate::doc::Document;
use crate::err::Error;
>>>>>>> main:core/src/doc/relation.rs

impl<'a> Document<'a> {
	pub async fn relation(
		&mut self,
<<<<<<< HEAD:lib/src/doc/relation.rs
		_ctx: &Context<'_>,
		opt: &Options,
		txn: &Transaction,
		stm: &Statement<'_>,
	) -> Result<(), Error> {
		let table_type = if let Statement::Relate(_) = stm {
			TableType::Relation(Default::default())
		} else {
			TableType::Normal
		};
		// TODO: Implicit table definition doesn't define in/out fields
		let tb = self.tb_with_rel(opt, txn, table_type).await?;

		let rid = self.id.as_ref().unwrap();
		match stm {
			Statement::Create(_) | Statement::Insert(_) => {
=======
		ctx: &Context<'_>,
		opt: &Options,
		stm: &Statement<'_>,
	) -> Result<(), Error> {
		let tb = self.tb(ctx, opt).await?;

		let rid = self.id.as_ref().unwrap();
		match stm {
			Statement::Create(_) => {
>>>>>>> main:core/src/doc/relation.rs
				if !tb.allows_normal() {
					return Err(Error::TableCheck {
						thing: rid.to_string(),
						relation: false,
<<<<<<< HEAD:lib/src/doc/relation.rs
=======
						target_type: tb.kind.clone(),
					});
				}
			}
			Statement::Upsert(_) => {
				if !tb.allows_normal() {
					return Err(Error::TableCheck {
						thing: rid.to_string(),
						relation: false,
						target_type: tb.kind.clone(),
>>>>>>> main:core/src/doc/relation.rs
					});
				}
			}
			Statement::Relate(_) => {
				if !tb.allows_relation() {
					return Err(Error::TableCheck {
						thing: rid.to_string(),
						relation: true,
<<<<<<< HEAD:lib/src/doc/relation.rs
					});
				}
			}
=======
						target_type: tb.kind.clone(),
					});
				}
			}
			Statement::Insert(_) => match self.extras {
				Workable::Relate(_, _, _) => {
					if !tb.allows_relation() {
						return Err(Error::TableCheck {
							thing: rid.to_string(),
							relation: true,
							target_type: tb.kind.clone(),
						});
					}
				}
				_ => {
					if !tb.allows_normal() {
						return Err(Error::TableCheck {
							thing: rid.to_string(),
							relation: false,
							target_type: tb.kind.clone(),
						});
					}
				}
			},
>>>>>>> main:core/src/doc/relation.rs
			_ => {}
		}
		// Carry on
		Ok(())
	}
}
