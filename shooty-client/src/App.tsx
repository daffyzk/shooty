import './App.css'

import { io, Socket } from "socket.io-client"
import { Level } from "./game/level.tsx"
import { Player } from "./game/player.tsx"


export type Coordinates = {[key: string]: number}

type OtherPlayer = { id: string, x: number, y: number, angle: number }

var canvas: HTMLCanvasElement,
	ctx: CanvasRenderingContext2D,
	scenario: Level,
	player: Player,
	crosshair: Coordinates = {x: 0, y: 0},
	mySocketId: string = "",
	otherPlayers: OtherPlayer[] = []

const socket     = io(
	"ws://localhost:4269/", {
	// protocol: "echo-protocol",	 
	transports: ["websocket"], 
	// reconnection: true,
	// upgrade: false,
    // reconnectionAttempts: 5,a
    // reconnectionDelay: 1000 
	}),
	FPS          = 60

	export const canvasHeight = 600,
	  canvasWidth  = 900

// Match keyboard events

document.addEventListener('keydown', (event) => {

  	switch(event.key){
		
		case "w":
			socket.emit("action", "moveup")
			console.log("moveup")
			player.moveUp()
		break

		case "a":
			socket.emit("action", "moveleft")
			console.log("moveleft")
			player.moveLeft()
		break
				
		case "s":
			socket.emit("action", "movedown")
			console.log("movedown")
			player.moveDown()
		break
		
		case "d":
			socket.emit("action", "moveright")
			console.log("moveright")
			player.moveRight()
		break
	}
})

document.addEventListener('keyup', (event) => {

	switch(event.key){

		case "w":
			player.stopMovingUp()
		break

		case "s":
			player.stopMovingDown()
		break

		case "a":
			player.stopMovingLeft()
		break

		case "d":
			player.stopMovingRight()
		break
	}
})

export function normalizeAngle(angle: number) : number {
	angle = angle % (2 * Math.PI)
	if(angle < 0){
		angle = (2 * Math.PI) + angle	// if it's negative, turn around
	}
	return angle
}

export function radianConvert(angle: number): number{
	angle = angle * (Math.PI / 180)
	return angle
}

export function lineSegment(x1: number,y1: number,x2: number,y2: number): number{
	return Math.sqrt((x2 - x1) * (x2 - x1) + (y2-y1)*(y2-y1))
} 

function init(){

	console.log("init started")
	socket.on("tick", (data: OtherPlayer[]) => {
		// filter out our own player, keep everyone else
		otherPlayers = data.filter(p => p.id !== mySocketId)
	})

	socket.on("connect", () => {
		mySocketId = socket.id ?? ""
		console.log("connected, id:", mySocketId);
	})


	socket.on("gameinfo", (data) => {
		console.log(data)
	})

	socket.on("disconnect", (reason: Socket.DisconnectReason, details) => {
		console.log(reason);

    if (details === undefined) {
      console.log("disconnect details are undefined")
    } else {
      console.log(details);
    }
	})
  
	//console.log("game started")
  let getCanvas = document.getElementById('game')
  if(getCanvas !== null && getCanvas instanceof HTMLCanvasElement) {
    canvas = getCanvas
    let getContext = canvas.getContext('2d')
    if(getContext !== null && getContext instanceof CanvasRenderingContext2D) {
    ctx = getContext
    }
  }

	// set canvas size (based on values hardcoded in css)
	canvas.width  = canvas.clientWidth
	canvas.height = canvas.clientHeight

	scenario      = new Level(canvas, ctx)
	player        = new Player(ctx, scenario)

	document.addEventListener('mousemove', (event) => {

		const rect = canvas.getBoundingClientRect();
		crosshair.x = event.clientX - rect.left;
		crosshair.y = event.clientY - rect.top;
	
		player.aim(crosshair)
		
	});

	document.addEventListener('click', () => {
		player.shoot()
		
	});

	setInterval(function(){gameLoop()},1000/FPS)  // start the game loop
}

function clearCanvas(){
	canvas.width  = canvas.width
	canvas.height = canvas.height
}

function drawOtherPlayers(){
	for(const p of otherPlayers){
		// server sends raw grid units, multiply by tileSize to get pixels
		let rx = p.x * scenario.tileSize
		let ry = p.y * scenario.tileSize

		// red square for other players
		ctx.save()
		ctx.fillStyle = "red"
		ctx.fillRect(rx - 4, ry - 4, 8, 8)

		// direction line
		let len = 15
		let xEnd = rx + Math.cos(p.angle) * len
		let yEnd = ry + Math.sin(p.angle) * len
		ctx.beginPath()
		ctx.moveTo(rx, ry)
		ctx.lineTo(xEnd, yEnd)
		ctx.strokeStyle = "red"
		ctx.stroke()
		ctx.restore()
	}
}

function gameLoop(){
	clearCanvas()

	scenario.draw()
	drawOtherPlayers()
	player.draw()
}


function App() {

	setTimeout(() => {
		init()
	}) 
		
  return (
    <>
      <main>
          <div id="game-wrapper">
              <canvas id="game"></canvas>
          </div>
  
          <div id="title">
              <h1>Shooty! (kill the red guys)</h1>
          </div>
      </main>
    </>
  )
}

export default App
