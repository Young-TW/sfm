use std::collections::HashMap;
use std::vec::Vec;
use std::fs;

fn unzip(path: &Path) -> Path {
    let mut archive = zip::ZipArchive::new(fs::File::open
        (path).expect("Unable to open file")).expect("Unable to read zip file");
    let dest = path.with_extension("");
    fs::create_dir(&dest).expect("Unable to create directory");
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Unable to read file");
        let path = dest.join(file.name());
        if file.name().ends_with('/') {
            fs::create_dir_all(&path).expect("Unable to create directory");
        } else {
            let mut out = fs::File::create(&path).expect("Unable to create file");
            io::copy(&mut file, &mut out).expect("Unable to copy file");
        }
    }

    return dest;
}


fn zip(path: &Path) {
    let mut archive = zip::ZipWriter::new(fs::File::create
        (path).expect("Unable to create file"));
    let src = path.with_extension("");
    for entry in fs::read_dir(&src).expect("Unable to read directory") {
        let entry = entry.expect("Unable to read directory entry");
        let path = entry.path();
        if path.is_dir() {
            archive.add_directory(path.to_str().unwrap(), Default::default()).expect("Unable to add directory");
        } else {
            archive.start_file(path.to_str().unwrap(), Default::default()).expect("Unable to add file");
            let mut file = fs::File::open(&path).expect("Unable to open file");
            io::copy(&mut file, &mut archive).expect("Unable to copy file");
        }
    }
}

/*
1. 儲存 zip 或 rar 檔案名稱到一個字串 儲存檔案雜湊值到另一個字串
2. 解壓縮 zip 或 rar 檔案
3. 進到解壓縮後的目錄
4. 刪除所有檔案名稱包含 "" 或 "" 的檔案
5. 壓縮回 zip 檔案
6. 刪除解壓縮後的目錄
7. 比較壓縮前後的檔案雜湊值 如果不同就刪除原本的壓縮檔案
*/