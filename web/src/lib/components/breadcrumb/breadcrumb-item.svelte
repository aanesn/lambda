<script lang="ts">
	import type { HTMLLiAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLLiAttributes = $props()
</script>

<li class={cx("inline-flex items-center gap-1.5", className)} {...restProps}>
	{@render children?.()}
</li>
