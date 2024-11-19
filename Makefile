##
## EPITECH PROJECT, 2023
## Architect
## File description:
## Makefile
##

BIN = architect
PACKAGE = architect
RELEASE = target/release/$(BIN)

all:
	cargo build --release --jobs 1 --package $(PACKAGE) --bin $(BIN)
	cp $(RELEASE) .

tests_run:
	cargo test --jobs 1 --package $(PACKAGE) --bin $(BIN)

clean:
	cargo clean

fclean: clean
	$(RM) $(BIN)

re:	fclean all