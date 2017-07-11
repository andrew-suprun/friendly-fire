# Project friendly-fire


* Using git and github
* Server / client communication
* Server written in Swift


* Every tic, server pulls the position, direction, speed, from each player/tank/unit, possibly other info. 
* When server sends message to a unit, it gives it some info
	* other units in field of vision
	* list of units within communication range.
* We need a protocol to determine how this information is encoded in these messages. 
	* simplest is probably Json
	* So every message would have message type
		* current_state
			* this type of message is sent from the server to each unit, each tic
			* Would include:
			* bot_position: x,y
			* bot_velocity: speed and direction (theta). 
			* enemies: An array that would include the position and velocity of visible enemies
			* friendlies: same as enemies, but some will be outside of “view” and inside of communication range.
			* health (0-100)
		* communication:
			* this type of message would be sent from one bot to another friendly bot via the server.
			* Would include:
			* to_id, from_id: id of the robot that the message is from / to. 
			* message (arbitrary format) 
				* it would be up to the programmer to decide how messages are encoded.
		* Control variable changes;
			* sent from bot to server to say the changes it wants to make.
			* Would include:
			* shoot: (distance and direction of the shot.)
			* desired change in velocity: 


### Other features:
* Collisions:
	* damage would depend on speed. 


### Configurable paramaters:

* battlefield size
* damage /radius that shots deal
* collision damage
* maximum speed, accel, turning rate, cooldown between shots. 
