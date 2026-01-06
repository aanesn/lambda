<script lang="ts">
	import { cx } from "$lib/utils"
	import type { HTMLAttributes } from "svelte/elements"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLLIElement> = $props()
</script>

<li class={cx("relative", className)} {...restProps}>
	{@render children?.()}
</li>
