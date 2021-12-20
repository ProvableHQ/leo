Tests in this folder test external dependencies that were imported. To mock that we 
need to have structure similar to typical Leo package: source files must be in the
directory next to `imports/`; but instead of `src/` we have `tests/` here.

Option `cwd` param in these tests must point to the `tests/`, so having it as `.` 
should be enough. 

Have fun testing!
