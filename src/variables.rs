use std::{env, collections::HashMap, path::{PathBuf, Path}};
use windows::{core::PWSTR, Win32::System::Environment::GetCurrentDirectoryW};
use crate::types::Type;

#[derive(Debug)]
pub struct VariableManager {
    // built_in: HashMap<String, Type>,
    variables: HashMap<String, Type>
}

impl VariableManager {
    pub fn new() -> Self {
        let mut this = Self {
            variables: HashMap::new(),
        };

        let exe_path = env::args().nth(0).unwrap_or_default();
        let path = PathBuf::from(&exe_path);
        let script_root = match path.parent() {
            Some(s) => s.canonicalize().unwrap_or_default().to_string_lossy().to_string(),
            None => env::var("USERPROFILE").unwrap_or_default(),
        };

        let cwd = unsafe {
            let mut buffer = [0_u16; 2048];
            let size = GetCurrentDirectoryW(&mut buffer);
            String::from_utf16_lossy(&buffer[..size as usize])
        };

        this.variables.insert("cwd".into(), Type::String(cwd));
        this.variables.insert("username".into(), Type::String(env::var("USERNAME").unwrap_or_default()));
        this.variables.insert("hostname".into(), Type::String(env::var("COMPUTERNAME").unwrap_or_default()));
        this.variables.insert("domain".into(), Type::String(env::var("USERDNSDOMAIN").unwrap_or_default()));
        this.variables.insert("tmp".into(), Type::String(env::var("TEMP").unwrap_or_default()));
        this.variables.insert("script_root".into(), Type::String(script_root));

        this
    }

    pub fn get(&self, name: &str) -> &Type {
        match self.variables.get(name) {
            Some(var) => var,
            None => &Type::None,
        }
    }

    pub fn set(&mut self, name: &str, val: Type) {
        self.variables.insert(name.into(), val);
    }
}

