#![allow(dead_code, unused_variables, unused_imports)]
// "Git is fundamentally a content-addressable filesystem with a VCS user interface written on top of it"
// the one true src: https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain

use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::rc::Rc;

// Typedefs.
type Objects = Vec<Object>;
type Blob = File;

enum Tree {
    MaybeTree(Option<Rc<Tree>>),
    MaybeBlob(Option<Blob>),
}

enum Object {
    Blob(File),
    Tree,
    Commit {},
}

// Commit objects are identified by the SHA-1 hash of their contents
struct Commit<'a, H>
where
    H: Hash,
{
    message: String,
    author: String,
    prev: &'a Commit<'a, H>,
    hash: H,
}

// are these mutually exclusive?
enum BlobState {
    Untracked, // in working tree
    Added,     // out of working tree, into index
    Committed, // out of index, into remote
}

// TODO: use typestate pattern to encode the staging area/ready to add/push whatever
// https://cliffle.com/blog/rust-typestate/

// TODO: `git hash-object,` which takes input data, runs a hash on it, puts the hashed data in the
// `.git/objects/` dir, and gives you back the key (the hashed value referring to that data)
