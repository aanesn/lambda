<script lang="ts">
	import { cx } from "$lib/utils"
	import type { HTMLAttributes } from "svelte/elements"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLDivElement> = $props()
</script>

<div class={cx("flex items-center px-6", className)} {...restProps}>
	{@render children?.()}
</div>
