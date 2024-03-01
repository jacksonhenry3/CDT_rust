#![allow(unused)]

use cdt_rust::eh_action;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::time::{self, Instant};
use weighted_rand::builder::*;

use cdt_rust::volume_profiles::{num_cdts_in_profile, VolumeProfile};
use cdt_rust::{cdt::CDT, cdt_iterator, utils::choose, volume_profiles::volume_profiles};
use itertools::Itertools;
use rayon::prelude::*;

// This file can be used for testing if it's easier, but final work should be done in the tests folder
fn main() {}
