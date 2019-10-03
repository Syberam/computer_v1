# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: Administrateur <Administrateur@student.    +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2019/09/26 09:42:49 by Administr         #+#    #+#              #
#    Updated: 2019/09/27 17:56:51 by Administrat      ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME= computor
CARGO = cargo


.PHONY: all build tests

all: build


build:
	@$(CARGO) build --release && cp ./target/release/computor .

tests:
	@$(CARGO) test

clean:
	rm -f $(NAME)

fclean: clean
	$(CARGO) clean

re: fclean all