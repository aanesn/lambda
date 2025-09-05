<script lang="ts">
	import { createProgram, createShader, parseObj } from "./webgl"
	import vertex from "./vertex.glsl?raw"
	import fragment from "./fragment.glsl?raw"
	import lambda from "./lambda.obj?raw"
	import * as m4 from "./m4"
	import { easeOut } from "./animation"

	let canvas: HTMLCanvasElement
	let clientWidth = $state(0)
	let clientHeight = $state(0)

	$effect(() => {
		const gl = canvas.getContext("webgl")
		if (!gl) {
			console.error("webgl is unsupported in your browser")
			return
		}

		const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertex)
		const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragment)
		const program = createProgram(gl, vertexShader, fragmentShader)

		const positionLocation = gl.getAttribLocation(program, "a_position")
		const normalLocation = gl.getAttribLocation(program, "a_normal")
		const matrixLocation = gl.getUniformLocation(program, "u_matrix")

		const { vertices, normals, indices } = parseObj(lambda)

		const positionBuffer = gl.createBuffer()
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)

		const normalBuffer = gl.createBuffer()
		gl.bindBuffer(gl.ARRAY_BUFFER, normalBuffer)
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(normals), gl.STATIC_DRAW)

		const indexBuffer = gl.createBuffer()
		gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer)
		gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices), gl.STATIC_DRAW)

		requestAnimationFrame(drawScene)

		function drawScene(time: number) {
			if (!gl || !matrixLocation) return

			time *= 0.001

			if (canvas.width !== clientWidth || canvas.height !== clientHeight) {
				canvas.width = clientWidth
				canvas.height = clientHeight
			}

			gl.viewport(0, 0, canvas.width, canvas.height)
			gl.clearColor(0, 0, 0, 0)
			gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)
			gl.enable(gl.DEPTH_TEST)
			gl.enable(gl.CULL_FACE)
			gl.useProgram(program)

			gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)
			gl.enableVertexAttribArray(positionLocation)
			gl.vertexAttribPointer(positionLocation, 3, gl.FLOAT, false, 0, 0)

			gl.bindBuffer(gl.ARRAY_BUFFER, normalBuffer)
			gl.enableVertexAttribArray(normalLocation)
			gl.vertexAttribPointer(normalLocation, 3, gl.FLOAT, false, 0, 0)

			const lambdas = [
				{
					finalPos: [-0.1, 0.6],
					finalScale: 0.3,
					baseRot: [0.6, 0.1],
					sway: [0.1, 0.2]
				},
				{
					finalPos: [-0.6, -0.1],
					finalScale: 0.5,
					baseRot: [0.2, 0.8],
					sway: [0.2, 0.1]
				},
				{
					finalPos: [0.6, 0],
					finalScale: 0.4,
					baseRot: [0.2, -0.8],
					sway: [0.1, -0.2]
				}
			]

			for (const lambda of lambdas) {
				const t = easeOut(Math.min(time, 1))

				const currX = lambda.finalPos[0] * t
				const currY = lambda.finalPos[1] * t
				const currScale = lambda.finalScale * t
				const currRotX = lambda.baseRot[0] + Math.sin(time * lambda.sway[0]) * 0.3 * t
				const currRotY = lambda.baseRot[1] + Math.sin(time * lambda.sway[1]) * 0.3 * t

				let matrix = m4.identity()
				matrix = m4.translate(matrix, currX, currY, 0)
				matrix = m4.scale(matrix, currScale * 0.3)
				matrix = m4.rotateX(matrix, currRotX)
				matrix = m4.rotateY(matrix, currRotY)
				gl.uniformMatrix4fv(matrixLocation, false, matrix)

				gl.drawElements(gl.TRIANGLES, indices.length, gl.UNSIGNED_SHORT, 0)
			}

			requestAnimationFrame(drawScene)
		}
	})
</script>

<canvas
	class="absolute left-0 top-0 -z-50 h-screen w-full"
	bind:this={canvas}
	bind:clientWidth
	bind:clientHeight
></canvas>
