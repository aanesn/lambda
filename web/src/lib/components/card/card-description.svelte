<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let {
		class: className,
		children,
		...restProps
	}: HTMLAttributes<HTMLParagraphElement> = $props()
</script>

<p class={cx("text-sm text-neutral-500", className)} {...restProps}>
	{@render children?.()}
</p>
