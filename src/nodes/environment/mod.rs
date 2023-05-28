use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Env<T: Clone> {
    stack: Vec<HashMap<String, T>>,
}

impl<T: Clone> Env<T> {
    pub fn get_stack(&self) -> &Vec<HashMap<String, T>> {
        &self.stack
    }
}

impl<T: Clone> Env<T> {
    pub fn new() -> Self {
        Self {
            stack: vec![HashMap::new()],
        }
    }

    pub fn top(self: &mut Self) -> Option<&mut HashMap<String, T>> {
        self.stack.last_mut()
    }

    pub fn push(self: &mut Self) {
        self.stack.push(HashMap::new())
    }

    pub fn pop(self: &mut Self) {
        self.stack.pop();
    }

    pub fn get<S: Into<String>>(self: &Self, name: S) -> Option<T> {
        let name = &name.into();
        for ele in &self.stack {
            if let Some(v) = ele.get(name) {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn add<S: Into<String>>(self: &mut Self, name: S, value: T) {
        if let Some(ks) = self.top() {
            ks.insert(name.into(), value);
        }
    }

    pub fn add_all<S: Into<String>>(self: &mut Self, xs: Vec<(S, T)>) {
        if let Some(ks) = self.top() {
            for (k, v) in xs {
                ks.insert(k.into(), v);
            }
        }
    }
}
