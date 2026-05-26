import { normalizeAngle, radianConvert, Coordinates } from "../App.tsx"
import { Ray } from "./rays.tsx"
import { Level } from "./level.tsx"

console.groupCollapsed("Player")

export class Player {

    ctx: CanvasRenderingContext2D
    scenario: Level
    x: number
    y: number
    FOV: number
    crosshair: Coordinates
    move: {[key: string]: boolean}
    rotationAngle: number				        			  // should be aiming angle later (in radians)
    movementSpeed: number

	directionAngle: number

    // fog of war renderer hell yeah
    maxRays: number               				   			  // amount of rays casted
    rays: Ray[]



		constructor(context: CanvasRenderingContext2D, scenario: Level){
		
		this.ctx           = context
		this.scenario      = scenario

		this.FOV           = 120,

		this.crosshair     = {x: 0, y: 0}
  
		this.x             = scenario.spawn.x
		this.y             = scenario.spawn.y
		this.move          = {xpos:false, ypos:false, xneg:false, yneg:false}
		
		this.rotationAngle = 0					        			// should be aiming angle later (in radians)
		this.movementSpeed = 3
		
		// fog of war renderer hell yeah
		this.maxRays       = 60         				   			// amount of rays casted
		this.rays          = []    			                		// array of rays

		this.directionAngle   = radianConvert(this.rotationAngle - (this.FOV / 2)) // player's angle of direction
		var angleIncrement = radianConvert(this.FOV / this.maxRays) // fraction of the full fov 
		var rayAngle       = this.directionAngle
		
		for(let i=0; i < this.maxRays; i++){               			// creating each one of the rays
			let currentAngle = normalizeAngle(rayAngle + this.rotationAngle)
			this.rays[i] = new Ray(this.ctx, this.scenario, this.x, this.y, this.directionAngle, currentAngle, 200, "lightgreen")
			rayAngle += angleIncrement 								// current ray slope + fractional increment until full FOV is covered
		}
	}

	moveUp(){
		//console.log("move up")
		this.move.ypos = true
	}
	
	moveDown(){
		//console.log("move down")
		this.move.yneg = true
	}

	moveLeft(){
		//console.log("move left")
		this.move.xneg = true
	}
	
	moveRight(){
		//console.log("move right")
		this.move.xpos = true
	}

	stopMovingUp(){
		this.move.ypos = false
 	}

	stopMovingDown(){
		this.move.yneg = false
 	}

	stopMovingLeft(){
		this.move.xneg = false
 	}

	stopMovingRight(){
		this.move.xpos = false
 	}

	aim(ch: Coordinates){
		this.crosshair.x = ch.x 
		this.crosshair.y = ch.y
	}

	shoot() {
		let bullet: Ray = new Ray(this.ctx, this.scenario, this.x, this.y, this.directionAngle, this.rotationAngle, 500, "red")
		console.log("player angle: %s, rotation angle: %s", this.directionAngle, this.rotationAngle)
		bullet.draw()
	}

	collision(x: number, y: number){
		// check all 4 corners of player bounding box (6x6, so ±3 from center)
		const half = 3
		const corners = [
			{x: x - half, y: y - half},  // top-left
			{x: x + half, y: y - half},  // top-right
			{x: x - half, y: y + half},  // bottom-left
			{x: x + half, y: y + half},  // bottom-right
		]
		for (const c of corners) {
			let tileX = Math.floor(c.x / this.scenario.tileWidth)
			let tileY = Math.floor(c.y / this.scenario.tileHeight)
			if (this.scenario.collision(tileX, tileY)) {
				return true
			}
		}
		return false
	}
	
	update(){

		// movement logic
		var coords = {x: 0, y: 0}
		
		if (this.move.ypos) coords.y -= this.movementSpeed
		if (this.move.yneg) coords.y += this.movementSpeed
		if (this.move.xneg) coords.x -= this.movementSpeed
		if (this.move.xpos) coords.x += this.movementSpeed
		
		if (coords.x !== 0 && coords.y !== 0) {								    // if coords are not zero, get the length of movement
			const length = Math.sqrt(coords.x * coords.x + coords.y * coords.y) // by doing the square root of the sum of square roots
			coords.x /= length												    // then normalize value
			coords.y /= length
			coords.x *= this.movementSpeed										// and scale to speed
			coords.y *= this.movementSpeed
		}
		
		var newX = this.x + coords.x
		var newY = this.y + coords.y

		// check X and Y separately so player can slide along walls
		if(!this.collision(newX, this.y)){
			this.x = newX
		}
		if(!this.collision(this.x, newY)){
			this.y = newY
		}
		
		// update rays
		for(let i=0; i < this.maxRays; i++){
			this.rays[i].x = this.x
			this.rays[i].y = this.y
			this.rays[i].setAngle(this.rotationAngle)
		}	
	}
	
	draw(){
        
		//update before drawing
		this.update()
		
		// draw rays
		for(let i=0; i < this.maxRays; i++){
			this.rays[i].draw()
		}

		// draw crosshair idk
		this.ctx.save()
		this.ctx.beginPath()
		this.ctx.arc(this.crosshair.x, this.crosshair.y, 7, 0, 2 * Math.PI) // cool circle
		this.ctx.strokeStyle = "yellow"
		this.ctx.lineWidth = 2
		this.ctx.stroke()
		this.ctx.restore()
		
		this.ctx.save()
		this.ctx.fillStyle = "yellow" 
		this.ctx.fillRect(this.crosshair.x - 7, this.crosshair.y - 1, 14, 1) // vertical line
		this.ctx.fillRect(this.crosshair.x - 1, this.crosshair.y - 7, 1, 14) // horizontal line
		this.ctx.restore()

        // white player circle
		this.ctx.save()
        this.ctx.fillStyle = "white" 
        this.ctx.fillRect(this.x - 3, this.y - 3, 6, 6)  					// 6x6 rectangle centered on player 0 axis
        
        
        // line parallel to player camera (crosshair)
		let angle = Math.atan2(this.crosshair.y - this.y, this.crosshair.x - this.x)

		// this angle can also be used to be the center of ray-casts!
		this.rotationAngle = angle

		let len = 20 
		var xPointer = this.x + Math.cos(angle) * len
        var yPointer = this.y + Math.sin(angle) * len
        

        this.ctx.beginPath()
        this.ctx.moveTo(this.x, this.y)
        this.ctx.lineTo(xPointer, yPointer)
        this.ctx.strokeStyle = "white"
        this.ctx.stroke()
		this.ctx.restore()

	}
}