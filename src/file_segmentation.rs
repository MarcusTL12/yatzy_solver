use std::{
    fs::{create_dir_all, read_dir, OpenOptions},
    io::{BufReader, Read, Write},
    path::PathBuf,
    str::from_utf8,
};

use itertools::Itertools;

fn segment_file(filename_in: PathBuf, path_out: &str, chunksize: usize) {
    let mut buffer = Vec::new();
    let mut strbuf = Vec::new();

    let filenamename = filename_in
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    for (i, chunk) in BufReader::with_capacity(
        chunksize,
        OpenOptions::new().read(true).open(filename_in).unwrap(),
    )
    .bytes()
    .map(|byte| byte.unwrap())
    .chunks(chunksize)
    .into_iter()
    .enumerate()
    {
        buffer.clear();
        buffer.extend(chunk);

        strbuf.clear();
        write!(&mut strbuf, "{path_out}/{filenamename}.{i}").unwrap();
        let new_filepath = from_utf8(&strbuf).unwrap();
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(new_filepath)
            .unwrap()
            .write_all(&buffer)
            .unwrap();
    }
}

pub fn segment_dir(path: &str, outpath: &str, chunksize: usize) {
    create_dir_all(outpath).unwrap();

    OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{outpath}/chunksize.dat"))
        .unwrap()
        .write_all(&chunksize.to_le_bytes())
        .unwrap();

    for filename in read_dir(path)
        .unwrap()
        .map(|x| x.unwrap().path())
        .filter(|x| x.is_file())
    {
        segment_file(filename, outpath, chunksize);
    }
}
