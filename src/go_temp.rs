use card_format::CData;
//use gtmpl::Template;
use gtmpl_value::{Number, Value};
use std::collections::{BTreeMap, HashMap};

pub fn as_go_v(dt: &CData) -> Value {
    match dt {
        CData::S(s) | CData::R(s) => Value::String(s.to_string()),
        CData::N(i) => Value::Number(Number::from(*i)),
        CData::L(l) => Value::Array(l.into_iter().map(|v| as_go_v(v)).collect()),
    }
}

pub struct CWH<'a> {
    name: &'a str,
    n: usize,
    n_of_type: usize,
    w: f64,
    h: f64,
    data: &'a BTreeMap<String, CData>,
}
impl<'a> CWH<'a> {
    pub fn new(
        name: &'a str,
        w: f64,
        h: f64,
        n: usize,
        n_of_type: usize,
        data: &'a BTreeMap<String, CData>,
    ) -> Self {
        CWH {
            name,
            w,
            h,
            n,
            n_of_type,
            data,
        }
    }
}

impl<'a> Into<Value> for CWH<'a> {
    fn into(self) -> Value {
        let mut rmap = HashMap::new();
        rmap.insert("name".to_string(), Value::String(self.name.to_string()));
        rmap.insert("w".to_string(), Value::Number(self.w.into()));
        rmap.insert("h".to_string(), Value::Number(self.h.into()));
        rmap.insert("n".to_string(), Value::Number(self.n.into()));
        rmap.insert(
            "n_of_type".to_string(),
            Value::Number(self.n_of_type.into()),
        );

        let mut data = HashMap::new();
        for (k, v) in self.data {
            data.insert(k.to_string(), as_go_v(v));
        }
        rmap.insert("data".to_string(), Value::Map(data));

        let res = Value::Map(rmap);

        res
    }
}
