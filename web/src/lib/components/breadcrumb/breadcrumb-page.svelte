<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLSpanElement> = $props()
</script>

<span class={cx("text-white", className)} {...restProps}>
	{@render children?.()}
</span>
