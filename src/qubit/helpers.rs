use super::model::definition::Qubit;

pub fn print_qubit(qb: &Qubit) {
    println!("zero part is {:?}", qb.zer_part);
    println!("one part is {:?}", qb.one_part);
}