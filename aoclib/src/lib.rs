#![feature(test)]
extern crate test;

use std::env;
use std::fmt::Display;
use std::io::{stdin, Read, BufRead, BufReader};
use std::process::exit;

#[must_use = "This value does nothing unless you call .run() or .run_display()"]
#[derive(Clone)]
pub struct AocTask<I, F1, F2> {
    pub input: I,
    f1: F1,
    f2: F2,
}

fn not_implemented_1<T>(_: T) -> ! {
    eprintln!("Task 1 is not implemented");
    exit(1);
}

fn not_implemented_2<T>(_: T) -> ! {
    eprintln!("Task 2 is not implemented");
    exit(1);
}

impl<I> AocTask<I, fn(I) -> !, fn(I) -> !>
{
    pub fn read_full(mut reader: impl FnMut(&str) -> I) -> Self {
        let mut input = String::new();
        stdin().read_to_string(&mut input).unwrap();

        AocTask {
            input: reader(&input),
            f1: not_implemented_1,
            f2: not_implemented_2,
        }
    }
}

impl<I> AocTask<Vec<I>, fn(Vec<I>) -> !, fn(Vec<I>) -> !>
{
    pub fn read_lines(mut reader: impl FnMut(&str) -> I) -> Self {
        let input = BufReader::new(stdin())
            .lines()
            .map(Result::unwrap)
            .map(|line| reader(&line))
            .collect();

        AocTask {
            input,
            f1: not_implemented_1,
            f2: not_implemented_2,
        }
    }
}

impl<I, F1, F2> AocTask<I, F1, F2>
{
    pub fn task1<R, F11: FnOnce(I) -> R>(self, task: F11) -> AocTask<I, F11, F2> {
        AocTask {
            input: self.input,
            f1: task,
            f2: self.f2,
        }
    }

    pub fn task2<R, F21: FnOnce(I) -> R>(self, task: F21) -> AocTask<I, F1, F21> {
        AocTask {
            input: self.input,
            f1: self.f1,
            f2: task,
        }
    }
}

impl<I, F1, F2, R1, R2> AocTask<I, F1, F2>
where
    F1: FnOnce(I) -> R1,
    F2: FnOnce(I) -> R2,
{
    pub fn run(self) {
        match env::args().nth(1) {
            Some(s) if s == "1" => drop((self.f1)(self.input)),
            Some(s) if s == "2" => drop((self.f2)(self.input)),
            Some(_) => {
                eprintln!("Only 1 and 2 are acceptable arguments");
                exit(1);
            }
            _ => {
                eprintln!("Provide task number (1 or 2)");
                exit(1);
            }
        }
    }

    pub fn run_display(self)
    where
        R1: Display,
        R2: Display,
    {
        match env::args().nth(1) {
            Some(s) if s == "1" => println!("{}", (self.f1)(self.input)),
            Some(s) if s == "2" => println!("{}", (self.f2)(self.input)),
            Some(_) => {
                eprintln!("Only 1 and 2 are acceptable arguments");
                exit(1);
            }
            _ => {
                eprintln!("Provide task number (1 or 2)");
                exit(1);
            }
        }
    }

    pub fn bench1(self, t: &mut test::Bencher)
    where 
        I: Clone,
        F1: Clone,
    {
        let Self { input, f1, .. } = self;
        t.iter(|| (f1.clone())(input.clone()));
    }

    pub fn bench2(self, t: &mut test::Bencher)
    where 
        I: Clone,
        F2: Clone,
    {
        let Self { input, f2, .. } = self;
        t.iter(|| (f2.clone())(input.clone()));
    }
}
