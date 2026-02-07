<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLDivElement> = $props()
</script>

<div
	class={cx("grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6", className)}
	{...restProps}
>
	{@render children?.()}
</div>
