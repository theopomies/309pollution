##
## EPITECH PROJECT, 2021
## makefile
## File description:
## 309pollution
##

MAKEFLAGS	+=	--no-print-directory -j
BINARY_PATH	=	target/release/pollution
BINARY_NAME =	309pollution

all:
			cargo build --release
			cp $(BINARY_PATH) ./$(BINARY_NAME)

clean:
			cargo clean

fclean:		clean
			rm -f $(BINARY_NAME)

tests_run:	clean
			cargo test
			
re:			fclean all

.PHONY:		all clean fclean re debug test