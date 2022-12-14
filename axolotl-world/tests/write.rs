use std::fs::{remove_file, OpenOptions};
use std::path::PathBuf;

use axolotl_world::chunk::RawChunk;
use axolotl_world::region::file::RegionFile;
use axolotl_world::region::RegionHeader;

#[test]
pub fn test() {
    let path = PathBuf::new().join("test_region.mca");
    if path.exists() {
        remove_file(&path).unwrap();
    }

    {
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path)
            .unwrap();
        let header = RegionHeader::default();
        header.write_region(&mut file).unwrap();
    }
    let header = {
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();

        RegionHeader::read_region_header(&mut file).unwrap()
    };
    let mut region = RegionFile {
        file: path.clone(),
        region_header: header,
        write_buffer: vec![],
    };
    region.write_chunk(RawChunk::default()).unwrap();
    let chunk = RawChunk {
        x_pos: 1,
        z_pos: 1,
        ..RawChunk::default()
    };
    region.write_chunk(chunk).unwrap();
    region.save().unwrap();

    let mut file = OpenOptions::new().read(true).open(&path).unwrap();
    let header = RegionHeader::read_region_header(&mut file).unwrap();
    println!("{:?}", header);

    drop(file);
    for i in header.locations {
        if i.0 != 0 {
            let option = region.read_chunk::<RawChunk>(&i).unwrap();
            println!("{:?}", option);
        } else {
            println!("{:?}", i);
        }
    }
}
