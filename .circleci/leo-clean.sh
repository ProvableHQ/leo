# Create a new Leo program named `foo`.
$LEO new foo || exit
ls -la
cd foo && ls -la

# Run `leo run`.
$LEO run || exit

# Assert that the 'build' folder exists.
if [ "$(ls -A build)" ]; then
  echo "build is not empty"
else
  echo "build is empty"
  exit 1
fi

# Run `leo clean`
$LEO clean || exit

# Assert that the 'build' folder is empty.
if [ "$(ls -A build)" ]; then
  echo "build is not empty"
  exit 1
else
  echo "build is empty"
  exit 0
fi

