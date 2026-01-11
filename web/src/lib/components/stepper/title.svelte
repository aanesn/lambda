<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLHeadingElement> = $props()
</script>

<div class="pointer-events-none absolute left-1/2 w-full max-w-xl -translate-x-1/2">
	<h1
		class={cx(
			"absolute top-46.25 right-[calc(100%+3rem)] hidden text-right whitespace-nowrap lg:block",
			className
		)}
		{...restProps}
	>
		{@render children?.()}
	</h1>
</div>
