// test-data/structured.npy is generated by this Python code:
//
// import numpy as np
// a = np.array([(1,2.5,4), (2,3.1,5)], dtype=[('a', 'i4'),('b', 'f4'),('c', 'i8')])
// np.save('test-data/structured.npy', a)

#[derive(npyz::Deserialize, Debug, PartialEq)]
struct Struct {
    a: i32,
    b: f32,
    c: i64,
}

fn main() -> std::io::Result<()> {
    let bytes = std::fs::read("test-data/structured.npy")?;

    let reader = npyz::NpyFile::new(&bytes[..])?;
    let vec = reader.into_vec::<Struct>()?;
    assert_eq!(vec, vec![
        Struct { a: 1, b: 2.5, c: 4 },
        Struct { a: 2, b: 3.1, c: 5 },
    ]);
    Ok(())
}
