extends AnimatedSprite2D

const SPEED = 24 * 5

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta):
	var dir = int(Input.is_action_pressed("move_right")) - int(Input.is_action_pressed("move_left"))
	if(dir == 0):
		self.animation = "Idle"
	else:
		self.animation = "Walk"

func _physics_process(delta):
	var dir = int(Input.is_action_pressed("move_right")) - int(Input.is_action_pressed("move_left"))
	if(dir > 0):
		self.flip_h = true
	elif (dir < 0):
		self.flip_h = false
	
	self.position.x += dir * delta * SPEED
