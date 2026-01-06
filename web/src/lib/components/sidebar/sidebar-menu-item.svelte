<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLLIElement> = $props()
</script>

<li class={cx("relative", className)} {...restProps}>
	{@render children?.()}
</li>
