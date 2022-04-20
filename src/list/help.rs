use crate::commadline::ListOptions;

pub fn of(list: ListOptions) {
    match list {
        ListOptions::PageSize => {
            println!("{}", include_str!("_PageSize"));
        }
        ListOptions::Operations => {
            println!("{}", include_str!("_Operations"));
        }
    }
}
