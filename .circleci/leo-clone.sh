# leo clone

$LEO clone leobot/test-app

# Assert that the 'test-app' folder is not empty

cd test-app || exit 1
if [ "$(ls -A $DIR)" ]; then
  echo "$DIR is not empty"
else
  echo "$DIR is empty"
  exit 1
fi

ls -la
$LEO run
