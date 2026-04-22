program test.aleo {
    // `cond ? true : false` folds to `cond`. This pattern commonly arises
    // after composite forwarding erases a boolean flag that was selected
    // across a ternary (e.g. `opt.is_some` on a ternary of optional structs).
    fn fold_to_cond(cond: bool) -> bool {
        return cond ? true : false;
    }

    // `cond ? false : true` folds to `!cond`.
    fn fold_to_not_cond(cond: bool) -> bool {
        return cond ? false : true;
    }

    // `cond ? true : true` folds to `true`; the condition is an SSA atom at
    // this point, so dropping it is side-effect-free.
    fn fold_both_true(cond: bool) -> bool {
        return cond ? true : true;
    }

    // `cond ? false : false` folds to `false` for the same reason.
    fn fold_both_false(cond: bool) -> bool {
        return cond ? false : false;
    }

    // Non-constant branches are never folded: the ternary must be preserved
    // because `a` and `b` may hold any boolean values.
    fn no_fold_variable(cond: bool, a: bool, b: bool) -> bool {
        return cond ? a : b;
    }
}
