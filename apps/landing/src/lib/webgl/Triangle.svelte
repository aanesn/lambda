<script lang="ts">
	import { createProgram, createShader, init, render } from "./webgl"
	import vertex from "./vertex.glsl?raw"
	import fragment from "./fragment.glsl?raw"

	let canvas: HTMLCanvasElement
	let gl: WebGLRenderingContext | null
	let program: WebGLProgram
	let positionLocation: number

	$effect(() => {
		gl = canvas.getContext("webgl")
		if (!gl) {
			console.error("webgl is unsupported in your browser")
			return
		}

		const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertex)
		const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragment)
		program = createProgram(gl, vertexShader, fragmentShader)

		positionLocation = gl.getAttribLocation(program, "a_position")

		init(gl)
	})

	let clientWidth = $state(0)
	let clientHeight = $state(0)

	$effect(() => {
		if (!gl) return
		console.log("ran")

		canvas.width = clientWidth
		canvas.height = clientHeight
		gl.viewport(0, 0, canvas.width, canvas.height)

		gl.clearColor(0, 0, 0, 0)
		gl.clear(gl.COLOR_BUFFER_BIT)

		render(gl, program, positionLocation)
	})
</script>

<canvas
	class="absolute left-0 top-0 -z-50 h-screen w-full"
	bind:this={canvas}
	bind:clientWidth
	bind:clientHeight
></canvas>
