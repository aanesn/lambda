<script lang="ts">
	import { useStepper } from "./context.svelte"

	const stepper = useStepper()
</script>

<div
	class="pointer-events-none absolute left-1/2 flex h-full w-full max-w-xl -translate-x-1/2 items-center justify-center"
>
	<div class="absolute top-39 -right-1.25 -left-1.5 z-10">
		<div
			class="absolute left-0 before:absolute before:ml-[-5.5px] before:h-px before:w-3 before:bg-neutral-700 after:absolute after:ml-[-5.5px] after:h-px after:w-3 after:rotate-90 after:bg-neutral-700"
		></div>
		<div
			class="absolute right-0 before:absolute before:ml-[-5.5px] before:h-px before:w-3 before:bg-neutral-700 after:absolute after:ml-[-5.5px] after:h-px after:w-3 after:rotate-90 after:bg-neutral-700"
		></div>
	</div>
	<div
		class="absolute -right-1.25 -left-1.5 z-10 transition-[top] duration-500"
		style:top="{155 + stepper.height + 48}px"
	>
		<div
			class="absolute left-0 before:absolute before:ml-[-5.5px] before:h-px before:w-3 before:bg-neutral-700 after:absolute after:ml-[-5.5px] after:h-px after:w-3 after:rotate-90 after:bg-neutral-700"
		></div>
		<div
			class="absolute right-0 before:absolute before:ml-[-5.5px] before:h-px before:w-3 before:bg-neutral-700 after:absolute after:ml-[-5.5px] after:h-px after:w-3 after:rotate-90 after:bg-neutral-700"
		></div>
	</div>
	<div
		class="absolute -left-1.5 h-full w-[calc(100%+12px)] border-x border-y-0 border-dashed"
	></div>
	<div class="absolute top-45 left-1/2 w-screen -translate-x-1/2">
		<div
			class="-translate-y-6 border-x-0 border-y border-dashed transition-[height] duration-500"
			style:height="{stepper.height + 48}px"
		></div>
	</div>
</div>
