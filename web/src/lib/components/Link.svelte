<script lang="ts">
	import { cx } from "$lib/utils"
	import type { HTMLAnchorAttributes } from "svelte/elements"

	let { class: className, children, ...others }: HTMLAnchorAttributes = $props()
</script>

<a class={cx("text-sm transition-colors hover:text-white/80", className)} {...others}>
	{@render children?.()}
</a>
