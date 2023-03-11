git pull;
sudo systemctl stop hermod.service;
diesel migration run;
cargo build --release;
sudo systemctl start hermod.service;
exit 0;