<script lang="ts">
	import { useStepper } from "./context.svelte"

	let { items }: { items: string[] } = $props()
	const stepper = useStepper()
</script>

<div class="absolute left-full hidden w-max translate-x-5 flex-col lg:flex">
	{#each items as item, i}
		<button
			type="button"
			data-state={stepper.isActive(i) ? "active" : stepper.curr > i ? "prev" : undefined}
			class="group flex w-full items-center gap-2 rounded-xl px-3 py-2 text-neutral-400 transition-colors duration-300
				data-[state=active]:text-white
				data-[state=prev]:cursor-pointer data-[state=prev]:hover:bg-neutral-900"
			onclick={() => stepper.curr > i && stepper.goto(i)}
			disabled={stepper.curr <= i}
		>
			<div
				class="h-1.5 w-1.5 rounded-full border border-neutral-400 transition-colors duration-300
					group-data-[state=active]:border-0 group-data-[state=active]:bg-white
					group-data-[state=prev]:bg-neutral-700"
			></div>
			<span class="flex text-sm whitespace-nowrap">{item}</span>
		</button>
	{/each}
</div>
