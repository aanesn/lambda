export function createShader(gl: WebGLRenderingContext, type: number, source: string) {
	const shader = gl.createShader(type)
	if (!shader) throw new Error("failed to create shader")

	gl.shaderSource(shader, source)
	gl.compileShader(shader)

	// think this returns a bool but idk javascript is so fucking bad
	const ok = gl.getShaderParameter(shader, gl.COMPILE_STATUS)
	if (!ok) throw new Error("failed to compile shader")

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
	const ok = gl.getProgramParameter(program, gl.LINK_STATUS)
	if (!ok) {
		gl.deleteProgram(program)
		throw new Error("failed to link program")
	}

	return program
}
