// Copyright (C) 2019-2026 Provable Inc.
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

//! `leo test` clap surface. The actual test logic lives in
//! [`leo_cli_core::commands::test::handle_test`]; this file just collects
//! flags, drives `LeoBuild` per workspace member, and forwards.

use super::*;

use leo_ast::NetworkName;
use leo_cli_core::commands::test::handle_test;

/// Test a leo program.
#[derive(Parser, Debug)]
pub struct LeoTest {
    #[clap(
        name = "TEST_NAME",
        help = "If specified, run only tests whose qualified name matches against this string.",
        default_value = ""
    )]
    pub(crate) test_name: String,

    #[clap(long, help = "Run all tests with full proof generation.", default_value = "false")]
    pub(crate) prove: bool,

    #[clap(flatten)]
    pub(crate) compiler_options: BuildOptions,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
}

impl Command for LeoTest {
    type Input = <LeoBuild as Command>::Output;
    type Output = TestOutput;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        let mut options = self.compiler_options.clone();
        options.build_tests = true;
        (LeoBuild { env_override: self.env_override.clone(), options }).execute(context)
    }

    fn apply(self, _: Context, input: Self::Input) -> Result<Self::Output> {
        let network = self.env_override.network.unwrap_or(NetworkName::TestnetV0);
        handle_test(input, &self.test_name, network, self.prove)
    }

    fn execute(self, context: Context) -> Result<Self::Output> {
        // Check for workspace mode before falling through to the default
        // prelude+apply flow, because test needs to build and test each
        // member independently.
        match context.resolve_targets()? {
            Some(targets) if targets.len() > 1 => {
                let mut aggregate = TestOutput::default();
                for target in &targets {
                    let member_name = target.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                    println!("\n--- workspace member '{member_name}' ---");

                    let member_ctx = context.with_path(target.clone());

                    let mut opts = self.compiler_options.clone();
                    opts.build_tests = true;
                    let package =
                        (LeoBuild { env_override: self.env_override.clone(), options: opts }).execute(member_ctx)?;

                    let network = self.env_override.network.unwrap_or(NetworkName::TestnetV0);
                    let result = handle_test(package, &self.test_name, network, self.prove)?;
                    aggregate.passed += result.passed;
                    aggregate.failed += result.failed;
                    aggregate.tests.extend(result.tests);
                }
                Ok(aggregate)
            }
            _ => {
                let input = self.prelude(context.clone())?;
                let span = self.log_span();
                let span = span.enter();
                let out = self.apply(context, input);
                drop(span);
                out
            }
        }
    }
}
