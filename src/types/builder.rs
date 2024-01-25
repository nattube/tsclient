use std::{collections::{HashMap, HashSet}, any::TypeId, sync::Mutex, path::PathBuf, error::Error, fs};

use crate::FILE_HEADER;

use super::model::Component;

#[derive(Debug)]
pub struct TypeBuilder {
    pub file_map: HashMap::<String, Mutex<ComponentFileBuilder>>,
    pub rename_map: HashMap::<u64, String>,
}

impl TypeBuilder {
    pub fn build(registry: &GlobalTypeRegistry) -> Self {
        let mut builder = Self {
            file_map: HashMap::<String, Mutex<ComponentFileBuilder>>::new(),
            rename_map: HashMap::<u64, String>::new(),
        };

        for comp in &registry.components {
            comp.build(&mut builder, registry);
        }

        return builder
    }

    pub fn export_to(&self, dto_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        if !dto_path.exists() {
            fs::create_dir_all(&dto_path)?;
        }

        for (name, file) in self.file_map.iter() {
            let mut file_content = String::new();
            file_content += FILE_HEADER;

            {
                // Nasty but don't want to fight lifetimes atm.
                let file_guard = file.lock().expect("Poisened");

                for (name, renamed) in file_guard.imports.iter() {
                    let rename = match renamed {
                        Some(x) => format!(" as {}", x),
                        None => String::from(""),
                    };
                    
                    file_content += &format!("import {{type {}{}}} from \"./{}\";\n", name, rename, name);
                }

                for def in file_guard.type_defs.iter() {
                    file_content += &format!("\n{}", def);
                }

                file_content += &format!("{}", file_guard.content);
            }
            
            
            fs::write(dto_path.join(format!("{}.ts", name)), file_content)?;
        }

        Ok(())
    }

    pub fn start_file(&mut self, name: &str, hash: u64) -> Option<&Mutex<ComponentFileBuilder>> {
        let name = match self.rename_map.get(&hash) {
            Some(c) => c,
            None => name
        };

        let name = match self.file_map.get(name) {
            Some(has) => {
                let guard = has.lock().unwrap();
                if guard.hash == hash {
                    return None;
                } else {
                    let mut i = 1;
                    while let Some(_) = self.file_map.get(&format!("{}{}", name, i)) {
                        i += 1;
                    }
                    let new_name = format!("{}{}", name, i);
                    self.rename_map.insert(hash, new_name.clone());

                    new_name
                }
            }
            None => name.to_string()
        };

        let file_builder = ComponentFileBuilder::new(name.clone(), hash);
        self.file_map.insert(file_builder.name.clone(), Mutex::new(file_builder));

        return self.file_map.get(&name)
    }

    pub fn get_type_and_import(&self, name: &str, hash: u64, level: usize) -> (String, String) {
        let name = match self.rename_map.get(&hash) {
            Some(c) => c,
            None => name
        };

        let levels = (0..level).map(|_| "..").collect::<Vec<_>>().join("/");

        return match self.file_map.get(name) {
            Some(_) => {
                (name.to_string(), format!("import {{ type {} }} from \"{}/dto/{}\";", name, levels, name))
            }
            None => (name.to_string(), String::from(""))
        };
    }

    pub fn get_file(&mut self, name: &str, hash: u64) -> Option<&Mutex<ComponentFileBuilder>> {
        let name = match self.rename_map.get(&hash) {
            Some(c) => c,
            None => name
        };

        return match self.file_map.get(name) {
            Some(has) => {
                let guard = has.lock().unwrap();
                if guard.hash == hash {
                    Some(has)
                } else {
                    let mut i = 1;
                    while let Some(_) = self.file_map.get(&format!("{}{}", name, i)) {
                        i += 1;
                    }
                    let new_name = format!("{}{}", name, i);
                    self.file_map.get(&new_name)
                }
            }
            None => None
        };
    }
}

#[derive(Clone, Debug)]
pub enum HasIndexed {
    Prebuild(TypeId),
    Build(usize)
}

#[derive(Default, Debug)]
pub struct GlobalTypeRegistry {
    prebuild: Vec<TypeId>,
    hasher_stopper: Vec<TypeId>,
    hasher_hash: HashMap<TypeId, u64>,
    type_index: HashMap<TypeId, usize>,
    hash_index: HashMap<u64, usize>,
    pub components: Vec<Component>
}

impl GlobalTypeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_indexed(&self, index: &HasIndexed) -> &Component {
        let idx = match index {
            HasIndexed::Prebuild(i) => self.type_index.get(i).expect(&format!("Builder error??? Index: {:?}\n\n{:?}",index, self)),
            HasIndexed::Build(i) => i,
        };

        &self.components[*idx]
    }

    pub fn has(&self, type_id: TypeId) -> bool {
        self.prebuild.iter().find(|p| **p == type_id).is_some() || 
        self.type_index.contains_key(&type_id)
    }

    pub fn reset(&mut self) {
        self.prebuild = Vec::new();
        self.hasher_stopper = Vec::new();
        self.hasher_hash = HashMap::new();
        self.type_index = HashMap::new();
        self.hash_index = HashMap::new();

        self.components = Vec::new();
    }

    pub fn return_existing(&self, type_id: TypeId) -> Option<HasIndexed> {
        if let Some(i) = self.type_index.get(&type_id) {
            return Some(HasIndexed::Build(*i));
        } else if self.prebuild.iter().find(|p| **p == type_id).is_some() {
            return Some(HasIndexed::Prebuild(type_id));
        } else {
            return None
        }
    }

    pub fn start(&mut self, type_id: TypeId) {
        if !self.prebuild.contains(&type_id) {
            self.prebuild.push(type_id);
        }
    }

    pub fn start_hash(&mut self, type_id: TypeId) -> Option<u64> {
        if !self.hasher_stopper.contains(&type_id) {
            self.hasher_stopper.push(type_id);
        } else {
            if let Some(h) = self.hasher_hash.get(&type_id) {
                return Some(*h);
            } else {
                return Some(0);
            }
        }  

        return None
    }

    pub fn finalize_hash(&mut self, type_id: TypeId, hash: u64) {
        if !self.hasher_hash.contains_key(&type_id) {
            self.hasher_hash.insert(type_id, hash);
        }
    }

    pub fn finalize(&mut self, type_id: TypeId, component: Component) -> HasIndexed {
        let ind = self.components.len();
        let hash = component.hash;

        if let Some(i) = self.type_index.get(&type_id) {
            return HasIndexed::Build(*i);
        } /*else if let Some(i) = self.hash_index.get(&hash) {
            self.type_index.insert(type_id, *i);
            return HasIndexed::Build(*i);
        }*/

        self.components.push(component);
        self.type_index.insert(type_id, ind);
        self.hash_index.insert(hash, ind);

        return HasIndexed::Build(ind);
    }
}

#[derive(Clone, Debug)]
pub struct ComponentFileBuilder {
    pub name: String,
    pub hash: u64,
    pub imports: Vec<(String, Option<String>)>,
    pub type_defs: HashSet<String>,
    pub content: String,
    pub exports: Vec<String>
}

impl ComponentFileBuilder {
    pub fn new(name: String, hash: u64) -> Self {
        Self {
            name,
            hash,
            imports: Vec::new(),
            type_defs: HashSet::new(),
            content: String::new(),
            exports: Vec::new(),
        }
    }
}