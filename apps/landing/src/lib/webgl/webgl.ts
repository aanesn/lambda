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
	const positions = [0, 0, 0, 0.5, 0.7, 0]
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW)
}

export function render(
	gl: WebGLRenderingContext,
	program: WebGLProgram,
	positionLocation: number,
	matrixLocation: WebGLUniformLocation
) {
	gl.useProgram(program)

	gl.enableVertexAttribArray(positionLocation)
	gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0)

	let matrix = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]
	matrix = m4.translate(matrix, -0.5, 0, 0)
	matrix = m4.rotateZ(matrix, Math.PI / 2)
	gl.uniformMatrix4fv(matrixLocation, false, matrix)

	gl.drawArrays(gl.TRIANGLES, 0, 3)
}
