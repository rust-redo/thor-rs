use std::{fs::{read_dir, File}};
use crate::package::Package;

pub fn get_package_list(root: &str) -> Result<Vec<Package>, String>  {
  let node_modules;
  let real_root = root.to_owned();

  match read_dir(real_root + "/node_modules") {
    Ok(n) => node_modules = n,
    Err(err) => {
      return Err(format!("failed to read node_modules in {}: {}" , root , err));
    }
  }

  let mut packages:Vec<Package> = Vec::new();
  for m in node_modules { 
    let mut path = m.unwrap().path();
    path.push("package.json");

    match File::open(path) {
      Ok(file) => {
        let pkg: Package = serde_json::from_reader(file).unwrap();
        packages.push(pkg);
      },
      Err(_) => {continue;}
    }
  }

  Ok(packages)
}

#[cfg(test)]
mod tests {
    use std::env::{current_dir};
    use crate::{list::get_package_list, package::Package};

    #[test]
    fn node_modules_not_exist() {
        let modules = get_package_list("invalid-root").unwrap_err();
        assert!(modules.contains("failed to read node_modules in invalid-root: No such file or directory"));
    }

    #[test]
    fn should_work() {
      
      let modules = get_package_list(current_dir().unwrap().to_str().unwrap()).unwrap();
      assert_eq!(modules, [Package {name: "x".to_string(), version: "0.1.0".to_string()}])
    }
}