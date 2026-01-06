<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"
	import { useSidebar } from "./context.svelte"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLButtonElement> = $props()

	const sidebar = useSidebar()
</script>

<button
	onclick={sidebar.toggle}
	class={cx(
		"absolute inset-y-0 -end-4 z-20 hidden w-4 -translate-x-1/2 cursor-w-resize transition-all ease-linear after:absolute after:inset-y-0 after:start-[calc(1/2*100%-1px)] after:w-0.5 hover:after:bg-white/15 in-data-[state=collapsed]:cursor-e-resize sm:flex",
		className
	)}
	{...restProps}
>
	{@render children?.()}
</button>
