<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLElement> = $props()
</script>

<div class={cx("flex flex-col gap-2 p-2", className)} {...restProps}>
	{@render children?.()}
</div>
