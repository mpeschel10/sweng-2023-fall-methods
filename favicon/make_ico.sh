__file__="$(pwd)/$0"
script_dir="$(dirname "$__file__")"
script_dir=$(cd "$script_dir"; pwd) # resolve . and ..

for size in 16 32 64; do
    inkscape -w $size -h $size -o "${script_dir}/${size}.png" "${script_dir}/server-minimalistic-svgrepo-com.svg";
done;

convert "${script_dir}/*.png" "${script_dir}/favicon.ico"
cp "${script_dir}/favicon.ico" "${script_dir}/../serve"
