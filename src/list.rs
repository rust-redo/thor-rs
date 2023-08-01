use std::fs::{read_dir, read};

// use core::result;

#[derive(PartialEq, Debug)]
pub struct Module {
  pub name: String,
  pub version: String
}

// impl PartialEq for Module {
//   fn eq(&self, other: &Self) -> bool {
//       self.name == other.name && 
//   }
// }

pub fn get_module_list(root: &str) -> Result<Vec<Module>, &str>  {
  let node_modules = read_dir(root.to_owned() + "/node_modules").unwrap();
  let modules:Vec<Module> = Vec::new();
  for m in node_modules { 
    let mut path = m.unwrap().path();
    path.push("/package.json");
    let data = read(path).unwrap().to_owned();
  }

  Result::Ok(modules)
}

#[cfg(test)]
mod tests {
    use crate::list::get_module_list;

    #[test]
    fn node_modules_not_exist() {
        let modules = get_module_list("never");
        assert_eq!(modules, Err("xxxx"));
    }
}