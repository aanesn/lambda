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

export function parseObj(text: string) {
	const vertices: number[] = []
	const normals: number[] = []
	const indices: number[] = []

	for (const line of text.split("\n")) {
		const parts = line.trim().split(/\s+/)
		if (parts[0] === "v") {
			const x = parseFloat(parts[1])
			const y = parseFloat(parts[2])
			const z = parseFloat(parts[3])
			vertices.push(x, y, z)
		} else if (parts[0] === "vn") {
			const x = parseFloat(parts[1])
			const y = parseFloat(parts[2])
			const z = parseFloat(parts[3])
			normals.push(x, y, z)
		} else if (parts[0] === "f") {
			const v1 = parseInt(parts[1].split("//")[0]) - 1
			const v2 = parseInt(parts[2].split("//")[0]) - 1
			const v3 = parseInt(parts[3].split("//")[0]) - 1
			indices.push(v1, v2, v3)
		}
	}

	const faceNormals: number[] = []
	for (const line of text.split("\n")) {
		const parts = line.trim().split(/\s+/)
		if (parts[0] === "f") {
			for (let i = 1; i <= 3; i++) {
				const normalIndex = parseInt(parts[i].split("//")[1]) - 1
				faceNormals.push(normals[normalIndex * 3])
				faceNormals.push(normals[normalIndex * 3 + 1])
				faceNormals.push(normals[normalIndex * 3 + 2])
			}
		}
	}

	return { vertices, normals: faceNormals, indices }
}
