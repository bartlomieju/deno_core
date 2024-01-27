// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use std::collections::HashMap;

use crate::extensions::Extension;
use crate::extensions::OpMiddlewareFn;
use crate::OpDecl;

/// Collects ops from extensions & applies middleware
pub fn init_ops_from_extensions(
  deno_core_ext: &mut Extension,
  exts: &mut [Extension],
) -> Vec<OpDecl> {
  // In debug build verify there that inter-Extension dependencies
  // are setup correctly.
  #[cfg(debug_assertions)]
  check_extensions_dependencies(deno_core_ext, exts);

  // TODO(bartlomieju)
  let no_of_ops = exts
    .iter()
    .map(|e| e.op_count())
    .fold(0, |ext_ops_count, count| count + ext_ops_count);
  let mut ops = Vec::with_capacity(no_of_ops + deno_core_ext.op_count());

  // Collect all middlewares - deno_core extension must not have a middleware!
  let middlewares: Vec<Box<OpMiddlewareFn>> = exts
    .iter_mut()
    .filter_map(|e| e.take_middleware())
    .collect();

  // Create a single macroware out of all middleware functions.
  let macroware = move |d| middlewares.iter().fold(d, |d, m| m(d));

  // Collect ops from all extensions and apply a macroware to each of them.
  let ext_ops = deno_core_ext.init_ops();
  for ext_op in ext_ops {
    ops.push(OpDecl {
      name: ext_op.name,
      ..macroware(*ext_op)
    });
  }

  for ext in exts.iter_mut() {
    let ext_ops = ext.init_ops();
    for ext_op in ext_ops {
      ops.push(OpDecl {
        name: ext_op.name,
        ..macroware(*ext_op)
      });
    }
  }

  // In debug build verify there are no duplicate ops.
  #[cfg(debug_assertions)]
  check_no_duplicate_op_names(&ops);

  ops
}

/// This functions panics if any of the extensions is missing its dependencies.
fn check_extensions_dependencies(
  deno_core_ext: &Extension,
  exts: &[Extension],
) {
  for (index, ext) in exts.iter().enumerate() {
    let mut previous_exts = vec![deno_core_ext];
    previous_exts.extend_from_slice(&exts[..index].iter().collect::<Vec<_>>());
    ext.check_dependencies(&previous_exts);
  }
}

/// This function panics if there are ops with duplicate names
fn check_no_duplicate_op_names(ops: &[OpDecl]) {
  let mut count_by_name = HashMap::new();

  for op in ops.iter() {
    count_by_name
      .entry(&op.name)
      .or_insert(vec![])
      .push(op.name.to_string());
  }

  let mut duplicate_ops = vec![];
  for (op_name, _count) in count_by_name.iter().filter(|(_k, v)| v.len() > 1) {
    duplicate_ops.push(op_name.to_string());
  }
  if !duplicate_ops.is_empty() {
    let mut msg = "Found ops with duplicate names:\n".to_string();
    for op_name in duplicate_ops {
      msg.push_str(&format!("  - {}\n", op_name));
    }
    msg.push_str("Op names need to be unique.");
    panic!("{}", msg);
  }
}