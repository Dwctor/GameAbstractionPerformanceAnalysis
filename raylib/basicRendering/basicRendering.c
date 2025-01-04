#include <raylib.h>

#define SCREEN_WIDTH 1280
#define SCREEN_HEIGHT 720
#define GAME_NAME "Basic Rendering"

#define SPRITE_SIZE 24

#define PLAYER_ANIM_FPS 6
#define PLAYER_SPEED 24 * 5

typedef struct Player{
	Vector2 pos;
	bool flip;
	Texture2D sprite[2];
	Rectangle source;
	int frame;
} Player;

int main(){
	//--------------------------------------------------------------------------
	// Initialize game
	//--------------------------------------------------------------------------
	InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, GAME_NAME);

	SetTargetFPS(0);

	//Game Specific Initialization Here:
	
	Camera2D camera = { 0 };
	camera.zoom = 2.0;
	
	Player player = {.pos = {.x = SCREEN_WIDTH / 4.0, .y = SCREEN_HEIGHT / 4.0}, .flip = false, .frame = 0};

	player.sprite[0] = LoadTexture("assets/Player_0.png");
	player.sprite[1] = LoadTexture("assets/Player_1.png");

	player.source = (Rectangle){.x = 0, .y = 0, .width = SPRITE_SIZE, .height = SPRITE_SIZE};

	int dir = 0;
	
	
	
	//--------------------------------------------------------------------------
	// Run Game
	//--------------------------------------------------------------------------
	while(!WindowShouldClose()){
		//----------------------------------------------------------------------
		// Game Logic
		//----------------------------------------------------------------------
		dir = IsKeyDown(KEY_RIGHT) - IsKeyDown(KEY_LEFT);

		player.pos = (Vector2) {.x = player.pos.x += PLAYER_SPEED * dir * GetFrameTime(), .y = player.pos.y};

		if(dir == 0){
			player.frame = 0;
		} else if (dir == 1) {
			player.frame = (int) (GetTime() * PLAYER_ANIM_FPS) % 2;
			player.source = (Rectangle){.x = 0, .y = 0, .width = -SPRITE_SIZE, .height = SPRITE_SIZE};
		} else if (dir == -1) {
			player.frame = (int) (GetTime() * PLAYER_ANIM_FPS) % 2;
			player.source = (Rectangle){.x = 0, .y = 0, .width = SPRITE_SIZE, .height = SPRITE_SIZE};
		}
		
		//----------------------------------------------------------------------
		// Rendering Logic
		//----------------------------------------------------------------------
		BeginDrawing();
			ClearBackground((Color){0,0,0,0});

			BeginMode2D(camera);
				DrawTexturePro(player.sprite[player.frame], player.source, (Rectangle){.x = player.pos.x, .y = player.pos.y, .width = 48, .height = 48}, (Vector2) {0,0}, 0, WHITE);
			EndMode2D();

			DrawFPS(0, 0);
		EndDrawing();
		//Post-Rendering Logic (Most likely empty)
	}

	//--------------------------------------------------------------------------
	// Clean-up
	//--------------------------------------------------------------------------
	
	//======================================
	CloseWindow(); //Closes the open window.
	//======================================
}
