#![feature(globs)]
#![feature(macro_rules)]
#![feature(default_type_params)]

extern crate collections;
extern crate std;
extern crate time;

mod float_compares; // note that macros don't really obey scopes so ordering of these modules matters
mod blendable;
mod sinuous_signal;
mod trigger;
mod sig_gen_tester;
mod mixer;
mod mixer_unthreaded;
mod gen_and_mix_tester;
mod ghetto_benchmarker;