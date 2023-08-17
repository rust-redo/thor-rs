use std::collections::HashMap;
use std::{fs::{read_dir, File}};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Package {
  name: String,
  version: String,
  dependencies: Option<HashMap<String, String>>,
  #[serde(rename(deserialize = "devDependencies", serialize = "dev_dependencies"))]
  dev_dependencies: Option<HashMap<String, String>>,
  installed_packages: Option<Vec<Package>>
}

impl Package {
  pub(crate) fn new(root: &str) -> Result<Package, String> {
    let root_package = root.to_owned() + "package.json";
    let file = File::open(&root_package);

    match file {
      Ok(f) => match from_reader::<File, Package>(f) {
        Ok(pkg) => Ok(pkg),
        Err(e) => Err(format!("failed to read=== {}: {}", root_package, e))
      },
      Err(e) => Err(format!("failed to read {}: {}", root_package, e))
    }
  }
}



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
    use std::{env::{current_dir}, collections::HashMap};
    use crate::{package::Package};

    // #[test]
    // fn node_modules_not_exist() {
    //     let modules = get_package_list("invalid-root").unwrap_err();
    //     assert!(modules.contains("failed to read node_modules in invalid-root: No such file or directory"));
    // }

    #[test]
    fn get_root_package() {
      let package = Package::new("./").unwrap();
      let dev_dependencies = HashMap::from([
        ("@napi-rs/cli".to_owned(), "^2.16.2".to_owned()),
        ("ava".to_owned(), "^5.1.1".to_owned())
      ]);
      assert_eq!(package, Package {name: "thor-rs".to_string(), version: "0.0.0".to_string(), dependencies: None, dev_dependencies: Some(dev_dependencies), installed_packages: None});
    }
}