<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import type { HTMLAnchorAttributes } from "svelte/elements"

	type Props = HTMLAnchorAttributes & {
		href: string
	}

	let { class: className, children, href, ...restProps }: Props = $props()
</script>

<a class={cx("text-sm hover:text-white/85", className)} {href} {...restProps}>
	{@render children?.()}
</a>
