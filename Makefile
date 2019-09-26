# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: Administrateur <Administrateur@student.    +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2019/09/26 09:42:49 by Administr         #+#    #+#              #
#    Updated: 2019/09/26 09:49:53 by Administrat      ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

COLOR ?= always
CARGO = cargo --color $(COLOR)

.PHONY: all build

all: build

build:
	@$(CARGO) build --release && cp ./target/release/computer_v1 .

test:
	@$(CARGO) test