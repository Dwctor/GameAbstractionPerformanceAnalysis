# Project Names
# -------------
PROJECT_NAME	?= template
GAME_NAME		?= executable 

# Compilation Variables
# ---------------------
CC = clang++
CFLAGS = -Wall
#INCLUDE_PATHS =
RAYLIBS = -lraylib -lGL -lm -lpthread -ldl -lrt -lX11

# Compilation Files
# -----------------
OBJ = $(PROJECT_NAME).o


# Makefile Rules
# --------------
$(GAME_NAME): $(OBJ)
	$(CC) $(CFLAGS) $^ -o $@ $(RAYLIBS) 

%.o: %.c
	$(CC) -c $(CFLAGS) $< -o $@


# Cleaning Compilation Files
clean:
	rm -f $(PROJECT_NAME) $(OBJ)
