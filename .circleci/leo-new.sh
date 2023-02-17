
echo "
Step 4: Downloading parameters. This may take a few minutes..."

# Create a new dummy Leo project.
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

# Try to run `leo run`.
$LEO run || exit

# Remove the dummy program.
cd .. && rm -rf dummy
