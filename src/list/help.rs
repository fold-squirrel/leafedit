use crate::commadline::ListOptions;

pub fn of(list: ListOptions) {
    match list {
        ListOptions::PageSize => {
            println!("{}", include_str!("_PageSizes"));
        }
        ListOptions::Operations => {
            println!("{}", include_str!("_Operations"));
        }
        ListOptions::GridType => {
            println!("{}", include_str!("_GridTypes"));
        }
    }
}
