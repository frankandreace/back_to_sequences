use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{self};
use std::sync::Arc;
<<<<<<< HEAD
use clap::ArgMatches;
use fxread::initialize_reader;
use atomic_counter::{RelaxedCounter, AtomicCounter};
use rayon::prelude::*;

fn validate_non_empty_file(in_file: String) {
    // check that inkmers is a non empty file:
    // Attempt to get metadata for the file
    if let Ok(metadata) = fs::metadata(in_file.clone()) {
        // Check if the file exists
        if ! metadata.is_file() {
            panic!("{:#} exists, but it's not a file.", in_file);
        }
    } else {
        panic!("The {} file does not exist or there was an error checking its existence.", in_file);
    }
}

pub fn validate_kmers(sub_matches: &ArgMatches) -> std::io::Result<()> {
    let kmer_file = sub_matches.get_one::<String>("IN_KMERS").map(|s| s.clone()).unwrap();
    let reads_file = sub_matches.get_one::<String>("INFASTA").map(|s| s.clone()).unwrap();
    let out_reads_file = sub_matches.get_one::<String>("OUTFASTA").map(|s| s.clone()).unwrap();
    let out_kmers_file = sub_matches.get_one::<String>("OUTKMERS").map(|s| s.clone()).unwrap();
    let kmer_size = sub_matches.get_one::<usize>("K").map(|s| s.clone()).unwrap();
  
    // check that inkmers and reads_file are non empty files:
    validate_non_empty_file(kmer_file.clone());
    validate_non_empty_file(reads_file.clone());
=======
use fxread::initialize_reader;
use atomic_counter::{RelaxedCounter, AtomicCounter};
use rayon::prelude::*;
>>>>>>> dev

    
    match index_kmers::<RelaxedCounter>(kmer_file, kmer_size) {

<<<<<<< HEAD
        Ok((kmer_set, kmer_size)) => {
            let kmer_set = Arc::new(kmer_set);
            let _ = count_kmers_in_fasta_file_par(reads_file, &kmer_set, kmer_size, out_reads_file.clone());
            println!("Filtered reads with exact kmer count are in file {}", out_reads_file);
            

            // if the out_kmers_file is not empty, we output counted kmers in the out_kmers_file file
            if out_kmers_file.len() > 0 {
                
                // prints all kmers from kmer_set that have a count > 0
                let mut output = File::create(out_kmers_file.clone())?;
                // let mut output = File::create(out_kmers_file);
                for (kmer, count) in kmer_set.iter() {
                    if count.get() > 0 {
                        write!(output, "{} {}\n", kmer, count.get())?;
                        // output.write_all(kmer.as_bytes())?;
                        // output.write_all(b" ")?;
                        // output.write_all(count.get().to_string().as_bytes())?;
                        // output.write_all(b"\n")?;
                    }
                }
            println!("kmers with their number of occurrences in the original reads are in file {}", out_kmers_file);
            }
        }

        Err(err) => eprintln!("Error indexing kmers: {}", err),
    }
    Ok(())

}
=======
>>>>>>> dev

fn reverse_complement(kmer: &str) -> String {
    kmer.chars()
        .rev()
        .map(|base| match base {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => base,
        })
        .collect()
}

fn canonical(kmer: &str, stranded: bool) -> String {
    if stranded  {return kmer.to_string();}
    let rev_comp = reverse_complement(kmer);
    if kmer < &rev_comp {
        kmer.to_string()
    } else {
        rev_comp
    }
}

<<<<<<< HEAD
/////////////////// PARALLELIZATION

fn index_kmers<T:Default>(file_name: String, kmer_size: usize) -> Result<(HashMap<String, T>, usize), io::Error> {
=======





fn validate_non_empty_file(in_file: String) {
    // check that inkmers is a non empty file:
    // Attempt to get metadata for the file
    if let Ok(metadata) = fs::metadata(in_file.clone()) {
        // Check if the file exists
        if ! metadata.is_file() {
            panic!("{:#} exists, but it's not a file.", in_file);
        }
    } else {
        panic!("The {} file does not exist or there was an error checking its existence.", in_file);
    }
}
fn index_kmers<T:Default>(file_name: String, kmer_size: usize, stranded: bool) -> Result<(HashMap<String, T>, usize), io::Error> {
>>>>>>> dev
    let mut kmer_set = HashMap::new();

    let reader = initialize_reader(&file_name).unwrap();
    for record in reader {
        let record_as_string = record.as_str_checked().unwrap().trim().as_bytes();
        let mut iter = record_as_string.split(|&x| x == b'\n');
        let _ = iter.next().unwrap();
        let acgt_sequence = iter.next().unwrap().to_owned();
        let string_acgt_sequence = String::from_utf8(acgt_sequence).expect("Found invalid UTF-8");
        // for each kmer of the sequence, insert it in the kmer_set
        for i in 0..(string_acgt_sequence.len() - kmer_size + 1) {
            let kmer = &string_acgt_sequence[i..(i + kmer_size)];
<<<<<<< HEAD
            kmer_set.insert(canonical(&&kmer.to_ascii_uppercase()), Default::default()); // Atomic Counter Relaxed Counter RelaxedCounter::new(0);
=======
            kmer_set.insert(canonical(&&kmer.to_ascii_uppercase(), stranded), Default::default()); // Atomic Counter Relaxed Counter RelaxedCounter::new(0);
>>>>>>> dev
        }
        // kmer_set.insert(canonical(&&string_acgt_sequence.to_ascii_uppercase()), 0);
    }
    println!("Indexed {} kmers, each of size {}", kmer_set.len(), kmer_size);
    
    Ok((kmer_set, kmer_size))
}

<<<<<<< HEAD
fn count_kmers_in_fasta_file_par(file_name: String, kmer_set:  &Arc<HashMap<String, atomic_counter::RelaxedCounter>>, kmer_size: usize, out_fasta: String) -> std::io::Result<()>{
    let output = File::create(out_fasta)?;
    let write_lock = std::sync::Arc::new(std::sync::Mutex::new(output));
    let (tx, rx) = std::sync::mpsc::sync_channel(1024);
    let (_, result) = rayon::join(move ||{// lance deux threads 
        let reader = initialize_reader(&file_name).unwrap();
        for record in reader {
            tx.send(record).unwrap();
        }
    }, ||{
        rx.into_iter().par_bridge().try_for_each(|record| -> std::io::Result<()>{
            let record_as_string = record.as_str_checked().unwrap().trim().as_bytes(); // supprimer check ? unchecked ? 
            let mut iter = record_as_string.split(|&x| x == b'\n');
            let stringheader = iter.next().unwrap();
            let acgt_sequence = iter.next().unwrap().to_owned(); // eviter copie ?
            let string_acgt_sequence = String::from_utf8(acgt_sequence).expect("Found invalid UTF-8"); // eviter copie ?
            let nb_shared_kmers = count_shared_kmers_par(kmer_set, &string_acgt_sequence, kmer_size);
            if nb_shared_kmers > 0{ // read contains at least one indexed kmer
                let mut out = write_lock.lock().unwrap();
                out.write_all(stringheader)?;
                out.write_all(b" ")?;
                out.write_all(nb_shared_kmers.to_string().as_bytes())?; 
                out.write_all(b"\n")?;
                out.write_all(string_acgt_sequence.as_bytes())?;
                out.write_all(b"\n")?;
                for line in iter {
                    out.write_all(line)?;
                    out.write_all(b"\n")?;
                }
            } // end read contains at least one indexed kmer
        Ok(())
        }) // end of for each
    }); // end of rayon join
    result
}

fn count_shared_kmers_par(kmer_set:  &Arc<HashMap<String, atomic_counter::RelaxedCounter>>, read: &str, kmer_size: usize) -> usize {
=======
fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}

fn count_kmers_in_fasta_file_par(file_name: String, kmer_set:  &Arc<HashMap<String, atomic_counter::RelaxedCounter>>, kmer_size: usize, out_fasta: String, stranded: bool) -> std::io::Result<()>{
    let output = File::create(out_fasta)?;
    let write_lock = std::sync::Arc::new(std::sync::Mutex::new(output));
    let (tx, rx) = std::sync::mpsc::sync_channel(1024);
    let (_, result) = rayon::join(move ||{// lance deux threads 
        let reader = initialize_reader(&file_name).unwrap();
        for record in reader {
            tx.send(record).unwrap();
        }
    }, ||{
        rx.into_iter().par_bridge().try_for_each(|record| -> std::io::Result<()>{
            let record_as_string = record.as_str_checked().unwrap().trim().as_bytes(); // supprimer check ? unchecked ? 
            let mut iter = record_as_string.split(|&x| x == b'\n');
            let stringheader = iter.next().unwrap();
            let acgt_sequence = iter.next().unwrap().to_owned(); // eviter copie ?
            let string_acgt_sequence = String::from_utf8(acgt_sequence).expect("Found invalid UTF-8"); // eviter copie ?
            let nb_shared_kmers = count_shared_kmers_par(kmer_set, &string_acgt_sequence, kmer_size, stranded);
            let ratio_shared_kmers = nb_shared_kmers as f32 / (string_acgt_sequence.len() - kmer_size + 1) as f32;
            // round ratio_shared_kmers to 3 decimals and transform to percents
            let percent_shared_kmers = round(ratio_shared_kmers*100.0, 2);
            if nb_shared_kmers > 0{ // read contains at least one indexed kmer
                let mut out = write_lock.lock().unwrap();
                out.write_all(stringheader)?;
                out.write_all(b" ")?;
                out.write_all(nb_shared_kmers.to_string().as_bytes())?; 
                out.write_all(b" ")?;
                out.write_all(percent_shared_kmers.to_string().as_bytes())?;
                out.write_all(b"\n")?;
                out.write_all(string_acgt_sequence.as_bytes())?;
                out.write_all(b"\n")?;
                for line in iter {
                    out.write_all(line)?;
                    out.write_all(b"\n")?;
                }
            } // end read contains at least one indexed kmer
        Ok(())
        }) // end of for each
    }); // end of rayon join
    result
}

fn count_shared_kmers_par(kmer_set:  &Arc<HashMap<String, atomic_counter::RelaxedCounter>>, read: &str, kmer_size: usize, stranded: bool) -> usize {
>>>>>>> dev
    let mut shared_kmers_count = 0;

    for i in 0..(read.len() - kmer_size + 1) {
        let kmer = &read[i..(i + kmer_size)];
        let canonical_kmer = canonical(&kmer.to_ascii_uppercase(), stranded);
        if kmer_set.contains_key(&canonical_kmer){
            shared_kmers_count += 1;
            // kmer_set[&canonical_kmer] += 1;
            // kmer_set.insert(canonical_kmer, 1 + kmer_set[&canonical_kmer] );
            
            // *kmer_set.get_mut(&canonical_kmer).unwrap().add(1);
            kmer_set[&canonical_kmer].inc();

        }
    }
    shared_kmers_count
}



pub fn validate_kmers(in_fasta_reads: String, in_fasta_kmers: String, out_fasta_reads:String, out_txt_kmers: String, kmer_size: usize, stranded: bool) -> std::io::Result<()> {
      
    // check that inkmers and reads_file are non empty files:
    validate_non_empty_file(in_fasta_reads.clone());
    validate_non_empty_file(in_fasta_kmers.clone());

    
    match index_kmers::<RelaxedCounter>(in_fasta_kmers, kmer_size, stranded) {

        Ok((kmer_set, kmer_size)) => {
            let kmer_set = Arc::new(kmer_set);
            let _ = count_kmers_in_fasta_file_par(in_fasta_reads, &kmer_set, kmer_size, out_fasta_reads.clone(), stranded);
            println!("Filtered reads with exact kmer count are in file {}", out_fasta_reads);
            

            // if the out_kmers_file is not empty, we output counted kmers in the out_kmers_file file
            if out_txt_kmers.len() > 0 {
                
                // prints all kmers from kmer_set that have a count > 0
                let mut output = File::create(out_txt_kmers.clone())?;
                // let mut output = File::create(out_kmers_file);
                for (kmer, count) in kmer_set.iter() {
                    if count.get() > 0 {
                        write!(output, "{} {}\n", kmer, count.get())?;
                    }
                }
            println!("kmers with their number of occurrences in the original reads are in file {}", out_txt_kmers);
            }
        }

        Err(err) => eprintln!("Error indexing kmers: {}", err),
    }
    Ok(())

}