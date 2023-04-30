use crate::sun_lib::{pointer::SunPointer, sun_value::SunValue};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Table {
    array: Vec<SunPointer>,
    dict: HashMap<String, SunPointer>,
}

impl Table {
    pub fn new() -> Self {
        let t = Table {
            array: Vec::new(),
            dict: HashMap::new(),
        };
        t
    }

    pub fn copy_array(&mut self, array: Vec<SunValue>) {
        for v in array {
            self.array.push(SunPointer::new(v));
        }
    }

    pub fn replace(&mut self, i: usize, v: SunValue) {
        if let Some(elem) = self.array.get_mut(i) {
            elem.set_value(v)
        }
    }

    pub fn push(&mut self, v: SunValue) {
        self.array.push(SunPointer::new(v));
    }

    pub fn copy_dict(&mut self, dict: HashMap<String, SunValue>) {
        for (k, v) in dict {
            self.dict.insert(k.clone(), SunPointer::new(v));
        }
    }

    pub fn insert_kv(&mut self, k: String, v: SunValue) {
        if self.dict.contains_key(&k) {
            self.dict.get_mut(&k).unwrap().set_value(v);
        } else {
            self.dict.insert(k, SunPointer::new(v));
        }
    }

    pub fn get_by_key(&self, k: &str) -> Option<SunPointer> {
        if self.dict.contains_key(k) {
            Some(self.dict.get(k).unwrap().clone())
        } else {
            None
        }
    }

    pub fn get_by_index(&self, i: usize) -> Option<SunPointer> {
        if self.array.len() >= (i + 1) {
            Some(self.array.get(i).unwrap().clone())
        } else {
            None
        }
    }

    pub fn remove_by_index(&mut self, i: usize) -> Option<SunValue> {
        if self.array.len() >= (i + 1) {
            let res = self.array.remove(i).get_content();
            Some(res)
        } else {
            None
        }
    }

    pub fn remove_by_key(&mut self, k: String) -> Option<SunValue> {
        if self.dict.contains_key(&k) {
            let res = self.dict.remove(&k).unwrap().get_content();
            Some(res)
        } else {
            None
        }
    }
}

impl Hash for Table {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.array.len().hash(state);
        for p in self.array.iter() {
            p.hash(state);
        }
        self.dict.len().hash(state);
        for (k, p) in self.dict.iter() {
            k.hash(state);
            p.hash(state)
        }
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        if self.array.len() != other.array.len() {
            return false;
        }
        if self.dict.len() != other.dict.len() {
            return false;
        }
        for (self_array_value, other_array_value) in self.array.iter().zip(other.array.iter()) {
            if *self_array_value != *other_array_value {
                return false;
            }
        }
        for ((self_key, self_dict_value), (other_key, other_dict_value)) in
            self.dict.iter().zip(other.dict.iter())
        {
            if *self_key != *other_key {
                return false;
            }
            if *self_dict_value != *other_dict_value {
                return false;
            }
        }
        true
    }
}

impl Eq for Table {}

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.dict.len(), self.array.len()) {
            (0, 0) => write!(f, "<table>"),
            (_, 0) => {
                write!(f, "\n<table: ({})\n", self.dict.len())?;
                write!(f, "{{\n")?;
                let mut count = 0;
                for (key, value) in &self.dict {
                    count += 1;
                    write!(f, "{}: {}, ", key, value)?;
                    if let 5 = count {
                        write!(f, "\n")?;
                        count = 0
                    }
                }
                write!(f, "}}>\n")
            }
            (0, _) => {
                write!(f, "\n<table: ({})\n", self.array.len())?;
                write!(f, "[\n")?;
                let mut count = 0;
                for (i, item) in self.array.iter().enumerate() {
                    count += 1;
                    write!(f, "{}: {}, ", i, item)?;
                    if let 5 = count {
                        write!(f, "\n")?;
                        count = 0
                    }
                }
                write!(f, "]>\n")
            }
            (_, _) => {
                write!(f, "\n<table: ({}, {})\n", self.array.len(), self.dict.len())?;
                write!(f, "[\n")?;
                let mut count = 0;
                for (i, item) in self.array.iter().enumerate() {
                    count += 1;
                    write!(f, "{}: {}, ", i, item)?;
                    if let 5 = count {
                        write!(f, "\n")?;
                        count = 0
                    }
                }
                write!(f, "],\n")?;
                write!(f, "{{\n")?;
                for (key, value) in &self.dict {
                    count += 1;
                    write!(f, "{}: {}, ", key, value)?;
                    if let 5 = count {
                        write!(f, "\n")?;
                        count = 0
                    }
                }
                write!(f, "}}>\n")
            }
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.dict.len(), self.array.len()) {
            (0, 0) => write!(f, "table"),
            (_, 0) => {
                write!(f, "\ntable: ({})\n", self.dict.len())?;
                write!(f, "{{\n")?;
                let mut count = 0;
                for (key, value) in &self.dict {
                    count += 1;
                    write!(f, "{}: {}, ", key, value)?;
                    if let 5 = count {
                        write!(f, "\n")?;
                        count = 0
                    }
                }
                write!(f, "}}\n")
            }
            (0, _) => {
                write!(f, "\ntable: ({})\n", self.array.len())?;
                write!(f, "[\n")?;
                let mut count = 0;
                for (i, item) in self.array.iter().enumerate() {
                    count += 1;
                    write!(f, "{}: {}, ", i, item)?;
                    if let 5 = count {
                        write!(f, "\n")?;
                        count = 0
                    }
                }
                write!(f, "]\n")
            }
            (_, _) => {
                write!(f, "\ntable: ({}, {})\n", self.array.len(), self.dict.len())?;
                write!(f, "[\n")?;
                let mut count = 0;
                for (i, item) in self.array.iter().enumerate() {
                    count += 1;
                    write!(f, "{}: {}, ", i, item)?;
                    if let 5 = count {
                        write!(f, "\n")?;
                        count = 0
                    }
                }
                write!(f, "],\n")?;
                write!(f, "{{\n")?;
                for (key, value) in &self.dict {
                    count += 1;
                    write!(f, "{}: {}, ", key, value)?;
                    if let 5 = count {
                        write!(f, "\n")?;
                        count = 0
                    }
                }
                write!(f, "}}\n")
            }
        }
    }
}
