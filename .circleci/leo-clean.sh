# leo new hello-world

$LEO new hello-world
ls -la
cd hello-world && ls -la
$LEO run

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

cd outputs || exit 1
if [ "$(ls -A $DIR)" ]; then
  echo "$DIR is not empty"
  exit 1
else
  echo "$DIR is empty"
  exit 0
fi
