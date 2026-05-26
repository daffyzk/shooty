import './App.css'

import { io, Socket } from "socket.io-client"
import { Level, parseServerMap } from "./game/level.tsx"
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
	if(["w","a","s","d"].includes(event.key) && !event.repeat){
		socket.emit("keyinput", { key: event.key, pressed: true })
		if(player){
			if(event.key === "w") player.moveUp()
			if(event.key === "a") player.moveLeft()
			if(event.key === "s") player.moveDown()
			if(event.key === "d") player.moveRight()
		}
	}
})

document.addEventListener('keyup', (event) => {
	if(["w","a","s","d"].includes(event.key)){
		socket.emit("keyinput", { key: event.key, pressed: false })
		if(player){
			if(event.key === "w") player.stopMovingUp()
			if(event.key === "s") player.stopMovingDown()
			if(event.key === "a") player.stopMovingLeft()
			if(event.key === "d") player.stopMovingRight()
		}
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
		otherPlayers = data.filter(p => p.id !== mySocketId)
		// lerp local player toward server position (server is authoritative)
		const me = data.find(p => p.id === mySocketId)
		if(me && player){
			const serverX = me.x * scenario.tileSize
			const serverY = me.y * scenario.tileSize
			const t = 0.3 // interpolation factor — higher = snappier, lower = smoother
			player.x += (serverX - player.x) * t
			player.y += (serverY - player.y) * t
		}
	})

	socket.on("connect", () => {
		mySocketId = socket.id ?? ""
		console.log("connected, id:", mySocketId);
	})


	socket.on("gameinfo", (data: { map: number[], width: number, height: number }) => {
		console.log("gameinfo received:", data)

		const mapData = parseServerMap(data.map, data.width)
		const tileSize = 20
		// spawn at tile (2,2), convert to pixels
		const spawnX = 2 * tileSize
		const spawnY = 2 * tileSize

		scenario = new Level(canvas, ctx, mapData, tileSize, spawnX, spawnY)
		player   = new Player(ctx, scenario)

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
	})

	socket.on("disconnect", (reason: Socket.DisconnectReason, details) => {
		console.log(reason);

    if (details === undefined) {
      console.log("disconnect details are undefined")
    } else {
      console.log(details);
    }
	})

	// setup canvas
  let getCanvas = document.getElementById('game')
  if(getCanvas !== null && getCanvas instanceof HTMLCanvasElement) {
    canvas = getCanvas
    let getContext = canvas.getContext('2d')
    if(getContext !== null && getContext instanceof CanvasRenderingContext2D) {
    ctx = getContext
    }
  }

	canvas.width  = canvas.clientWidth
	canvas.height = canvas.clientHeight
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

	// send position to server in raw grid units (pixels / tileSize)
	if(player && scenario){
		socket.volatile.emit("position", {
			x: player.x / scenario.tileSize,
			y: player.y / scenario.tileSize,
			angle: player.rotationAngle
		})
	}
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
