# Create a new Leo program named `foo` and run `leo build`.

$LEO new foo
ls -la
cd foo && ls -la
$LEO build

# Assert that the 'outputs' folder is not empty

cd outputs || exit 1
if [ "$(ls -A $DIR)" ]; then
  echo "$DIR is not empty"
else
  echo "$DIR is empty"
  exit 1
fi
cd ..

# leo clean

$LEO clean
cd outputs && ls -la
cd ..

# Assert that the 'outputs' folder is empty

if [ "$(ls -A outputs)" ]; then
  echo "outputs is not empty"
  exit 1
else
  echo "outputs is empty"
fi

# Assert that the 'build' folder is empty

if [ "$(ls -A build)" ]; then
  echo "build is not empty"
  exit 1
else
  echo "build is empty"
  exit 0
fi

