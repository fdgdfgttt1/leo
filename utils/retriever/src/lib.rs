// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

pub mod retriever;
pub use retriever::*;

pub mod program_context;
pub use program_context::*;

#[cfg(test)]
mod tests {
    use super::*;
    use aleo_std::aleo_dir;
    use leo_span::{symbol::create_session_if_not_set_then, Symbol};
    use std::path::PathBuf;

    #[test]
    #[ignore]
    fn temp_dir_test() {
        // Test pulling nested dependencies from network
        const BUILD_DIRECTORY: &str = "../tmp/nested";
        const HOME_DIRECTORY: &str = "../tmp/.aleo";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);
            let home_dir = PathBuf::from(HOME_DIRECTORY);
            let mut retriever =
                Retriever::new(Symbol::intern("nested"), &build_dir, &home_dir).expect("Failed to build retriever");
            retriever.retrieve().expect("failed to retrieve");
            retriever.prepare_local(Symbol::intern("nested")).expect("failed to prepare local");
            retriever.process_local(Symbol::intern("nested")).expect("failed to process local");
        });
    }

    #[test]
    #[ignore]
    fn mac_test() {
        // Test pulling nested dependencies from network
        const BUILD_DIRECTORY: &str = "../tmp/nested";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);

            println!("aleo_dir: {:?}", aleo_dir());
            let mut retriever =
                Retriever::new(Symbol::intern("nested"), &build_dir, &aleo_dir()).expect("Failed to build retriever");
            retriever.retrieve().expect("failed to retrieve");
            retriever.prepare_local(Symbol::intern("nested")).expect("failed to prepare local");
            retriever.process_local(Symbol::intern("nested")).expect("failed to process local");
        });
    }

    #[test]
    #[ignore]
    fn simple_dir_test() {
        // Test pulling nested dependencies from network
        const BUILD_DIRECTORY: &str = "../tmp/simple";
        const HOME_DIRECTORY: &str = "../tmp/.aleo";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);
            let home_dir = PathBuf::from(HOME_DIRECTORY);
            let mut retriever =
                Retriever::new(Symbol::intern("simple"), &build_dir, &home_dir).expect("Failed to build retriever");
            retriever.retrieve().expect("failed to retrieve");
            retriever.prepare_local(Symbol::intern("simple")).expect("failed to prepare local");
            retriever.process_local(Symbol::intern("simple")).expect("failed to process local");
        });
    }

    #[test]
    #[ignore]
    fn local_dir_test() {
        // Test pulling nested dependencies from network
        const BUILD_DIRECTORY: &str = "../tmp/local_test";
        const HOME_DIRECTORY: &str = "../tmp/.aleo";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);
            let home_dir = PathBuf::from(HOME_DIRECTORY);
            let mut retriever =
                Retriever::new(Symbol::intern("local_test"), &build_dir, &home_dir).expect("Failed to build retriever");
            let _deps = retriever.retrieve().expect("failed to retrieve");
            retriever.prepare_local(Symbol::intern("nested")).expect("failed to prepare local");
            // retriever.process_local(Symbol::intern("nested")).expect("failed to process local");
            // retriever.prepare_local(Symbol::intern("local_dep_1")).expect("failed to prepare local");
        });
    }
}
