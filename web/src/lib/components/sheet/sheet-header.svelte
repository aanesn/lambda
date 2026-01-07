<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLDivElement> = $props()
</script>

<div class={cx("flex flex-col gap-1.5 p-4", className)} {...restProps}>
	{@render children?.()}
</div>
