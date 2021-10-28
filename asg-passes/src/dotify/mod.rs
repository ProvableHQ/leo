// Copyright (C) 2019-2021 Aleo Systems Inc.
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
mod dotgraph;
use dotgraph::*;

mod expression;

mod dotifier;
pub use dotifier::Dotifier;

mod statement;

mod program;

use leo_asg::*;
use leo_errors::Result;

use std::fs::File;
use std::path::PathBuf;

impl<'a, 'b> AsgPass<'a> for Dotifier<'a, 'b> {
    type Input = (Program<'a>, &'b AsgContext<'a>, &'b [String], String, PathBuf);
    type Output = Result<Program<'a>>;

    fn do_pass((asg, ctx, filter_keys, id, path): Self::Input) -> Self::Output {
        let graph = DotGraph::new(id, filter_keys);
        let mut director = MonoidalDirector::new(Dotifier::new(graph, ctx));
        director.reduce_program(&asg);

        let mut file = File::create(path).unwrap();
        dot::render(&director.reducer().graph, &mut file).unwrap();
        Ok(asg)
    }
}
