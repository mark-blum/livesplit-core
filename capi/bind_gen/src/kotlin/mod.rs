use std::path::Path;
use std::collections::BTreeMap;
use {Class, Result};
use std::fs::{create_dir_all, File};
use std::io::BufWriter;
use jni_cpp;

mod jni;

pub fn write<P: AsRef<Path>>(path: P, classes: &BTreeMap<String, Class>) -> Result<()> {
    let mut path = path.as_ref().to_owned();

    path.push("jni");
    create_dir_all(&path)?;
    jni::write(&path, classes)?;
    path.pop();

    path.push("LiveSplitCoreJNI.cpp");
    jni_cpp::write(BufWriter::new(File::create(&path)?), classes)?;
    path.pop();

    Ok(())
}
