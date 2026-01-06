<script lang="ts">
	import { cx } from "$lib/utils"
	import type { HTMLAttributes } from "svelte/elements"

	let { class: className, children, ...restProps }: HTMLAttributes<HTMLUListElement> = $props()
</script>

<ul class={cx("flex w-full min-w-0 flex-col gap-1", className)} {...restProps}>
	{@render children?.()}
</ul>
