pub mod alias;
mod animation;
pub mod command;
mod dialog;
mod init;
mod key;
mod theme;
mod tree;

// generated by build.rs
include!(concat!(env!("OUT_DIR"), "/mod_build_info.rs"));