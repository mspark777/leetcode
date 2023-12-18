ROWS=()
while IFS=' '; read -a columns
do
i=0

for col in "${columns[@]}"
do
  if [ -z "${ROWS[${i}]}" ]
  then
    ROWS[${i}]="${col}"
  else
    ROWS[${i}]+=" ${col}"
  fi
  ((i+=1))
done

done < file.txt

for row in "${ROWS[@]}"
do
  echo "${row}"
done
