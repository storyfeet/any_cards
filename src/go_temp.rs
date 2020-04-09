use card_format::CData;
use gtmpl::Template;
use gtmpl_value::{Number, Value};
use std::collections::{BTreeMap, HashMap};

pub fn go_value(name: &str, w: f64, h: f64, map: &BTreeMap<String, CData>) -> Value {
    let mut rmap = HashMap::new();
    rmap.insert("name".to_string(), Value::String(name.to_string()));
    rmap.insert("w".to_string(), Value::Number(w.into()));
    rmap.insert("h".to_string(), Value::Number(h.into()));
    let mut data = HashMap::new();
    for (k, v) in map {
        match v {
            CData::S(s) | CData::R(s) => {
                data.insert(k.to_string(), Value::String(s.to_string()));
            }
            CData::N(i) => {
                data.insert(k.to_string(), Value::Number((*i).into()));
            }
        }
    }
    rmap.insert("data".to_string(), Value::Map(data));

    let res = Value::Map(rmap);

    res
}

pub struct CWH<'a> {
    name: &'a str,
    w: f64,
    h: f64,
    data: &'a BTreeMap<String, CData>,
}
impl<'a> CWH<'a> {
    pub fn new(name: &'a str, w: f64, h: f64, data: &'a BTreeMap<String, CData>) -> Self {
        CWH { name, w, h, data }
    }
}

impl<'a> Into<Value> for CWH<'a> {
    fn into(self) -> Value {
        go_value(self.name, self.w, self.h, self.data)
    }
}

pub fn helper_template() -> Template {
    let mut res = Template::default();
    res.add_func("mul", mul);
    res.add_func("add", add);
    res.add_func("wrap", wrap);
    res
}

pub fn mul(args: &[Value]) -> Result<Value, String> {
    let mut res = Number::from(1);
    for a in args {
        if let Value::Number(n2) = a {
            if let (Some(f1), Some(f2)) = (res.as_f64(), n2.as_f64()) {
                res = Number::from(f1 * f2);
            } else if let (Some(i1), Some(i2)) = (res.as_i64(), n2.as_i64()) {
                res = Number::from(i1 * i2);
            } else if let (Some(u1), Some(u2)) = (res.as_u64(), n2.as_u64()) {
                res = Number::from(u1 * u2);
            } else {
                return Err("Numbers not compatible".to_string());
            }
        } else {
            return Err(format!("not a number {:?}", a));
        }
    }
    Ok(Value::Number(res))
}

pub fn add(args: &[Value]) -> Result<Value, String> {
    let mut res = Number::from(0);
    for a in args {
        if let Value::Number(n2) = a {
            if let (Some(f1), Some(f2)) = (res.as_f64(), n2.as_f64()) {
                res = Number::from(f1 + f2);
            } else if let (Some(i1), Some(i2)) = (res.as_i64(), n2.as_i64()) {
                res = Number::from(i1 + i2);
            } else if let (Some(u1), Some(u2)) = (res.as_u64(), n2.as_u64()) {
                res = Number::from(u1 + u2);
            } else {
                return Err("Numbers not compatible".to_string());
            }
        } else {
            return Err(format!("not a number {:?}", a));
        }
    }
    Ok(Value::Number(res))
}
pub fn wrap(args: &[Value]) -> Result<Value, String> {
    let s = match args.get(0) {
        Some(Value::String(s)) => s,
        _ => return Err("Value 0 not a string".to_string()),
    };
    let n = match args.get(1) {
        Some(Value::Number(n)) => n.as_u64().ok_or("Value 1 not a positive int".to_string())?,
        _ => return Err("Value 1 not a num".to_string()),
    };
    let vs = mksvg::text::wrap(s, n as usize);
    Ok(Value::Array(
        vs.into_iter().map(|v| Value::String(v)).collect(),
    ))
}
