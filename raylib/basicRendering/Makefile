# Project Names
# -------------
PROJECT_NAME	?= basicRendering
GAME_NAME		?= BasicRendering

# Compilation Variables
# ---------------------
CC = clang
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

# Make and run
run: $(GAME_NAME)
	./$(GAME_NAME)

# Cleaning Compilation Files
clean:
	rm -f $(PROJECT_NAME) $(OBJ)
