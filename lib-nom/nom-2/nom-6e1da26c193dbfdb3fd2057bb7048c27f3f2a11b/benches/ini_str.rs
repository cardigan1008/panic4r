#[macro_use]
extern crate nom;
#[macro_use]
extern crate criterion;
extern crate jemallocator;

use criterion::*;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use nom::{
  IResult,
  combinator::{map_res, opt},
  bytes::complete::{take_while, is_a},
  sequence::{delimited, terminated},
  character::complete::{char, alphanumeric1 as  alphanumeric, space0 as space, not_line_ending}
};


use std::collections::HashMap;

fn is_alphabetic(chr: char) -> bool {
  (chr as u8 >= 0x41 && chr as u8 <= 0x5A) || (chr as u8 >= 0x61 && chr as u8 <= 0x7A)
}

fn is_digit(chr: char) -> bool {
  chr as u8 >= 0x30 && chr as u8 <= 0x39
}

fn is_alphanumeric(chr: char) -> bool {
  is_alphabetic(chr) || is_digit(chr)
}

fn is_space(chr: char) -> bool {
  chr == ' ' || chr == '\t'
}

fn is_line_ending_or_comment(chr: char) -> bool {
  chr == ';' || chr == '\n'
}

/*
named!(alphanumeric<&str,&str>,         take_while!(is_alphanumeric));
named!(not_line_ending<&str,&str>,      is_not!("\r\n"));
named!(space<&str,&str>,                take_while!(is_space));
named!(space_or_line_ending<&str,&str>, is_a!(" \r\n"));
*/
fn space_or_line_ending(i: &str) -> IResult<&str, &str> {
  is_a(" \r\n")(i)
}

fn right_bracket(c: char) -> bool {
  c == ']'
}

fn category(i: &str) -> IResult<&str, &str> {
  terminated(delimited(char('['), take_while(|c| c != ']'), char(']')), opt(is_a(" \r\n")))(i)
}

/*
named!(category     <&str, &str>,
  do_parse!(
          tag!("[")                 >>
    name: take_till!(right_bracket) >>
          tag!("]")                 >>
          opt!(space_or_line_ending)  >>
    (name)
  )
);
*/

named!(key_value    <&str,(&str,&str)>,
  do_parse!(
    key: alphanumeric                              >>
         opt!(space)                               >>
         tag!("=")                               >>
         opt!(space)                               >>
    val: take_till!(is_line_ending_or_comment)   >>
         opt!(space)                               >>
         opt!(pair!(tag!(";"), not_line_ending)) >>
         opt!(space_or_line_ending)                >>
    (key, val)
  )
);

named!(keys_and_values_aggregator<&str, Vec<(&str, &str)> >, many0!(key_value));

fn keys_and_values(input: &str) -> IResult<&str, HashMap<&str, &str>> {
  match keys_and_values_aggregator(input) {
    Ok((i, tuple_vec)) => Ok((i, tuple_vec.into_iter().collect())),
    Err(e) => Err(e),
  }
}

named!(category_and_keys<&str,(&str,HashMap<&str,&str>)>,
  pair!(category, keys_and_values)
);

named!(categories_aggregator<&str, Vec<(&str, HashMap<&str,&str>)> >, many0!(category_and_keys));

fn categories(input: &str) -> IResult<&str, HashMap<&str, HashMap<&str, &str>>> {
  match categories_aggregator(input) {
    Ok((i, tuple_vec)) => Ok((i, tuple_vec.into_iter().collect())),
    Err(e) => Err(e),
  }
}

#[test]
fn parse_category_test() {
  let ini_file = "[category]

parameter=value
key = value2";

  let ini_without_category = "parameter=value
key = value2";

  let res = category(ini_file);
  println!("{:?}", res);
  match res {
    Ok((i, o)) => println!("i: {} | o: {:?}", i.0, o),
    _ => println!("error"),
  }

  assert_eq!(res, Ok((ini_without_category, "category")));
}

#[test]
fn parse_key_value_test() {
  let ini_file = "parameter=value
key = value2";

  let ini_without_key_value = "key = value2";

  let res = key_value(ini_file);
  println!("{:?}", res);
  match res {
    Ok((i, (o1, o2))) => println!("i: {} | o: ({:?},{:?})", i.0, o1, o2),
    _ => println!("error"),
  }

  assert_eq!(res, Ok((ini_without_key_value, ("parameter", "value"))));
}

#[test]
fn parse_key_value_with_space_test() {
  let ini_file = "parameter = value
key = value2";

  let ini_without_key_value = "key = value2";

  let res = key_value(ini_file);
  println!("{:?}", res);
  match res {
    Ok((i, (o1, o2))) => println!("i: {} | o: ({:?},{:?})", i.0, o1, o2),
    _ => println!("error"),
  }

  assert_eq!(res, Ok((ini_without_key_value, ("parameter", "value"))));
}

#[test]
fn parse_key_value_with_comment_test() {
  let ini_file = "parameter=value;abc
key = value2";

  let ini_without_key_value = "key = value2";

  let res = key_value(ini_file);
  println!("{:?}", res);
  match res {
    Ok((i, (o1, o2))) => println!("i: {} | o: ({:?},{:?})", i.0, o1, o2),
    _ => println!("error"),
  }

  assert_eq!(res, Ok((ini_without_key_value, ("parameter", "value"))));
}

#[test]
fn parse_multiple_keys_and_values_test() {
  let ini_file = "parameter=value;abc

key = value2

[category]";

  let ini_without_key_value = "[category]";

  let res = keys_and_values(ini_file);
  println!("{:?}", res);
  match res {
    Ok((i, ref o)) => println!("i: {} | o: {:?}", i.0, o),
    _ => println!("error"),
  }

  let mut expected: HashMap<&str, &str> = HashMap::new();
  expected.insert("parameter", "value");
  expected.insert("key", "value2");
  assert_eq!(res, Ok((ini_without_key_value, expected)));
}

#[test]
fn parse_category_then_multiple_keys_and_values_test() {
  //FIXME: there can be an empty line or a comment line after a category
  let ini_file = "[abcd]
parameter=value;abc

key = value2

[category]";

  let ini_after_parser = "[category]";

  let res = category_and_keys(ini_file);
  println!("{:?}", res);
  match res {
    Ok((i, ref o)) => println!("i: {} | o: {:?}", i.0, o),
    _ => println!("error"),
  }

  let mut expected_h: HashMap<&str, &str> = HashMap::new();
  expected_h.insert("parameter", "value");
  expected_h.insert("key", "value2");
  assert_eq!(res, Ok((ini_after_parser, ("abcd", expected_h))));
}

#[test]
fn parse_multiple_categories_test() {
  let ini_file = "[abcd]

parameter=value;abc

key = value2

[category]
parameter3=value3
key4 = value4
";

  let res = categories(ini_file);
  //println!("{:?}", res);
  match res {
    Ok((i, ref o)) => println!("i: {} | o: {:?}", i.0, o),
    _ => println!("error"),
  }

  let mut expected_1: HashMap<&str, &str> = HashMap::new();
  expected_1.insert("parameter", "value");
  expected_1.insert("key", "value2");
  let mut expected_2: HashMap<&str, &str> = HashMap::new();
  expected_2.insert("parameter3", "value3");
  expected_2.insert("key4", "value4");
  let mut expected_h: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
  expected_h.insert("abcd", expected_1);
  expected_h.insert("category", expected_2);
  assert_eq!(res, Ok(("", expected_h)));
}

fn bench_ini_str(c: &mut Criterion) {
  let s = "[owner]
name=John Doe
organization=Acme Widgets Inc.

[database]
server=192.0.2.62
port=143
file=payroll.dat
";

  c.bench(
    "bench ini str",
    Benchmark::new(
      "parse",
      move |b| {
        b.iter(|| categories(s).unwrap());
      },
    ).throughput(Throughput::Bytes(s.len() as u32)),
  );
}

criterion_group!(benches, bench_ini_str);
criterion_main!(benches);