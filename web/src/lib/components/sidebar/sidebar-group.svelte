<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLElement> = $props()
</script>

<div class={cx("relative flex w-full min-w-0 flex-col p-2", className)} {...restProps}>
	{@render children?.()}
</div>
