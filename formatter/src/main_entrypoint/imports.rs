// Copyright (C) 2019-2025 Provable Inc.
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

use leo_parser_lossless::{SyntaxKind, SyntaxNode};

use crate::{Formatter, Output, impl_tests};

impl Formatter<'_, '_> {
    pub(super) fn format_import(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Import);
        let [imp, pid, smc] = &node.children[..] else { panic!("Can't happen") };

        self.push_snippet(imp.text)?;
        self.space()?;
        self.push_snippet(pid.text)?;
        self.push_snippet(smc.text)?;
        self.space()?;
        self.consolidate_trivia(&imp.children[..], 1)?;
        self.consolidate_trivia(&pid.children[..], 1)?;
        self.consolidate_trivia(&smc.children[..], 1)
    }
}

impl_tests!(
    test_format_import,
    src = "import    
        a.aleo    
        
        ;

        import    
                    a.aleo    
        
        ;

        import //sdd
        a.aleo    
        
        ;
        program a.aleo {}
        ",
    exp = "import a.aleo;
import a.aleo;
import a.aleo; //sdd

program a.aleo {}
",
    Kind::Main
);
