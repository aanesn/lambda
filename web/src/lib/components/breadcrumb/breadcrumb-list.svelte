<script lang="ts">
	import type { HTMLOlAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLOlAttributes = $props()
</script>

<ol
	class={cx(
		"flex flex-wrap items-center gap-2 text-sm wrap-break-word text-neutral-400",
		className
	)}
	{...restProps}
>
	{@render children?.()}
</ol>
