use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    env, fmt, fs
};

fn main() -> anyhow::Result<()> {
    let mut args = env::args_os();
    args.next().unwrap();
    let (fst, snd) = (args.next().unwrap(), args.next().unwrap());
    let files = (fs::File::open(fst)?, fs::File::open(snd)?);
    let values: (Value, Value) = (
        serde_json::from_reader(files.0)?,
        serde_json::from_reader(files.1)?
    );
    let diffs = diff_interfaces(values);
    for diff in diffs {
        println!("{}", diff);
    }
    Ok(())
}

#[derive(Debug)]
enum DiffInterface {
    Add(String),
    Remove(String),
    Change {
        name: String,
        members: Vec<DiffMember>,
        others: bool
    }
}

#[derive(Debug)]
enum DiffMember {
    Add(String),
    Remove(String),
    Change(String)
}

fn diff_interfaces((fst, snd): (Value, Value)) -> Vec<DiffInterface> {
    let dic = (collect_dic(&fst), collect_dic(&snd));
    let names: (HashSet<&str>, HashSet<&str>) = (
        dic.0.keys().cloned().collect(),
        dic.1.keys().cloned().collect()
    );
    let (added, removed, intersections) = (
        &names.1 - &names.0,
        &names.0 - &names.1,
        names.0.intersection(&names.1).cloned()
    );
    let (added, removed, changes) = (
        added.into_iter().map(|s| DiffInterface::Add(s.into())),
        removed.into_iter().map(|s| DiffInterface::Remove(s.into())),
        intersections
            .into_iter()
            .filter(|s| dic.0.get(s).unwrap() != dic.1.get(s).unwrap())
            .map(|s| {
                let (mut fst, mut snd) = (
                    (*dic.0.get(s).unwrap()).clone(),
                    (*dic.1.get(s).unwrap()).clone()
                );
                *fst.as_object_mut().unwrap().get_mut("members").unwrap() = Vec::<Value>::new().into();
                *snd.as_object_mut().unwrap().get_mut("members").unwrap() = Vec::<Value>::new().into();
                let others = fst != snd;
                let members = diff_members((dic.0.get(s).unwrap(), dic.1.get(s).unwrap()));
                DiffInterface::Change {
                    name: s.to_owned(),
                    members,
                    others
                }
            })
            //.filter(|c| matches!(c, DiffInterface::Change { members, others, .. } if !members.is_empty() || *others))
    );
    added.chain(removed).chain(changes).collect()
}

fn diff_members((fst, snd): (&Value, &Value)) -> Vec<DiffMember> {
    let members = (
        collect_dic(fst.as_object().unwrap().get("members").unwrap()),
        collect_dic(snd.as_object().unwrap().get("members").unwrap())
    );
    let names: (HashSet<&str>, HashSet<&str>) = (
        members.0.keys().cloned().collect(),
        members.1.keys().cloned().collect()
    );
    let (added, removed, intersections) = (
        &names.1 - &names.0,
        &names.0 - &names.1,
        names.0.intersection(&names.1).cloned()
    );
    let (added, removed, changes) = (
        added.into_iter().map(|s| DiffMember::Add(s.into())),
        removed.into_iter().map(|s| DiffMember::Remove(s.into())),
        intersections
            .into_iter()
            .filter(|s| members.0.get(s) != members.1.get(s))
            .map(|s| DiffMember::Change(s.into()))
    );
    added.chain(removed).chain(changes).collect()
}

fn collect_dic<'a>(v: &'a Value) -> HashMap<&'a str, &'a Value> {
    v.as_array()
        .unwrap()
        .into_iter()
        .map(|x| {
            (
                x.as_object()
                    .unwrap()
                    .get("name")
                    .unwrap()
                    .as_str()
                    .unwrap(),
                x
            )
        })
        .collect()
}

impl fmt::Display for DiffInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(name) => write!(f, "Add {}", name),
            Self::Remove(name) => write!(f, "Remove {}", name),
            Self::Change {
                name,
                members,
                others
            } => {
                let mut first_line_exists = false;
                if *others {
                    write!(f, "Change {}", name)?;
                    first_line_exists = true;
                }
                let br = |x: bool| x.then(|| "\n").unwrap_or_default();
                for c in members {
                    write!(f, "{}Change {} {}", br(first_line_exists), name, c)?;
                    first_line_exists = true;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for DiffMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(name) => write!(f, "Add {}", name),
            Self::Remove(name) => write!(f, "Remove {}", name),
            Self::Change(name) => write!(f, "Change {}", name)
        }
    }
}
