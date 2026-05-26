import { Coordinates } from "../App.tsx"

type HEX = `#${string}`

// convert server map format (array of bit-packed integers) to 2D tile array
export function parseServerMap(rawMap: number[], width: number): number[][] {
	return rawMap.map(row => {
		const tiles: number[] = []
		for(let col = 0; col < width; col++){
			tiles.push((row >> (width - 1 - col)) & 1)
		}
		return tiles
	})
}

export class Level {

    tileSize: number
    canvas : HTMLCanvasElement
    ctx: CanvasRenderingContext2D
    matrix: number[][]

    spawn: Coordinates

    // map dimensions
    mapHeight: number
    mapWidth: number

    // canvas dimensions
    canvasHeight: number
    canvasWidth: number

    // tile size
    tileWidth: number
    tileHeight: number

	constructor(canvas: HTMLCanvasElement, context: CanvasRenderingContext2D, mapData: number[][], tileSize: number, spawnX: number, spawnY: number){

		this.tileSize = tileSize
		this.canvas = canvas
		this.ctx = context
		this.matrix = mapData

		this.spawn = {x: spawnX, y: spawnY}

		this.mapHeight  = this.matrix.length
		this.mapWidth = this.matrix[0].length

		this.canvasHeight = this.canvas.height
		this.canvasWidth = this.canvas.width

		this.tileWidth = this.tileSize
		this.tileHeight = this.tileSize

	}
	
	collision(x: number, y: number) {
		var crash: boolean = false

		if((y >= this.mapHeight || x >= this.mapWidth) || (y < 0 || x < 0)){
			console.error("X or Y value outside of map grid: x=" + x + " y=" + y )
		} else {
			// console.log(this.matrix)
			// console.log("x: %s, y: %s", x, y)
			// wierd typescript stuff happening here, for some reason it returns undefined unless I Math.round() the values.
			// I assume it's because the values are floating point and it expects an int
			if( this.matrix[Math.round(y)][Math.round(x)] != 0){
				crash = true
			}
			return crash
		}
	}
	
	tile(x: number, y: number) {
		var tilePosX = x / this.tileWidth
		var tilePosY = y / this.tileHeight
		return(this.matrix[tilePosY][tilePosX])
	}
	
	draw(){
		
		var color: HEX

		for(var y=0; y < this.mapHeight; y++){
			for(var x=0; x < this.mapWidth; x++){
				
				if(this.matrix[y][x]!=0){
					color = '#000000'
				}
				else{
					color = '#666666'
				}
				this.ctx.fillStyle = color
				this.ctx.fillRect(x * this.tileWidth, y * this.tileHeight, this.tileWidth, this.tileHeight)
			}
		}
	}
}