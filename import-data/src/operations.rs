use std::path::Path;


pub fn load_xlsx<P: AsRef<Path>>(path: P) -> () {
    let input = std::fs::read_to_string(path);
    let inputstr = match input {
        Ok(str_) => str_,
        Err(_) => std::string::String::from("input string error..is path correct?"),
    };
}


#[cfg(test)]
mod test {
    #[test]
    fn test_xlsx_import() {
            super::load_xlsx("../../../data/testdata/OldExcelDocument.xlsx");
    }
}
