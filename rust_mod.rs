// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::prelude::*;

module! {
    type: RustMod,
    name: "rust_mod",
    author: "Rust for Linux Contributors",
    description: "Rust out-of-tree sample",
    license: "GPL",
}

struct RustMod {
    numbers: KVec<i32>,
}

impl kernel::Module for RustMod {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust out-of-tree sample (init)\n");

        let mut numbers = KVec::new();
        numbers.push(72, GFP_KERNEL)?;
        numbers.push(108, GFP_KERNEL)?;
        numbers.push(200, GFP_KERNEL)?;

        Ok(RustMod { numbers })
    }
}

impl Drop for RustMod {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
