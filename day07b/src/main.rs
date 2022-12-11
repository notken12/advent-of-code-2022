use std::collections::HashMap;

#[derive(Debug)]
struct Dir {
    children: HashMap<String, DirEntry>,
    size: Option<u32>,
}

#[derive(Debug)]
enum DirEntry {
    Dir(Dir),
    File { size: u32 },
}

const MAX_SPACE: u32 = 70000000;
const REQUIRED_UNUSED_SPACE: u32 = 30000000;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut fs = Dir {
        children: HashMap::new(),
        size: None,
    };
    fs.children.insert(
        "/".into(),
        DirEntry::Dir(Dir {
            children: HashMap::new(),
            size: None,
        }),
    );
    let mut cwd: Vec<String> = Vec::new();

    let mut dir_sizes: Vec<u32> = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let line = line.trim();

            if line.starts_with("$ cd") {
                // Walk back up tree and calculate dir sizes
                // if cwd has all sizes present
                calc_dir_sizes_up(&cwd, &mut fs, &mut dir_sizes);

                let arg = line.get(5..).unwrap();
                if arg == ".." {
                    cwd.pop();
                } else {
                    cwd.push(arg.into());
                }
            } else if line.starts_with("$ ls") {
                // No extra action needed
            } else if line.starts_with("dir") {
                // Add dir to fs
                let dirname = line.get(4..).unwrap();

                let cwd_dir = get_cwd_dir(&cwd, &mut fs);
                cwd_dir.children.insert(
                    dirname.into(),
                    DirEntry::Dir(Dir {
                        children: HashMap::new(),
                        size: None,
                    }),
                );
            } else {
                // Add file to fs
                let words: Vec<&str> = line.split(" ").collect();
                let filesize: u32 = words[0].parse().unwrap();
                let filename = words[1];

                let cwd_dir = get_cwd_dir(&cwd, &mut fs);
                cwd_dir
                    .children
                    .insert(filename.into(), DirEntry::File { size: filesize });
            }
        }
    }

    calc_dir_sizes_up(&cwd, &mut fs, &mut dir_sizes);

    dir_sizes.sort();

    if let DirEntry::Dir(root) = fs.children.get("/").unwrap() {
        for size in dir_sizes {
            let unused_space = MAX_SPACE - root.size.unwrap();
            if unused_space + size >= REQUIRED_UNUSED_SPACE {
                println!("{}", size);
                return;
            }
        }
    }
}

fn get_cwd_dir<'a>(cwd: &'a Vec<String>, fs: &'a mut Dir) -> &'a mut Dir {
    let mut cwd_dir;
    cwd_dir = fs.children.get_mut(&cwd[0]).unwrap();
    for i in 1..cwd.len() {
        match cwd_dir {
            DirEntry::Dir(d) => {
                cwd_dir = d.children.get_mut(&cwd[i]).unwrap();
            }
            _ => {
                panic!()
            }
        }
    }
    if let DirEntry::Dir(cwd_dir) = cwd_dir {
        cwd_dir
    } else {
        panic!();
    }
}

fn calc_dir_sizes_up(cwd: &Vec<String>, fs: &mut Dir, sizes: &mut Vec<u32>) {
    if cwd.len() == 0 {
        return;
    }
    let cwd_dir = get_cwd_dir(cwd, fs);
    if let None = cwd_dir.children.iter().find(|e| match e.1 {
        DirEntry::Dir(d) => d.size == None,
        DirEntry::File { size: _ } => false,
    }) {
        // All entries have sizes, size is determinable
        let sum: u32 = cwd_dir
            .children
            .iter()
            .map(|e| match e.1 {
                DirEntry::File { size } => size.clone(),
                DirEntry::Dir(d) => {
                    if let Some(size) = d.size {
                        size
                    } else {
                        panic!()
                    }
                }
            })
            .sum();

        let cwd_dir = get_cwd_dir(&cwd, fs);
        if let None = cwd_dir.size {
            cwd_dir.size = Some(sum);
            sizes.push(sum);
        }
        // Recurse, crawl up the tree
        let mut upper_dir = cwd.clone();
        upper_dir.pop();
        calc_dir_sizes_up(&upper_dir, fs, sizes)
    }
    // End recursion
}
