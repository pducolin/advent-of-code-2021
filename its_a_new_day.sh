#!/bin/bash
TODAY=$(date +"%d");
XMAS=25;
if [[ "$TODAY" > "$XMAS" ]]; then
    echo "Xmas is over, so is AoC"
    exit 1
else
    echo "🎄 It's AoC day $TODAY, get ready to code 👩‍💻..."
fi
#create code folder and placeholder files
if [ -f src/day"$TODAY".rs ]; then
  echo "🎄 Day $TODAY already created 😲"
  exit 1
fi
touch src/day"$TODAY".rs;
touch inputs/day"$TODAY".txt;
touch inputs/day"$TODAY".test.txt;

day_code=$(<template/dayXX.rs);
echo "${day_code//XX/$TODAY}" > src/day"$TODAY".rs; 

day_toml=$(<template/dayXX.toml);
echo "${day_toml//XX/$TODAY}" >> Cargo.toml; 