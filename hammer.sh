if [ "$#" -eq "0" ]; then
    echo "Help: hammer.sh <count> <cmd and args...>"
    exit 1
fi

count=$1
shift
cmd="$@"

echo "Running the following command $count times: '$cmd'"

for i in $(seq 1 $count); do
    if ! $cmd; then
        echo "COMMAND FAILED: $cmd"
        break
    fi
done

echo "Ran $i times"