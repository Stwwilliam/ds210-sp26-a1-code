use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ColumnType {
    String,
    Integer,
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum Value {
    String(String),
    Integer(i32),
}
impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::String(value) => value.to_string(),
            Value::Integer(value) => value.to_string(),
        }
    }
}

pub struct Row {
    values: Vec<Value>,
}
impl Row {
    pub fn new(values: Vec<Value>) -> Row {
        return Row { values };
    }
    pub fn get_values(&self) -> &Vec<Value> {
        return &self.values;
    }
    pub fn get_value(&self, index: usize) -> &Value {
        return &self.values[index];
    }
    pub fn move_values(self) -> Vec<Value> {
        return self.values;
    }
}

pub struct Dataset {
    columns: Vec<(String, ColumnType)>,
    rows: Vec<Row>,
}
impl Dataset {
    pub fn new(columns: Vec<(String, ColumnType)>) -> Dataset {
        return Dataset {
            columns,
            rows: Vec::new(),
        };
    }
    pub fn add_row(&mut self, values: Vec<Value>) {
        self.rows.push(Row::new(values));
    }

    pub fn get_row(&self, i: usize) -> &Row {
         return &self.rows[i];
    }
    pub fn len(&self) -> usize {
        return self.columns.len();
    }
    pub fn columns(&self) -> &Vec<(String, ColumnType)> {
        return &self.columns;
    }
    pub fn column_type(&self, column_name: &String) -> &ColumnType {
       let i = self.column_index(column_name);
        return &self.columns[i].1;
    }
    pub fn column_index(&self, column_name: &String) -> usize {
        for i in 0..self.columns.len() {
            let (cname, _ctype) = &self.columns[i];
            if cname == column_name {
                return i;
            }
        }
        panic!("Column {} not found", column_name);
    }
}

impl Debug for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Dataset:")?;
        write!(f, "|")?;
        for (colname, coltype) in &self.columns {
            let description = format!("{colname}: {coltype:?}");
            write!(f, " {description: <25}|")?;
        }
        writeln!(f, "")?;

        write!(f, "|")?;
        for _ in &self.columns {
            write!(f, "==========================|")?;
        }
        writeln!(f, "")?;

        for row in &self.rows {
            write!(f, "|")?;
            for value in row.get_values() {
                write!(f, " {: <25}|", value.to_string())?;
            }
            writeln!(f, "")?;
        }
        return Ok(());
    }
}
impl Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return Debug::fmt(self, f);
    }
}