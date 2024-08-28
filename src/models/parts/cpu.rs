use crate::diesel_schema::parts::cpu::cpu;
use bigdecimal::BigDecimal;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = cpu)]
pub struct CPU {
    pub id: i32,
    pub name: String,
    pub price: BigDecimal,
    pub core_count: i32,
    pub core_clock: String,
    pub boost_clock: String,
    pub tdp: i32,
    pub integrated_graphics: Option<String>,
    pub smt: bool,
}
