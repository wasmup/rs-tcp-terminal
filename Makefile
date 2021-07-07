all:
	gnome-terminal -- cargo run
	gnome-terminal -- nc 127.0.0.1 8080
	gnome-terminal -- nc 127.0.0.1 8080