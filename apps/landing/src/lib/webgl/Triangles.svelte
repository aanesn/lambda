<script lang="ts">
	import { createProgram, createShader } from "./webgl"
	import vertex from "./vertex.glsl?raw"
	import fragment from "./fragment.glsl?raw"
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
		const colorLocation = gl.getAttribLocation(program, "a_color")
		const matrixLocation = gl.getUniformLocation(program, "u_matrix")

		const positionBuffer = gl.createBuffer()
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)
		gl.bufferData(
			gl.ARRAY_BUFFER,
			// prettier-ignore
			new Float32Array([
                0, 1, 0,            1, 0, 0,
                -0.8, -0.5, 0.5,    1, 0, 0,
                0.8, -0.5, 0.5,     1, 0, 0,
                    
                0, 1, 0,            0, 1, 0,
                0.8, -0.5, 0.5,     0, 1, 0,
                0, -0.5, -0.8,      0, 1, 0,
                    
                0, 1, 0,            0, 0, 1,
                0, -0.5, -0.8,      0, 0, 1,
                -0.8, -0.5, 0.5,    0, 0, 1,
                    
                -0.8, -0.5, 0.5,    1, 1, 0,
                0, -0.5, -0.8,      1, 1, 0,
                0.8, -0.5, 0.5,     1, 1, 0,
            ]),
			gl.STATIC_DRAW
		)

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

			gl.enableVertexAttribArray(positionLocation)
			gl.vertexAttribPointer(positionLocation, 3, gl.FLOAT, false, 24, 0)

			gl.enableVertexAttribArray(colorLocation)
			gl.vertexAttribPointer(colorLocation, 3, gl.FLOAT, false, 24, 12)

			const triangles = [
				{
					finalPos: [-0.1, 0.5],
					finalScale: 0.2,
					baseRot: [0.4, 0.8],
					sway: 0.6
				},
				{
					finalPos: [-0.5, 0],
					finalScale: 0.3,
					baseRot: [0, 0.7],
					sway: 0.4
				},
				{
					finalPos: [0.45, 0],
					finalScale: 0.3,
					baseRot: [1.1, 0.3],
					sway: 0.3
				}
			]

			for (const triangle of triangles) {
				const t = easeOut(Math.min(time, 1))

				const currX = triangle.finalPos[0] * t
				const currY = triangle.finalPos[1] * t
				const currScale = triangle.finalScale * t
				const currRotX = triangle.baseRot[0] + Math.sin(time * triangle.sway) * 0.3 * t
				const currRotY = triangle.baseRot[1] + Math.sin(time * triangle.sway) * 0.3 * t

				let matrix = m4.identity()
				matrix = m4.translate(matrix, currX, currY, 0)
				matrix = m4.scale(matrix, currScale)
				matrix = m4.rotateX(matrix, currRotX)
				matrix = m4.rotateY(matrix, currRotY)

				gl.uniformMatrix4fv(matrixLocation, false, matrix)
				gl.drawArrays(gl.TRIANGLES, 0, 12)
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
