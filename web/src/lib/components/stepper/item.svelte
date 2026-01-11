<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"
	import { useStepper } from "./context.svelte"

	let {
		class: className,
		i,
		onclick,
		children,
		...restProps
	}: HTMLAttributes<HTMLDivElement> & {
		i: number
	} = $props()

	const stepper = useStepper()
	const isActive = $derived(stepper.isActive(i))
	const isPrev = $derived(stepper.isPrev(i))
</script>

<div
	data-state={isActive ? "active" : isPrev ? "prev" : ""}
	class={cx(
		"absolute top-0 w-full translate-y-[210%] scale-100 px-6 opacity-0 transition-all duration-500 ease-in-out will-change-transform backface-hidden",
		"data-[state=active]:z-30 data-[state=active]:translate-y-0 data-[state=active]:opacity-100",
		"data-[state=prev]:z-20 data-[state=prev]:translate-y-[-110%] data-[state=prev]:scale-[0.85] data-[state=prev]:cursor-pointer data-[state=prev]:opacity-50 data-[state=prev]:hover:opacity-100",
		"data-[state=prev]:[&_a]:pointer-events-none data-[state=prev]:[&_button]:pointer-events-none",
		className
	)}
	onclick={(e) => {
		stepper.goto(i)
		onclick?.(e)
	}}
	role="button"
	{...restProps}
>
	{@render children?.()}
</div>
