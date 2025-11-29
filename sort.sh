if [ $# -lt 1 ]; then
    echo "Usage: $0 <output_file>"
    exit 1
fi
output_file="$1"

lines=()
while IFS= read -r line; do
    lines+=("$line")
done

IFS=$'\n' sorted=($(printf "%s\n" "${lines[@]}" | sort))
for line in "${sorted[@]}"; do
    echo "$line" >> "$output_file"
done
