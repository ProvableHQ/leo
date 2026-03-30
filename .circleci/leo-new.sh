
echo "
Step 4: Downloading parameters. This may take a few minutes..."

# Create a new dummy Leo program and verify run + test work.
$LEO new dummy || exit
cd dummy || exit

# Attempt to compile the dummy program until it passes.
# This is necessary to ensure that the universal parameters are downloaded.
declare -i DONE

DONE=1

while [ $DONE -ne 0 ]
do
      $LEO build
      DONE=$?
      sleep 0.5
done

$LEO run main 0u32 1u32 || exit
$LEO test || exit

cd .. && rm -rf dummy

# Create a new dummy Leo library and verify build + test work.
$LEO new --library dummy_lib || exit
cd dummy_lib || exit
$LEO build || exit
$LEO test || exit

cd .. && rm -rf dummy_lib
