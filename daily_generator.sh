day=$(date +%d)
month=$(date +%m)

if [ "$day" -ge 1 ] && [ "$day" -le 25 ]; then
  filename="day$(printf "%02d" $day).rs"
  if [ ! -f "src/solutions/$filename" ]; then
    cp src/template.rs "src/solutions/$filename"
    echo File \"src/solutions/$filename\" created.
  else
    echo "File \"src/solutions/$filename\" already exists."
    echo "No file created."
  fi
elif [ "$month" -ne 12 ]; then
  echo "It's not advent season yet :("
  echo "No file created."
fi
