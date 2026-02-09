<script lang="ts">
	import type { HTMLAnchorAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let {
		class: className,
		href,
		children,
		...restProps
	}: HTMLAnchorAttributes & {
		href: string
	} = $props()
</script>

<a {href} class={cx("transition-colors hover:text-white", className)} {...restProps}>
	{@render children?.()}
</a>
