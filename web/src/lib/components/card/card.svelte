<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLDivElement> = $props()
</script>

<div
	class={cx("flex flex-col gap-6 rounded-3xl border bg-neutral-950 py-6 shadow-sm", className)}
	{...restProps}
>
	{@render children?.()}
</div>
