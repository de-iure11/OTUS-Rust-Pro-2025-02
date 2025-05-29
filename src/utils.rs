use crate::smart_house::Report;

pub fn print_report(obj: &impl Report) {
    println!("{}", obj.get_report());
}
