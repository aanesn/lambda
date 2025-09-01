import { positions } from "./geometry"
import * as m4 from "./m4"

export function createShader(gl: WebGLRenderingContext, type: number, source: string) {
	const shader = gl.createShader(type)
	if (!shader) throw new Error("failed to create shader")

	gl.shaderSource(shader, source)
	gl.compileShader(shader)

	// think this returns a bool but idk javascript is so fucking bad
	if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		throw new Error("failed to compile shader")
	}

	return shader
}

export function createProgram(
	gl: WebGLRenderingContext,
	vertexShader: WebGLShader,
	fragmentShader: WebGLShader
) {
	const program = gl.createProgram()

	gl.attachShader(program, vertexShader)
	gl.attachShader(program, fragmentShader)
	gl.linkProgram(program)

	// again this probably returns a bool
	if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
		gl.deleteProgram(program)
		throw new Error("failed to link program")
	}

	return program
}

export function init(gl: WebGLRenderingContext) {
	const positionBuffer = gl.createBuffer()
	gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)
	gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW)
}

export function render(
	gl: WebGLRenderingContext,
	program: WebGLProgram,
	positionLocation: number,
	colorLocation: number,
	matrixLocation: WebGLUniformLocation
) {
	gl.useProgram(program)

	gl.enableVertexAttribArray(positionLocation)
	gl.vertexAttribPointer(positionLocation, 3, gl.FLOAT, false, 24, 0)

	gl.enableVertexAttribArray(colorLocation)
	gl.vertexAttribPointer(colorLocation, 3, gl.FLOAT, false, 24, 12)

	const triangles = [
		{ pos: [-0.1, 0.5, 0], scale: 0.2, rotX: 0.3, rotY: 0.8 },
		{ pos: [-0.4, 0, 0], scale: 0.3, rotX: 0.5, rotY: 0.7 },
		{ pos: [0.5, 0.1, 0], scale: 0.3, rotX: 1.5, rotY: 0.3 }
	]

	for (const triangle of triangles) {
		let matrix = m4.identity()
		matrix = m4.translate(matrix, triangle.pos[0], triangle.pos[1], triangle.pos[2])
		matrix = m4.scale(matrix, triangle.scale)
		matrix = m4.rotateX(matrix, triangle.rotX)
		matrix = m4.rotateY(matrix, triangle.rotY)

		gl.uniformMatrix4fv(matrixLocation, false, matrix)
		gl.drawArrays(gl.TRIANGLES, 0, 12)
	}
}
