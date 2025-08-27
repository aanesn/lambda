<script lang="ts">
	import { createProgram, createShader } from "./helpers"
	import vertex from "./vertex.glsl?raw"
	import fragment from "./fragment.glsl?raw"

	let canvas: HTMLCanvasElement
	let gl: WebGLRenderingContext | null
	let program: WebGLProgram
	let positionAttributeLocation: number

	$effect(() => {
		gl = canvas.getContext("webgl")
		if (!gl) {
			console.error("webgl is unsupported in your browser")
			return
		}

		const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertex)
		const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragment)
		program = createProgram(gl, vertexShader, fragmentShader)
		positionAttributeLocation = gl.getAttribLocation(program, "a_position")

		// set up state
		const positionBuffer = gl.createBuffer()
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)
		const positions = [0, 0, 0, 0.5, 0.7, 0]
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW)
	})

	let clientWidth = $state(0)
	let clientHeight = $state(0)

	$effect(() => {
		if (!gl || (canvas.width === clientWidth && canvas.height === clientHeight)) {
			return
		}

		// resize
		canvas.width = clientWidth
		canvas.height = clientHeight
		gl.viewport(0, 0, canvas.width, canvas.height)

		gl.clearColor(0, 0, 0, 0)
		gl.clear(gl.COLOR_BUFFER_BIT)

		// render
		gl.useProgram(program)
		gl.enableVertexAttribArray(positionAttributeLocation)
		gl.vertexAttribPointer(positionAttributeLocation, 2, gl.FLOAT, false, 0, 0)
		gl.drawArrays(gl.TRIANGLES, 0, 3)
	})
</script>

<canvas
	class="absolute left-0 top-0 -z-50 h-screen w-full"
	bind:this={canvas}
	bind:clientWidth
	bind:clientHeight
></canvas>
