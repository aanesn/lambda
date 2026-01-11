<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"
	import { setStepper } from "./context.svelte"

	let {
		class: className,
		curr = $bindable(0),
		heights = [],
		children,
		...restProps
	}: HTMLAttributes<HTMLDivElement> & {
		curr?: number
		heights?: number[]
	} = $props()

	setStepper({
		curr: () => curr,
		setCurr: (v) => (curr = v),
		heights: () => heights
	})
</script>

<div class={cx("relative", className)} {...restProps}>
	{@render children?.()}
</div>
