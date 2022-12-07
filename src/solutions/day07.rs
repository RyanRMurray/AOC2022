use std::collections::HashMap;

use crate::utils::solver_types::{solve_simultaneous, SolutionSimultaneous};
use anyhow::Result;
use itertools::Itertools;

//not yet implemented
pub struct Day7Solution {}

#[derive(Default, Debug)]
struct FileTree {
    visit_stack: Vec<String>,
    tree: HashMap<String, Vec<String>>,
    contents: HashMap<String, Vec<usize>>,
}

impl FileTree {
    fn from(instrs: impl Iterator<Item = String>) -> Self {
        let mut f_tree = FileTree::default();

        f_tree.visit_stack.push("/".to_string());

        for i in instrs {
            match i.split_once(' ').unwrap() {
                ("$", "ls") => (),
                ("$", "cd ..") => _ = f_tree.visit_stack.pop().unwrap(),
                ("$", cd_dir) => f_tree
                    .visit_stack
                    .push(cd_dir.split_once(' ').unwrap().1.to_string()),
                ("dir", dir) => {
                    let current_dir = f_tree.visit_stack.join("");
                    let sub_dir = [&current_dir, dir].join("");
                    f_tree
                        .tree
                        .entry(current_dir)
                        .or_default()
                        .push(sub_dir.to_string());
                }
                (filesize, _) => {
                    let current_dir = f_tree.visit_stack.join("");
                    f_tree
                        .contents
                        .entry(current_dir)
                        .or_default()
                        .push(filesize.parse().unwrap());
                }
            }
        }

        f_tree
    }
}

fn get_sizes(f_tree: &FileTree, dir: String) -> HashMap<String, usize> {
    let mut sizes = HashMap::default();

    // create subtrees
    for sub in f_tree.tree.get(&dir).unwrap_or(&vec![]) {
        sizes.extend(get_sizes(f_tree, sub.clone()));
    }

    // calculate size for this node
    let mut size: usize = f_tree.contents.get(&dir).unwrap_or(&vec![]).iter().sum();

    size += f_tree
        .tree
        .get(&dir)
        .unwrap_or(&vec![])
        .iter()
        .map(|sub| sizes.get(sub).unwrap())
        .sum::<usize>();

    sizes.insert(dir, size);

    sizes
}

impl SolutionSimultaneous<FileTree, usize, usize> for Day7Solution {
    fn load(input: &str) -> Result<FileTree> {
        let is = input.lines().skip(1).map(|s| s.to_string());
        Ok(FileTree::from(is))
    }

    fn solve(input: FileTree) -> Result<(usize, usize)> {
        let sizes = get_sizes(&input, "/".to_string());

        // get the sum of all directories under the specified size
        let p1 = sizes.values().filter(|s| **s <= 100_000).sum();

        // calculate the amount of space we need to free
        let target = 30_000_000 - (70_000_000 - sizes.get("/").unwrap());

        // find the smallest directory we can delete to meet the above target
        let p2 = sizes
            .iter()
            .filter(|(_, size)| **size >= target)
            .sorted_by(|a, b| a.1.cmp(b.1))
            .next()
            .unwrap()
            .1;

        Ok((p1, *p2))
    }
}

pub fn day07(input: &str) -> Result<f32> {
    solve_simultaneous::<Day7Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day7Solution;
    use crate::utils::solver_types::SolutionSimultaneous;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

        let p1_expected = 95437;
        let p2_expected = 24933642;

        let loaded = Day7Solution::load(input).unwrap();

        let (p1, p2) = Day7Solution::solve(loaded).unwrap();

        assert_eq!(p1_expected, p1);
        assert_eq!(p2_expected, p2);
    }
}
