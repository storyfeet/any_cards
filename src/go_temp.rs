use card_format::CData;
use gtmpl::Template;
use gtmpl_value::{Number, Value};
use std::collections::{BTreeMap, HashMap};

pub fn as_go_v(dt: &CData) -> Value {
    match dt {
        CData::S(s) | CData::R(s) => Value::String(s.to_string()),
        CData::N(i) => Value::Number(Number::from(*i)),
        CData::L(l) => Value::Array(l.into_iter().map(|v| as_go_v(v)).collect()),
    }
}

pub fn go_value(name: &str, w: f64, h: f64, map: &BTreeMap<String, CData>) -> Value {
    let mut rmap = HashMap::new();
    rmap.insert("name".to_string(), Value::String(name.to_string()));
    rmap.insert("w".to_string(), Value::Number(w.into()));
    rmap.insert("h".to_string(), Value::Number(h.into()));
    let mut data = HashMap::new();
    for (k, v) in map {
        data.insert(k.to_string(), as_go_v(v));
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
    res.add_func("xywh", xywh);
    res.add_func("fl_stk", fl_stk);
    res.add_func("b_sel", b_sel);
    res.add_func("fnt", fnt);
    res.add_func("ccat", ccat);
    res.add_func("xml_es", xml_es);
    res.add_func("sep", sep);
    res.add_func("is_list", is_list);
    res.add_func("s_len", s_len);
    res.add_func("as_list", as_list);
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

pub fn b_sel(args: &[Value]) -> Result<Value, String> {
    let b_val = match args.get(0) {
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => (*n) >= Number::from(0),
        Some(Value::NoValue) | Some(Value::Nil) => false,
        Some(Value::String(s)) => s.len() != 0,
        Some(Value::Array(a)) => a.len() != 0,
        Some(Value::Map(m)) => m.len() != 0,
        _ => return Err("First Expr must be bool or Num".to_string()),
    };

    if b_val {
        return args
            .get(1)
            .map(|m| m.clone())
            .ok_or("Ok Expression not supplied".to_string());
    }
    Ok(args
        .get(2)
        .map(|v| v.clone())
        .unwrap_or(Value::String(String::new())))
}

pub fn xywh(args: &[Value]) -> Result<Value, String> {
    let h = args.get(3).ok_or("H not supplied".to_string())?;
    let w = args.get(2).ok_or("W not supplied".to_string())?;
    let y = args.get(1).ok_or("Y not supplied".to_string())?;
    let x = args.get(0).ok_or("X not supplied".to_string())?;
    Ok(Value::String(format!(
        r#"x="{}px" y="{}px" width="{}px" height="{}px" "#,
        x, y, w, h
    )))
}
pub fn fl_stk(args: &[Value]) -> Result<Value, String> {
    let f = args.get(0).ok_or("Fill not supplied".to_string())?;
    let s = args.get(1).ok_or("Stroke not supplied".to_string())?;
    let w = args.get(2).ok_or("StrokeWidth not supplied".to_string())?;
    //TODO add px only for numbers
    Ok(Value::String(format!(
        r#"fill="{}" stroke="{}" stroke-width="{}px" "#,
        f, s, w
    )))
}

pub fn fnt(args: &[Value]) -> Result<Value, String> {
    let sz = args.get(0).ok_or("Font Size not supplied".to_string())?;
    let ff = args
        .get(1)
        .map(|s| format!(r#"font-family="{}" "#, s))
        .unwrap_or(String::new());

    Ok(Value::String(format!(r#"font-size="{}px" {}"#, sz, ff)))
}

fn _xml_es(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        match c {
            '&' => res.push_str("&amp;"),
            '>' => res.push_str("&gt;"),
            '<' => res.push_str("&lt;"),
            '\"' => res.push_str("&quot;"),
            '\'' => res.push_str("&apos;"),
            cv => res.push(cv),
        }
    }
    res
}

pub fn xml_es(args: &[Value]) -> Result<Value, String> {
    let mut res = String::new();
    for a in args {
        res.push_str(&_xml_es(&a.to_string()));
    }
    Ok(Value::String(res))
}

pub fn ccat(args: &[Value]) -> Result<Value, String> {
    let mut res = String::new();
    for v in args {
        res.push_str(&v.to_string());
    }
    Ok(Value::String(res))
}

fn _sep(sp: &str, args: &[Value]) -> Result<String, String> {
    let mut first = true;
    let mut res = String::new();
    for a in args {
        if !first {
            res.push_str(sp);
        }
        match a {
            Value::Array(l) => res.push_str(&_sep(sp, l)?),
            v => res.push_str(&v.to_string()),
        }

        first = false;
    }
    Ok(res)
}

pub fn is_list(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("Is list expected 1 elem, got{}", args.len()));
    }
    Ok(Value::Bool(match args[0] {
        Value::Map(_) | Value::Array(_) => true,
        _ => false,
    }))
}

pub fn v_as_list(v: &Value) -> Vec<Value> {
    match v {
        Value::Map(m) => m.values().map(|v| v.clone()).collect(),
        Value::Array(l) => l.iter().map(|v| v.clone()).collect(),
        Value::Nil | Value::NoValue => Vec::new(),
        _ => vec![v.clone()],
    }
}

pub fn as_list(args: &[Value]) -> Result<Value, String> {
    if args.len() == 0 {
        return Err("'as_list' expected at least 1 elem".to_string());
    }
    let mut res = Vec::new();
    for a in args {
        res.extend(v_as_list(a));
    }
    Ok(Value::Array(res))
}

// first arg is seperator, after flatten
pub fn sep(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Nothing to separate".to_string());
    }
    let sp = args[0].to_string();
    Ok(Value::String(_sep(&sp, &args[1..])?))
}

pub fn s_len(args: &[Value]) -> Result<Value, String> {
    let mut res = 0;
    for a in args {
        res += match a {
            Value::Map(m) => m.len(),
            Value::Array(l) => l.len(),
            _ => 0,
        }
    }
    Ok(Value::Number(Number::from(res)))
}
