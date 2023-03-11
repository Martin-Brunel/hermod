git pull;
sudo systemctl stop hermod.service;
/home/ubuntu/.cargo/bin/diesel migration run;
cargo build --release;
sudo systemctl start hermod.service;
exit 0;