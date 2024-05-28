use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
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
    // Чтение исходного файла
    let mut file = File::open("C:/Users/ryv26/OneDrive/Рабочий стол/ris/zlw/src/text.txt")?;
    let mut input_data = Vec::new();
    file.read_to_end(&mut input_data)?;

    // Замер времени сжатия
    let start_compress = Instant::now();
    let compressed_data = compress(&input_data);
    let compress_duration = start_compress.elapsed();

    // Запись сжатых данных в файл
    let mut output_file = File::create("compressed_data.lzw")?;
    for &num in &compressed_data {
        output_file.write_all(&num.to_le_bytes())?;
    }

    println!("Размер исходных данных: {:?}", input_data.len());
    println!("Размер сжатых данных: {:?}", compressed_data.len());
    let slice_data = &compressed_data[..100.min(compressed_data.len())];
    println!("Часть сжатых данных: {:?}", slice_data);

    // Замер времени декомпрессии
    let start_decompress = Instant::now();
    let decompressed_data = decompress(&compressed_data);
    let decompress_duration = start_decompress.elapsed();

    println!("Время сжатия: {:?}", compress_duration);
    println!("Время декомпрессии: {:?}", decompress_duration);

    // Проверка целостности декомпрессированных данных
    if input_data == decompressed_data {
        println!("Декомпрессированные данные совпадают с исходными.");
    } else {
        println!("Декомпрессированные данные НЕ совпадают с исходными.");
    }

    Ok(())
}
