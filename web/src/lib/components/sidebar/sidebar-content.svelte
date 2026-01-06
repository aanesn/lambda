<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLElement> = $props()
</script>

<div
	class={cx(
		"flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[state=collapsed]:overflow-hidden",
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>
