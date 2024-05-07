use std::collections::HashMap;
use std::io::{self, Read};
use std::fs::File;
use std::time::Instant;

fn compress(data: &[u8]) -> Vec<u32> {
    let mut dictionary: HashMap<Vec<u8>, u32> = (0u32..=255)
        .map(|i| (vec![i as u8], i))
        .collect();

    let mut w = Vec::new();
    let mut compressed = Vec::new();

    for &b in data {
        let mut wc = w.clone();
        wc.push(b);

        if dictionary.contains_key(&wc) {
            w = wc;
        } else {
            compressed.push(dictionary[&w]);

            dictionary.insert(wc, dictionary.len() as u32);
            w.clear();
            w.push(b);
        }
    }

    if !w.is_empty() {
        compressed.push(dictionary[&w]);
    }

    compressed
}

fn decompress(data: &[u32]) -> Vec<u8> {
    let mut dictionary: HashMap<u32, Vec<u8>> = (0u32..=255)
        .map(|i| (i, vec![i as u8]))
        .collect();

    let mut w = Vec::new();
    let mut decompressed = Vec::new();

    for &k in data {
        let entry = if dictionary.contains_key(&k) {
            dictionary[&k].clone()
        } else if k == dictionary.len() as u32 {
            let mut entry = w.clone();
            entry.push(w[0]);
            entry
        } else {
            panic!("Invalid dictionary!");
        };

        decompressed.extend_from_slice(&entry);

        if !w.is_empty() {
            let mut wc = w.clone();
            wc.push(entry[0]);
            dictionary.insert(dictionary.len() as u32, wc);
        }

        w = entry;
    }

    decompressed
}

fn main() -> io::Result<()> {
    let mut file = File::open("C:/Users/ryv26/OneDrive/Рабочий стол/ris/zlw/src/data.txt")?;
    let mut input_data = Vec::new();
    file.read_to_end(&mut input_data)?;

    let start_compress = Instant::now();
    let compressed_data = compress(&input_data);
    let compress_duration = start_compress.elapsed();

    println!("Count compressed data: {:?}", compressed_data.len());
    let slice_data = &compressed_data[1..100];
    println!("Data: {:?}", slice_data);

    let start_decompress = Instant::now();
    let decompressed_data = decompress(&compressed_data);
    let decompress_duration = start_decompress.elapsed();
    println!("Compress time: {:?}", compress_duration);
    println!("Decompress time: {:?}", decompress_duration);

    // println!("Decompressed data: {:?}", String::from_utf8_lossy(&decompressed_data));

    Ok(())
}